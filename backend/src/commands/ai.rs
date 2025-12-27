//! AI Commands
//!
//! Tauri commands for AI-powered face recognition.

use serde::{Deserialize, Serialize};
use tauri::State;

use crate::ai::AiState;
use crate::commands::composite::CompositeState;
use crate::models::{
    ComparisonConfig, DetectionConfig, FaceDetection, ImageSource, SimilarityResult,
};
use crate::services::image_processor::render_composite_to_png;

/// Result of AI operations.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiOperationResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> AiOperationResult<T> {
    fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    fn error(msg: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.into()),
        }
    }
}

/// AI system status.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiStatus {
    pub initialized: bool,
    pub ready: bool,
    pub detector_id: String,
    pub embedder_id: String,
}

/// Initialize the AI system.
#[tauri::command]
pub async fn initialize_ai(state: State<'_, AiState>) -> Result<AiOperationResult<()>, String> {
    match state.initialize().await {
        Ok(()) => {
            tracing::info!("AI system initialized successfully");
            Ok(AiOperationResult::success(()))
        }
        Err(e) => {
            tracing::error!("AI initialization failed: {}", e);
            Ok(AiOperationResult::error(e.to_string()))
        }
    }
}

/// Check if AI system is ready.
#[tauri::command]
pub fn is_ai_ready(state: State<'_, AiState>) -> bool {
    state.is_ready()
}

/// Get AI system status.
#[tauri::command]
pub fn get_ai_status(state: State<'_, AiState>) -> AiStatus {
    let detector_id = state.detector().read().id().to_string();
    let embedder_id = state.embedder().read().id().to_string();

    AiStatus {
        initialized: state.is_initialized(),
        ready: state.is_ready(),
        detector_id,
        embedder_id,
    }
}

/// Detect faces in an image.
#[tauri::command]
pub async fn detect_faces(
    source: ImageSource,
    config: Option<DetectionConfig>,
    ai_state: State<'_, AiState>,
    composite_state: State<'_, CompositeState>,
) -> Result<AiOperationResult<Vec<FaceDetection>>, String> {
    let config = config.unwrap_or_default();

    // Load image based on source
    let image = match source {
        ImageSource::FilePath { path } => match AiState::load_image(&path) {
            Ok(img) => img,
            Err(e) => return Ok(AiOperationResult::error(e.to_string())),
        },
        ImageSource::Base64 { data, .. } => match AiState::load_image_base64(&data) {
            Ok(img) => img,
            Err(e) => return Ok(AiOperationResult::error(e.to_string())),
        },
        ImageSource::Bytes { data } => {
            match image::load_from_memory(&data) {
                Ok(img) => img,
                Err(e) => return Ok(AiOperationResult::error(e.to_string())),
            }
        }
        ImageSource::CurrentComposite => {
            // Render current composite
            let current = composite_state.current.lock().map_err(|e| e.to_string())?;
            let composite = match current.as_ref() {
                Some(c) => c,
                None => return Ok(AiOperationResult::error("No composite loaded")),
            };

            let png_data = match render_composite_to_png(composite, composite.canvas.width, composite.canvas.height) {
                Ok(data) => data,
                Err(e) => return Ok(AiOperationResult::error(e)),
            };

            match image::load_from_memory(&png_data) {
                Ok(img) => img,
                Err(e) => return Ok(AiOperationResult::error(e.to_string())),
            }
        }
    };

    // Detect faces
    match ai_state.detect_faces(&image, &config).await {
        Ok(detections) => {
            tracing::info!("Detected {} faces", detections.len());
            Ok(AiOperationResult::success(detections))
        }
        Err(e) => {
            tracing::warn!("Face detection failed: {}", e);
            Ok(AiOperationResult::error(e.to_string()))
        }
    }
}

