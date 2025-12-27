//! Image Processor
//!
//! Image processing utilities for compositing and export.

use std::fs;
use std::path::Path;

use tiny_skia::{Pixmap, Transform as SkiaTransform};
use usvg::{Options, Tree};

use crate::models::{Composite, Layer};
use crate::services::FeatureLibraryService;

/// Render configuration.
#[derive(Debug, Clone)]
pub struct RenderConfig {
    /// Output width in pixels.
    pub width: u32,
    /// Output height in pixels.
    pub height: u32,
    /// Background color in hex format.
    pub background_color: String,
    /// Scale factor for high-DPI rendering.
    pub scale_factor: f32,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 1000,
            background_color: "#FFFFFF".to_string(),
            scale_factor: 1.0,
        }
    }
}

impl RenderConfig {
    /// Create config from composite.
    pub fn from_composite(composite: &Composite) -> Self {
        Self {
            width: composite.canvas.width,
            height: composite.canvas.height,
            background_color: composite.canvas.background_color.clone(),
            scale_factor: 1.0,
        }
    }

    /// Create config with custom dimensions.
    pub fn with_dimensions(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            ..Default::default()
        }
    }
}

/// Parse a hex color string to RGBA components.
fn parse_hex_color(hex: &str) -> (u8, u8, u8, u8) {
    let hex = hex.trim_start_matches('#');

    let (r, g, b, a) = match hex.len() {
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);
            (r, g, b, 255)
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);
            let a = u8::from_str_radix(&hex[6..8], 16).unwrap_or(255);
            (r, g, b, a)
        }
        3 => {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).unwrap_or(255);
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).unwrap_or(255);
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).unwrap_or(255);
            (r, g, b, 255)
        }
        _ => (255, 255, 255, 255),
    };

    (r, g, b, a)
}

/// Render a composite to a PNG buffer.
pub fn render_composite_to_png(
    composite: &Composite,
    width: u32,
    height: u32,
) -> Result<Vec<u8>, String> {
    let config = RenderConfig {
        width,
        height,
        background_color: composite.canvas.background_color.clone(),
        scale_factor: 1.0,
    };

    render_composite(composite, &config, None)
}

/// Render a composite using feature library.
pub fn render_composite_with_library(
    composite: &Composite,
    config: &RenderConfig,
    library: &FeatureLibraryService,
) -> Result<Vec<u8>, String> {
    render_composite(composite, config, Some(library))
}

/// Render a composite to PNG bytes.
pub fn render_composite(
    composite: &Composite,
    config: &RenderConfig,
    library: Option<&FeatureLibraryService>,
) -> Result<Vec<u8>, String> {
    let scaled_width = (config.width as f32 * config.scale_factor) as u32;
    let scaled_height = (config.height as f32 * config.scale_factor) as u32;

    // Create pixmap with background color
    let mut pixmap = Pixmap::new(scaled_width, scaled_height)
        .ok_or("Failed to create pixmap")?;

    // Fill background
    let (r, g, b, a) = parse_hex_color(&config.background_color);
    let bg_color = tiny_skia::Color::from_rgba8(r, g, b, a);
    pixmap.fill(bg_color);

    // Sort layers by z-index
    let mut layers: Vec<_> = composite.layers.iter().collect();
    layers.sort_by_key(|l| l.z_index);

    // Render each visible layer
    for layer in layers {
        if !layer.visible {
            continue;
        }

        // Get SVG content
        let svg_content = get_layer_svg(layer, library)?;

        // Apply color overrides if any
        let svg_content = apply_color_overrides(&svg_content, &layer.color_overrides);

        // Render layer
        render_layer_to_pixmap(
            &mut pixmap,
            &svg_content,
            layer,
            config.scale_factor,
        )?;
    }

    // Encode as PNG
    let png_data = pixmap
        .encode_png()
        .map_err(|e| format!("Failed to encode PNG: {}", e))?;

    tracing::info!(
        "Rendered composite to PNG: {}x{} ({} layers)",
        config.width,
        config.height,
        composite.layers.len()
    );

    Ok(png_data)
}

