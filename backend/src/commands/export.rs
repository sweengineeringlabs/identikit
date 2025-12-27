//! Export Commands
//!
//! Export composites to PNG and PDF formats.

use serde::Serialize;
use tauri::State;

use super::composite::CompositeState;

/// Result of an export operation.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportResult {
    pub success: bool,
    pub path: Option<String>,
    pub error: Option<String>,
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

    // TODO: Implement actual PNG export using resvg/tiny-skia
    // This is a placeholder that would:
    // 1. Create a pixmap of the specified size
    // 2. Render each layer's SVG onto the pixmap
    // 3. Save the pixmap as PNG

    Ok(ExportResult {
        success: true,
        path: Some(path),
        error: None,
    })
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

    // TODO: Implement actual PDF export using printpdf
    // This would:
    // 1. Create a new PDF document
    // 2. Add a page with the composite image
    // 3. Optionally add a metadata page
    // 4. Save the PDF

    Ok(ExportResult {
        success: true,
        path: Some(path),
        error: None,
    })
}
