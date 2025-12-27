//! AI State
//!
//! Provider registry and shared state for AI operations.

use std::sync::Arc;

use image::DynamicImage;
use parking_lot::RwLock;

use crate::models::{
    ComparisonConfig, DetectionConfig, FaceDetection, FaceEmbedding, SimilarityResult,
};

use super::alignment::AffineAligner;
use super::comparison::CosineSimilarity;
use super::detection::RustfaceDetector;
use super::embedding::DlibEmbedder;
use super::error::{AiError, AiResult};
use super::traits::{FaceAligner, FaceDetector, FaceEmbedder, SimilarityComputer};

/// AI provider registry and state manager.
pub struct AiState {
    /// Face detector provider.
    detector: Arc<RwLock<Box<dyn FaceDetector>>>,
    /// Face aligner provider.
    aligner: Arc<RwLock<Box<dyn FaceAligner>>>,
    /// Face embedder provider.
    embedder: Arc<RwLock<Box<dyn FaceEmbedder>>>,
    /// Similarity computer.
    similarity: Arc<Box<dyn SimilarityComputer>>,
    /// Whether the AI system has been initialized.
    initialized: Arc<RwLock<bool>>,
}

impl AiState {
    /// Create a new AI state with default providers.
    pub fn new() -> Self {
        Self {
            detector: Arc::new(RwLock::new(Box::new(RustfaceDetector::new()))),
            aligner: Arc::new(RwLock::new(Box::new(AffineAligner::new()))),
            embedder: Arc::new(RwLock::new(Box::new(DlibEmbedder::new()))),
            similarity: Arc::new(Box::new(CosineSimilarity::new())),
            initialized: Arc::new(RwLock::new(false)),
        }
    }

    /// Check if AI system is initialized.
    pub fn is_initialized(&self) -> bool {
        *self.initialized.read()
    }

    /// Check if all providers are ready.
    pub fn is_ready(&self) -> bool {
        self.detector.read().is_ready() && self.embedder.read().is_ready()
    }

    /// Initialize all AI providers.
    pub async fn initialize(&self) -> AiResult<()> {
        // Initialize detector - drop lock before await
        {
            let mut detector = self.detector.write();
            // Note: In a real implementation, this would be async
            // For now, we use a blocking approach to avoid Send issues
        }

        // For now, mark as initialized without full model loading
        // Full initialization would require proper async handling
        *self.initialized.write() = true;
        tracing::info!("AI system initialized (stub mode)");

        Ok(())
    }

    /// Get the current detector.
    pub fn detector(&self) -> Arc<RwLock<Box<dyn FaceDetector>>> {
        self.detector.clone()
    }

    /// Get the current aligner.
    pub fn aligner(&self) -> Arc<RwLock<Box<dyn FaceAligner>>> {
        self.aligner.clone()
    }

    /// Get the current embedder.
    pub fn embedder(&self) -> Arc<RwLock<Box<dyn FaceEmbedder>>> {
        self.embedder.clone()
    }

    /// Get the similarity computer.
    pub fn similarity(&self) -> Arc<Box<dyn SimilarityComputer>> {
        self.similarity.clone()
    }

    /// Load image from file path.
    pub fn load_image(path: &str) -> AiResult<DynamicImage> {
        if !std::path::Path::new(path).exists() {
            return Err(AiError::FileError(format!("File not found: {}", path)));
        }

        image::open(path).map_err(|e| AiError::ImageError(e.to_string()))
    }

    /// Load image from base64 string.
    pub fn load_image_base64(data: &str) -> AiResult<DynamicImage> {
        let bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, data)
            .map_err(|e| AiError::InvalidImageData(format!("Base64 decode error: {}", e)))?;

        image::load_from_memory(&bytes).map_err(|e| AiError::ImageError(e.to_string()))
    }

    /// Detect faces in an image.
    pub async fn detect_faces(
        &self,
        image: &DynamicImage,
        config: &DetectionConfig,
    ) -> AiResult<Vec<FaceDetection>> {
        // Clone the detector to avoid holding lock across await
        let detector = self.detector.clone();
        let image = image.clone();
        let config = config.clone();

        tokio::task::spawn_blocking(move || {
            let det = detector.read();
            // Use synchronous detection for now
            // In production, this would use proper async handling
            det.detect_sync(&image, &config)
        })
        .await
        .map_err(|e| AiError::Internal(format!("Task join error: {}", e)))?
    }

    /// Align a detected face.
    pub fn align_face(
        &self,
        image: &DynamicImage,
        detection: &FaceDetection,
    ) -> AiResult<DynamicImage> {
        let aligner = self.aligner.read();

        if let Some(ref landmarks) = detection.landmarks {
            aligner.align(image, landmarks)
        } else {
            aligner.align_from_bbox(image, detection)
        }
    }

    /// Generate embedding from aligned face.
    pub async fn generate_embedding(
        &self,
        aligned_face: &DynamicImage,
    ) -> AiResult<FaceEmbedding> {
        let embedder = self.embedder.clone();
        let face = aligned_face.clone();

        tokio::task::spawn_blocking(move || {
            let emb = embedder.read();
            emb.embed_sync(&face)
        })
        .await
        .map_err(|e| AiError::Internal(format!("Task join error: {}", e)))?
    }

    /// Compare two embeddings.
    pub fn compare_embeddings(
        &self,
        emb1: &FaceEmbedding,
        emb2: &FaceEmbedding,
        config: &ComparisonConfig,
    ) -> AiResult<SimilarityResult> {
        self.similarity.compare(emb1, emb2, config)
    }

    /// Full pipeline: detect, align, and embed a face from an image.
    pub async fn extract_embedding(
        &self,
        image: &DynamicImage,
        detection_config: &DetectionConfig,
    ) -> AiResult<FaceEmbedding> {
        // Detect faces
        let detections = self.detect_faces(image, detection_config).await?;

        let detection = match detections.len() {
            0 => return Err(AiError::NoFacesDetected),
            1 => &detections[0],
            n => return Err(AiError::MultipleFacesDetected(n)),
        };

        // Align face
        let aligned = self.align_face(image, detection)?;

        // Generate embedding
        let mut embedding = self.generate_embedding(&aligned).await?;
        embedding.face_id = Some(detection.id);

        Ok(embedding)
    }

    /// Compare two images for face similarity.
    pub async fn compare_images(
        &self,
        image1: &DynamicImage,
        image2: &DynamicImage,
        detection_config: &DetectionConfig,
        comparison_config: &ComparisonConfig,
    ) -> AiResult<SimilarityResult> {
        // Extract embeddings from both images
        let emb1 = self.extract_embedding(image1, detection_config).await?;
        let emb2 = self.extract_embedding(image2, detection_config).await?;

        // Compare
        self.compare_embeddings(&emb1, &emb2, comparison_config)
    }

    /// Compare a reference image to a rendered composite.
    pub async fn compare_to_composite(
        &self,
        reference: &DynamicImage,
        composite_png: &[u8],
        detection_config: &DetectionConfig,
        comparison_config: &ComparisonConfig,
    ) -> AiResult<SimilarityResult> {
        // Load composite image
        let composite = image::load_from_memory(composite_png)
            .map_err(|e| AiError::ImageError(e.to_string()))?;

        // Compare
        self.compare_images(reference, &composite, detection_config, comparison_config)
            .await
    }
}

impl Default for AiState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_state_creation() {
        let state = AiState::new();
        assert!(!state.is_initialized());
    }
}
