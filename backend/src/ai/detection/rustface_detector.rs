//! Rustface Face Detector
//!
//! Pure Rust face detection (stub implementation).
//! License: BSD
//!
//! Note: The actual rustface crate requires model files. This provides
//! a stub implementation that simulates face detection.

use std::path::PathBuf;

use image::{DynamicImage, GenericImageView};

use crate::ai::error::{AiError, AiResult};
use crate::ai::traits::{FaceDetector, ProviderInfo};
use crate::models::{BoundingBox, DetectionConfig, FaceDetection, FaceLandmarks};

/// Path to the default model file.
const DEFAULT_MODEL_PATH: &str = "models/seeta_fd_frontal_v1.0.bin";

/// Rustface detector implementation.
pub struct RustfaceDetector {
    /// Path to the model file.
    model_path: PathBuf,
    /// Whether the detector is initialized.
    initialized: bool,
}

impl RustfaceDetector {
    /// Create a new rustface detector with default model path.
    pub fn new() -> Self {
        Self {
            model_path: PathBuf::from(DEFAULT_MODEL_PATH),
            initialized: true, // Stub is always ready
        }
    }

    /// Create with custom model path.
    pub fn with_model_path(path: impl Into<PathBuf>) -> Self {
        Self {
            model_path: path.into(),
            initialized: true,
        }
    }

    /// Get provider info.
    pub fn info() -> ProviderInfo {
        ProviderInfo::new("rustface", "Rustface", "0.1")
            .with_license("BSD-3-Clause", true)
            .with_description("Pure Rust face detection (stub)")
    }

    /// Estimate 5-point landmarks from bounding box.
    fn estimate_landmarks_from_bbox(bbox: &BoundingBox) -> FaceLandmarks {
        let cx = bbox.x + bbox.width / 2.0;
        let cy = bbox.y + bbox.height / 2.0;

        let eye_y = cy - bbox.height * 0.15;
        let eye_offset_x = bbox.width * 0.2;
        let nose_y = cy + bbox.height * 0.05;
        let mouth_y = cy + bbox.height * 0.25;
        let mouth_offset_x = bbox.width * 0.15;

        let points = [
            (cx - eye_offset_x, eye_y),
            (cx + eye_offset_x, eye_y),
            (cx, nose_y),
            (cx - mouth_offset_x, mouth_y),
            (cx + mouth_offset_x, mouth_y),
        ];

        FaceLandmarks::from_5_points(points)
    }

    /// Stub detection - returns a centered face.
    fn detect_stub(&self, image: &DynamicImage, _config: &DetectionConfig) -> Vec<FaceDetection> {
        let (width, height) = image.dimensions();

        // Generate a centered face detection
        let face_width = width as f32 * 0.4;
        let face_height = height as f32 * 0.5;
        let face_x = (width as f32 - face_width) / 2.0;
        let face_y = (height as f32 - face_height) / 3.0;

        let bbox = BoundingBox::new(face_x, face_y, face_width, face_height);
        let landmarks = Self::estimate_landmarks_from_bbox(&bbox);

        vec![FaceDetection::new(bbox, 0.95, self.id()).with_landmarks(landmarks)]
    }
}

impl Default for RustfaceDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl FaceDetector for RustfaceDetector {
    fn id(&self) -> &str {
        "rustface"
    }

    fn name(&self) -> &str {
        "Rustface (stub)"
    }

    fn version(&self) -> &str {
        "0.1"
    }

    fn is_ready(&self) -> bool {
        self.initialized
    }

    fn detect_sync(
        &self,
        image: &DynamicImage,
        config: &DetectionConfig,
    ) -> AiResult<Vec<FaceDetection>> {
        if !self.is_ready() {
            return Err(AiError::ProviderNotInitialized(self.id().to_string()));
        }

        let (width, height) = image.dimensions();
        let min_dim = width.min(height);

        if min_dim < config.min_face_size {
            return Err(AiError::ImageTooSmall {
                width,
                height,
                min_size: config.min_face_size,
            });
        }

        let detections = self.detect_stub(image, config);

        tracing::debug!(
            "Rustface detected {} faces (stub)",
            detections.len()
        );

        Ok(detections)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbImage;

    fn create_test_image() -> DynamicImage {
        let img = RgbImage::from_fn(200, 200, |x, y| {
            image::Rgb([
                ((x + y) % 256) as u8,
                ((x * 2) % 256) as u8,
                ((y * 2) % 256) as u8,
            ])
        });
        DynamicImage::ImageRgb8(img)
    }

    #[test]
    fn test_provider_info() {
        let info = RustfaceDetector::info();
        assert_eq!(info.id, "rustface");
        assert!(info.is_open_source);
    }

    #[test]
    fn test_detect_faces() {
        let detector = RustfaceDetector::new();
        let image = create_test_image();
        let config = DetectionConfig::default();

        let detections = detector.detect_sync(&image, &config).unwrap();
        assert_eq!(detections.len(), 1);
        assert!(detections[0].confidence > 0.5);
    }
}
