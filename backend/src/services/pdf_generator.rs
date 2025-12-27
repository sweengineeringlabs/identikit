//! PDF Generator
//!
//! Generate PDF documents from composites.

use std::fs;
use std::io::BufWriter;
use std::path::Path;

use printpdf::*;

use crate::models::Composite;

/// PDF generation configuration.
#[derive(Debug, Clone)]
pub struct PdfConfig {
    /// Page width in mm.
    pub page_width_mm: f32,
    /// Page height in mm.
    pub page_height_mm: f32,
    /// Margin in mm.
    pub margin_mm: f32,
    /// Whether to include metadata page.
    pub include_metadata: bool,
    /// Title for the document.
    pub title: Option<String>,
    /// Author name.
    pub author: Option<String>,
}

impl Default for PdfConfig {
    fn default() -> Self {
        Self {
            page_width_mm: 210.0,  // A4 width
            page_height_mm: 297.0, // A4 height
            margin_mm: 20.0,
            include_metadata: true,
            title: None,
            author: None,
        }
    }
}

impl PdfConfig {
    /// Create US Letter size config.
    pub fn us_letter() -> Self {
        Self {
            page_width_mm: 215.9,
            page_height_mm: 279.4,
            ..Default::default()
        }
    }

    /// Create A4 config.
    pub fn a4() -> Self {
        Self::default()
    }
}

/// Generate a PDF from a composite.
pub fn generate_pdf(
    composite: &Composite,
    include_metadata: bool,
) -> Result<Vec<u8>, String> {
    let config = PdfConfig {
        include_metadata,
        title: Some(composite.name.clone()),
        author: composite.metadata.author.clone(),
        ..Default::default()
    };

    generate_pdf_with_config(composite, &config)
}

/// Generate a PDF with custom configuration.
pub fn generate_pdf_with_config(
    composite: &Composite,
    config: &PdfConfig,
) -> Result<Vec<u8>, String> {
    // Create PDF document
    let doc_title = config.title.clone().unwrap_or_else(|| composite.name.clone());
    let (doc, page1, layer1) = PdfDocument::new(
        &doc_title,
        Mm(config.page_width_mm),
        Mm(config.page_height_mm),
        "Composite",
    );

    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Add placeholder text (actual image embedding requires more complex setup)
    let font = doc
        .add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|e| format!("Failed to add font: {}", e))?;

    let margin = config.margin_mm;
    let y_pos = config.page_height_mm - margin;

    current_layer.use_text(
        &format!("Composite: {}", composite.name),
        14.0,
        Mm(margin),
        Mm(y_pos),
        &font,
    );

    current_layer.use_text(
        &format!("Layers: {}", composite.layers.len()),
        12.0,
        Mm(margin),
        Mm(y_pos - 10.0),
        &font,
    );

    current_layer.use_text(
        &format!("Canvas: {}x{}", composite.canvas.width, composite.canvas.height),
        12.0,
        Mm(margin),
        Mm(y_pos - 20.0),
        &font,
    );

    // Note: Full image embedding would use printpdf's image API
    // For now, this creates a placeholder PDF
    current_layer.use_text(
        "[Composite Image - use export_png for full render]",
        10.0,
        Mm(margin),
        Mm(y_pos - 40.0),
        &font,
    );

    // Add metadata page if requested
    if config.include_metadata {
        add_metadata_page(&doc, composite, config)?;
    }

    // Serialize to bytes
    let mut buffer = Vec::new();
    {
        let mut writer = BufWriter::new(&mut buffer);
        doc.save(&mut writer)
            .map_err(|e| format!("Failed to save PDF: {}", e))?;
    }

    tracing::info!(
        "Generated PDF: {} bytes (metadata: {})",
        buffer.len(),
        config.include_metadata
    );

    Ok(buffer)
}

