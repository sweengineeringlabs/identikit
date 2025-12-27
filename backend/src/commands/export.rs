//! Export Commands
//!
//! Export composites to PNG and PDF formats.

use std::fs;
use std::path::Path;

use serde::Serialize;
use tauri::State;

use super::composite::CompositeState;
use crate::services::image_processor::{render_composite_to_png, RenderConfig};
use crate::services::pdf_generator::{generate_pdf, PdfConfig};

/// Result of an export operation.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportResult {
    pub success: bool,
    pub path: Option<String>,
    pub size_bytes: Option<usize>,
    pub error: Option<String>,
}

impl ExportResult {
    fn success(path: String, size: usize) -> Self {
        Self {
            success: true,
            path: Some(path),
            size_bytes: Some(size),
            error: None,
        }
    }

    fn error(msg: String) -> Self {
        Self {
            success: false,
            path: None,
            size_bytes: None,
            error: Some(msg),
        }
    }
}

/// Export the current composite to PNG.
#[tauri::command]
pub async fn export_png(
    path: String,
    width: u32,
    height: u32,
    state: State<'_, CompositeState>,
) -> Result<ExportResult, String> {
    let current = state.current.lock().map_err(|e| e.to_string())?;
    let composite = current.as_ref().ok_or("No composite loaded")?;

    tracing::info!(
        "Exporting PNG: {} ({}x{}) - {} layers",
        path,
        width,
        height,
        composite.layers.len()
    );

    // Render composite to PNG
    let png_data = match render_composite_to_png(composite, width, height) {
        Ok(data) => data,
        Err(e) => {
            tracing::error!("PNG render failed: {}", e);
            return Ok(ExportResult::error(format!("Render failed: {}", e)));
        }
    };

    // Write to file
    let file_path = Path::new(&path);
    if let Some(parent) = file_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }
    }

    let size = png_data.len();
    fs::write(file_path, &png_data)
        .map_err(|e| format!("Failed to write PNG: {}", e))?;

    tracing::info!("PNG exported: {} ({} bytes)", path, size);

    Ok(ExportResult::success(path, size))
}

/// Export the current composite to PNG with custom render config.
#[tauri::command]
pub async fn export_png_advanced(
    path: String,
    width: u32,
    height: u32,
    scale_factor: f32,
    background_color: Option<String>,
    state: State<'_, CompositeState>,
) -> Result<ExportResult, String> {
    let current = state.current.lock().map_err(|e| e.to_string())?;
    let composite = current.as_ref().ok_or("No composite loaded")?;

    let config = RenderConfig {
        width,
        height,
        background_color: background_color.unwrap_or_else(|| composite.canvas.background_color.clone()),
        scale_factor,
    };

    tracing::info!(
        "Exporting PNG (advanced): {} ({}x{} @{}x)",
        path,
        width,
        height,
        scale_factor
    );

    // Render composite to PNG
    let png_data = match crate::services::image_processor::render_composite(composite, &config, None) {
        Ok(data) => data,
        Err(e) => {
            tracing::error!("PNG render failed: {}", e);
            return Ok(ExportResult::error(format!("Render failed: {}", e)));
        }
    };

    // Write to file
    let file_path = Path::new(&path);
    if let Some(parent) = file_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }
    }

    let size = png_data.len();
    fs::write(file_path, &png_data)
        .map_err(|e| format!("Failed to write PNG: {}", e))?;

    tracing::info!("PNG exported: {} ({} bytes)", path, size);

    Ok(ExportResult::success(path, size))
}

/// Export the current composite to PDF.
#[tauri::command]
pub async fn export_pdf(
    path: String,
    include_metadata: bool,
    state: State<'_, CompositeState>,
) -> Result<ExportResult, String> {
    let current = state.current.lock().map_err(|e| e.to_string())?;
    let composite = current.as_ref().ok_or("No composite loaded")?;

    tracing::info!(
        "Exporting PDF: {} (metadata: {}) - {} layers",
        path,
        include_metadata,
        composite.layers.len()
    );

    // Generate PDF
    let pdf_data = match generate_pdf(composite, include_metadata) {
        Ok(data) => data,
        Err(e) => {
            tracing::error!("PDF generation failed: {}", e);
            return Ok(ExportResult::error(format!("PDF generation failed: {}", e)));
        }
    };

    // Write to file
    let file_path = Path::new(&path);
    if let Some(parent) = file_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }
    }

    let size = pdf_data.len();
    fs::write(file_path, &pdf_data)
        .map_err(|e| format!("Failed to write PDF: {}", e))?;

    tracing::info!("PDF exported: {} ({} bytes)", path, size);

    Ok(ExportResult::success(path, size))
}

/// Export the current composite to PDF with custom config.
#[tauri::command]
pub async fn export_pdf_advanced(
    path: String,
    page_size: String,
    include_metadata: bool,
    title: Option<String>,
    author: Option<String>,
    state: State<'_, CompositeState>,
) -> Result<ExportResult, String> {
    let current = state.current.lock().map_err(|e| e.to_string())?;
    let composite = current.as_ref().ok_or("No composite loaded")?;

    let mut config = match page_size.to_lowercase().as_str() {
        "letter" | "us-letter" => PdfConfig::us_letter(),
        "a4" | _ => PdfConfig::a4(),
    };

    config.include_metadata = include_metadata;
    config.title = title.or_else(|| Some(composite.name.clone()));
    config.author = author.or_else(|| composite.metadata.author.clone());

    tracing::info!(
        "Exporting PDF (advanced): {} (page: {}, metadata: {})",
        path,
        page_size,
        include_metadata
    );

    // Generate PDF
    let pdf_data = match crate::services::pdf_generator::generate_pdf_with_config(composite, &config) {
        Ok(data) => data,
        Err(e) => {
            tracing::error!("PDF generation failed: {}", e);
            return Ok(ExportResult::error(format!("PDF generation failed: {}", e)));
        }
    };

    // Write to file
    let file_path = Path::new(&path);
    if let Some(parent) = file_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }
    }

    let size = pdf_data.len();
    fs::write(file_path, &pdf_data)
        .map_err(|e| format!("Failed to write PDF: {}", e))?;

    tracing::info!("PDF exported: {} ({} bytes)", path, size);

    Ok(ExportResult::success(path, size))
}

/// Get composite as base64-encoded PNG for preview.
#[tauri::command]
pub async fn get_composite_preview(
    width: u32,
    height: u32,
    state: State<'_, CompositeState>,
) -> Result<String, String> {
    let current = state.current.lock().map_err(|e| e.to_string())?;
    let composite = current.as_ref().ok_or("No composite loaded")?;

    let png_data = render_composite_to_png(composite, width, height)?;

    Ok(base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        &png_data,
    ))
}