/// Compare two images for face similarity.
#[tauri::command]
pub async fn compare_faces(
    source1: ImageSource,
    source2: ImageSource,
    threshold: Option<f32>,
    ai_state: State<'_, AiState>,
    composite_state: State<'_, CompositeState>,
) -> Result<AiOperationResult<SimilarityResult>, String> {
    let detection_config = DetectionConfig::default();
    let mut comparison_config = ComparisonConfig::default();
    if let Some(t) = threshold {
        comparison_config.threshold = t;
    }

    // Helper to load image
    let load_image = |source: ImageSource| -> Result<image::DynamicImage, String> {
        match source {
            ImageSource::FilePath { path } => {
                AiState::load_image(&path).map_err(|e| e.to_string())
            }
            ImageSource::Base64 { data, .. } => {
                AiState::load_image_base64(&data).map_err(|e| e.to_string())
            }
            ImageSource::Bytes { data } => {
                image::load_from_memory(&data).map_err(|e| e.to_string())
            }
            ImageSource::CurrentComposite => {
                // Can't borrow composite_state in closure, handle separately
                Err("CurrentComposite must be handled separately".to_string())
            }
        }
    };

    // Load first image
    let image1 = if matches!(source1, ImageSource::CurrentComposite) {
        let current = composite_state.current.lock().map_err(|e| e.to_string())?;
        let composite = current.as_ref().ok_or("No composite loaded")?;
        let png_data = render_composite_to_png(composite, composite.canvas.width, composite.canvas.height)?;
        image::load_from_memory(&png_data).map_err(|e| e.to_string())?
    } else {
        match load_image(source1) {
            Ok(img) => img,
            Err(e) => return Ok(AiOperationResult::error(e)),
        }
    };

    // Load second image
    let image2 = if matches!(source2, ImageSource::CurrentComposite) {
        let current = composite_state.current.lock().map_err(|e| e.to_string())?;
        let composite = current.as_ref().ok_or("No composite loaded")?;
        let png_data = render_composite_to_png(composite, composite.canvas.width, composite.canvas.height)?;
        image::load_from_memory(&png_data).map_err(|e| e.to_string())?
    } else {
        match load_image(source2) {
            Ok(img) => img,
            Err(e) => return Ok(AiOperationResult::error(e)),
        }
    };

    // Compare
    match ai_state
        .compare_images(&image1, &image2, &detection_config, &comparison_config)
        .await
    {
        Ok(result) => {
            tracing::info!(
                "Face comparison: score={:.3}, match={}",
                result.score,
                result.is_match
            );
            Ok(AiOperationResult::success(result))
        }
        Err(e) => {
            tracing::warn!("Face comparison failed: {}", e);
            Ok(AiOperationResult::error(e.to_string()))
        }
    }
}

/// Compare a reference image to the current composite.
#[tauri::command]
pub async fn compare_to_composite(
    reference_path: String,
    threshold: Option<f32>,
    ai_state: State<'_, AiState>,
    composite_state: State<'_, CompositeState>,
) -> Result<AiOperationResult<SimilarityResult>, String> {
    let detection_config = DetectionConfig::default();
    let mut comparison_config = ComparisonConfig::default();
    if let Some(t) = threshold {
        comparison_config.threshold = t;
    }

    // Load reference image
    let reference = match AiState::load_image(&reference_path) {
        Ok(img) => img,
        Err(e) => return Ok(AiOperationResult::error(e.to_string())),
    };

    // Render current composite
    let composite_png = {
        let current = composite_state.current.lock().map_err(|e| e.to_string())?;
        let composite = match current.as_ref() {
            Some(c) => c,
            None => return Ok(AiOperationResult::error("No composite loaded")),
        };

        match render_composite_to_png(composite, composite.canvas.width, composite.canvas.height) {
            Ok(data) => data,
            Err(e) => return Ok(AiOperationResult::error(e)),
        }
    };

    // Compare
    match ai_state
        .compare_to_composite(&reference, &composite_png, &detection_config, &comparison_config)
        .await
    {
        Ok(result) => {
            tracing::info!(
                "Composite comparison: score={:.3}, match={}",
                result.score,
                result.is_match
            );
            Ok(AiOperationResult::success(result))
        }
        Err(e) => {
            tracing::warn!("Composite comparison failed: {}", e);
            Ok(AiOperationResult::error(e.to_string()))
        }
    }
}