/// Add metadata page to PDF.
fn add_metadata_page(
    doc: &PdfDocumentReference,
    composite: &Composite,
    config: &PdfConfig,
) -> Result<(), String> {
    let (page2, layer2) = doc.add_page(
        Mm(config.page_width_mm),
        Mm(config.page_height_mm),
        "Metadata",
    );

    let current_layer = doc.get_page(page2).get_layer(layer2);

    let font = doc
        .add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|e| format!("Failed to add font: {}", e))?;

    let font_bold = doc
        .add_builtin_font(BuiltinFont::HelveticaBold)
        .map_err(|e| format!("Failed to add bold font: {}", e))?;

    let margin = config.margin_mm;
    let mut y_pos = config.page_height_mm - margin;
    let line_height = 8.0;
    let section_spacing = 12.0;

    // Title
    current_layer.use_text(
        &composite.name,
        18.0,
        Mm(margin),
        Mm(y_pos),
        &font_bold,
    );
    y_pos -= section_spacing * 2.0;

    // Document Info section
    current_layer.use_text(
        "Document Information",
        14.0,
        Mm(margin),
        Mm(y_pos),
        &font_bold,
    );
    y_pos -= section_spacing;

    let info_items = [
        ("ID:", composite.id.to_string()),
        ("Created:", composite.created_at.format("%Y-%m-%d %H:%M").to_string()),
        ("Modified:", composite.modified_at.format("%Y-%m-%d %H:%M").to_string()),
        ("Canvas:", format!("{}x{} px", composite.canvas.width, composite.canvas.height)),
        ("Layers:", composite.layers.len().to_string()),
    ];

    for (label, value) in info_items {
        current_layer.use_text(label, 10.0, Mm(margin), Mm(y_pos), &font_bold);
        current_layer.use_text(&value, 10.0, Mm(margin + 25.0), Mm(y_pos), &font);
        y_pos -= line_height;
    }

    y_pos -= section_spacing;

    // Metadata section
    if composite.metadata.author.is_some()
        || composite.metadata.case_number.is_some()
        || composite.metadata.description.is_some()
    {
        current_layer.use_text(
            "Case Information",
            14.0,
            Mm(margin),
            Mm(y_pos),
            &font_bold,
        );
        y_pos -= section_spacing;

        if let Some(ref author) = composite.metadata.author {
            current_layer.use_text("Author:", 10.0, Mm(margin), Mm(y_pos), &font_bold);
            current_layer.use_text(author, 10.0, Mm(margin + 25.0), Mm(y_pos), &font);
            y_pos -= line_height;
        }

        if let Some(ref case_number) = composite.metadata.case_number {
            current_layer.use_text("Case #:", 10.0, Mm(margin), Mm(y_pos), &font_bold);
            current_layer.use_text(case_number, 10.0, Mm(margin + 25.0), Mm(y_pos), &font);
            y_pos -= line_height;
        }

        if let Some(ref description) = composite.metadata.description {
            current_layer.use_text("Description:", 10.0, Mm(margin), Mm(y_pos), &font_bold);
            y_pos -= line_height;
            current_layer.use_text(description, 10.0, Mm(margin), Mm(y_pos), &font);
            y_pos -= line_height;
        }

        y_pos -= section_spacing;
    }

    // Layers section
    current_layer.use_text("Layers", 14.0, Mm(margin), Mm(y_pos), &font_bold);
    y_pos -= section_spacing;

    for (i, layer) in composite.layers.iter().enumerate() {
        let visibility = if layer.visible { "visible" } else { "hidden" };
        let layer_info = format!(
            "{}. {} ({}) - opacity: {:.0}%",
            i + 1,
            layer.name,
            visibility,
            layer.opacity * 100.0
        );
        current_layer.use_text(&layer_info, 10.0, Mm(margin), Mm(y_pos), &font);
        y_pos -= line_height;

        if y_pos < margin {
            break;
        }
    }

    // Footer
    let footer_y = margin / 2.0;
    let generated_text = format!(
        "Generated by Identikit - {}",
        chrono::Utc::now().format("%Y-%m-%d %H:%M UTC")
    );
    current_layer.use_text(&generated_text, 8.0, Mm(margin), Mm(footer_y), &font);

    Ok(())
}

/// Save PDF directly to file.
pub fn save_pdf_to_file(
    composite: &Composite,
    path: &Path,
    include_metadata: bool,
) -> Result<(), String> {
    let pdf_data = generate_pdf(composite, include_metadata)?;
    fs::write(path, &pdf_data)
        .map_err(|e| format!("Failed to write PDF: {}", e))?;

    tracing::info!("Saved PDF to {:?}", path);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdf_config_defaults() {
        let config = PdfConfig::default();
        assert_eq!(config.page_width_mm, 210.0);
        assert_eq!(config.page_height_mm, 297.0);
        assert_eq!(config.margin_mm, 20.0);
        assert!(config.include_metadata);
    }

    #[test]
    fn test_pdf_config_us_letter() {
        let config = PdfConfig::us_letter();
        assert!((config.page_width_mm - 215.9).abs() < 0.1);
        assert!((config.page_height_mm - 279.4).abs() < 0.1);
    }
}