/// Get SVG content for a layer.
fn get_layer_svg(
    layer: &Layer,
    library: Option<&FeatureLibraryService>,
) -> Result<String, String> {
    // Try to get from library first
    if let Some(lib) = library {
        if let Ok(svg) = lib.read_svg(&layer.feature_id) {
            return Ok(svg);
        }
    }

    // Fall back to reading from feature_id as path
    // The feature_id might contain a path like "/features/hair/style-001.svg"
    let path = if layer.feature_id.starts_with('/') || layer.feature_id.contains(":\\") {
        layer.feature_id.clone()
    } else {
        format!("features/{}.svg", layer.feature_id.replace('-', "/"))
    };

    fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read SVG for layer {}: {}", layer.feature_id, e))
}

/// Apply color overrides to SVG content.
fn apply_color_overrides(
    svg_content: &str,
    overrides: &std::collections::HashMap<String, String>,
) -> String {
    if overrides.is_empty() {
        return svg_content.to_string();
    }

    let mut result = svg_content.to_string();

    for (zone_id, new_color) in overrides {
        // Find elements with data-color-zone="zone_id" and replace their fill color
        // This is a simplified implementation - a proper one would use XML parsing
        let pattern = format!(r#"data-color-zone="{}""#, zone_id);
        if result.contains(&pattern) {
            // For each matching element, try to replace the fill attribute
            let parts: Vec<&str> = result.split(&pattern).collect();
            if parts.len() > 1 {
                let mut new_result = String::new();
                for (i, part) in parts.iter().enumerate() {
                    if i > 0 {
                        // Re-add the pattern
                        new_result.push_str(&pattern);
                    }

                    // If this is not the last part, try to find and replace fill
                    if i < parts.len() - 1 {
                        // Look backwards for fill attribute in the same element
                        if let Some(fill_pos) = part.rfind("fill=\"") {
                            let before_fill = &part[..fill_pos];
                            // Check if we're still in the same element (no '>' between)
                            if !before_fill.chars().rev().any(|c| c == '>') {
                                let after_fill = &part[fill_pos + 6..];
                                if let Some(end_quote) = after_fill.find('"') {
                                    new_result.push_str(&part[..fill_pos]);
                                    new_result.push_str(&format!(r#"fill="{}""#, new_color));
                                    new_result.push_str(&after_fill[end_quote + 1..]);
                                    continue;
                                }
                            }
                        }
                    }

                    new_result.push_str(part);
                }
                result = new_result;
            }
        }
    }

    result
}

/// Render a single layer onto a pixmap.
fn render_layer_to_pixmap(
    pixmap: &mut Pixmap,
    svg_content: &str,
    layer: &Layer,
    scale_factor: f32,
) -> Result<(), String> {
    let options = Options::default();

    let tree = Tree::from_str(svg_content, &options)
        .map_err(|e| format!("Failed to parse SVG: {}", e))?;

    let svg_size = tree.size();
    let svg_width = svg_size.width() as f32;
    let svg_height = svg_size.height() as f32;

    // Build transformation matrix
    let transform = &layer.transform;

    // Start with identity transform
    let mut ts = SkiaTransform::identity();

    // Apply scale factor first
    ts = ts.post_scale(scale_factor, scale_factor);

    // Translate to position
    ts = ts.post_translate(transform.x as f32, transform.y as f32);

    // Apply flip transformations
    if transform.flip_x {
        ts = ts.post_scale(-1.0, 1.0);
    }
    if transform.flip_y {
        ts = ts.post_scale(1.0, -1.0);
    }

    // Apply rotation (around the center of the SVG)
    if transform.rotation.abs() > 0.001 {
        let angle_rad = transform.rotation.to_radians() as f32;
        ts = ts.post_rotate(angle_rad.to_degrees());
    }

    // Apply scale
    ts = ts.post_scale(
        transform.scale_x as f32,
        transform.scale_y as f32,
    );

    // Offset to center the SVG at the transform position
    ts = ts.post_translate(
        -svg_width / 2.0,
        -svg_height / 2.0,
    );

    // Create a temporary pixmap for this layer if we need opacity
    if (layer.opacity - 1.0).abs() > 0.001 {
        // Render to temporary pixmap
        let mut temp_pixmap = Pixmap::new(pixmap.width(), pixmap.height())
            .ok_or("Failed to create temp pixmap")?;

        resvg::render(&tree, ts, &mut temp_pixmap.as_mut());

        // Composite with opacity
        let opacity = (layer.opacity * 255.0) as u8;
        let paint = tiny_skia::PixmapPaint {
            opacity: layer.opacity as f32,
            blend_mode: tiny_skia::BlendMode::SourceOver,
            quality: tiny_skia::FilterQuality::Bilinear,
        };

        pixmap.draw_pixmap(
            0,
            0,
            temp_pixmap.as_ref(),
            &paint,
            SkiaTransform::identity(),
            None,
        );
    } else {
        // Render directly
        resvg::render(&tree, ts, &mut pixmap.as_mut());
    }

    Ok(())
}

/// Generate a thumbnail for a feature SVG.
pub fn generate_thumbnail(svg_path: &str, size: u32) -> Result<Vec<u8>, String> {
    let svg_content = fs::read_to_string(svg_path)
        .map_err(|e| format!("Failed to read SVG: {}", e))?;

    generate_thumbnail_from_svg(&svg_content, size)
}

/// Generate a thumbnail from SVG content.
pub fn generate_thumbnail_from_svg(svg_content: &str, size: u32) -> Result<Vec<u8>, String> {
    let options = Options::default();

    let tree = Tree::from_str(svg_content, &options)
        .map_err(|e| format!("Failed to parse SVG: {}", e))?;

    // Calculate scaling to fit in square while maintaining aspect ratio
    let svg_size = tree.size();
    let svg_width = svg_size.width() as f32;
    let svg_height = svg_size.height() as f32;

    let scale = if svg_width > svg_height {
        size as f32 / svg_width
    } else {
        size as f32 / svg_height
    };

    let out_width = (svg_width * scale).ceil() as u32;
    let out_height = (svg_height * scale).ceil() as u32;

    let mut pixmap = Pixmap::new(out_width, out_height)
        .ok_or("Failed to create pixmap")?;

    // Fill with transparent background
    pixmap.fill(tiny_skia::Color::TRANSPARENT);

    // Render scaled
    let transform = SkiaTransform::from_scale(scale, scale);
    resvg::render(&tree, transform, &mut pixmap.as_mut());

    // Encode as PNG
    let png_data = pixmap
        .encode_png()
        .map_err(|e| format!("Failed to encode PNG: {}", e))?;

    tracing::info!("Generated thumbnail: {}x{}", out_width, out_height);

    Ok(png_data)
}

/// Render composite to image bytes and return as base64.
pub fn render_composite_to_base64(
    composite: &Composite,
    width: u32,
    height: u32,
) -> Result<String, String> {
    let png_data = render_composite_to_png(composite, width, height)?;
    Ok(base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        &png_data,
    ))
}

/// Save rendered composite directly to file.
pub fn save_composite_to_file(
    composite: &Composite,
    path: &Path,
    width: u32,
    height: u32,
) -> Result<(), String> {
    let png_data = render_composite_to_png(composite, width, height)?;
    fs::write(path, &png_data)
        .map_err(|e| format!("Failed to write file: {}", e))?;

    tracing::info!("Saved composite to {:?}", path);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_svg() -> String {
        String::from(r##"<?xml version="1.0" encoding="UTF-8"?><svg width="100" height="100" xmlns="http://www.w3.org/2000/svg"><rect width="100" height="100" fill="#FFFFFF"/><circle cx="50" cy="50" r="40" fill="#FF0000"/></svg>"##)
    }

    #[test]
    fn test_parse_hex_color() {
        assert_eq!(parse_hex_color("#FFFFFF"), (255, 255, 255, 255));
        assert_eq!(parse_hex_color("#000000"), (0, 0, 0, 255));
        assert_eq!(parse_hex_color("#FF0000"), (255, 0, 0, 255));
        assert_eq!(parse_hex_color("#00FF00FF"), (0, 255, 0, 255));
        assert_eq!(parse_hex_color("FFF"), (255, 255, 255, 255));
    }

    #[test]
    fn test_generate_thumbnail() {
        let svg = create_test_svg();
        let result = generate_thumbnail_from_svg(&svg, 64);
        assert!(result.is_ok());

        let png_data = result.unwrap();
        // PNG magic bytes
        assert_eq!(&png_data[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    }

    #[test]
    fn test_render_config_from_composite() {
        let composite = Composite::with_canvas("Test".to_string(), 1024, 768);
        let config = RenderConfig::from_composite(&composite);

        assert_eq!(config.width, 1024);
        assert_eq!(config.height, 768);
        assert_eq!(config.background_color, "#FFFFFF");
    }
}
