//! AI Provider Traits (SPI)
//!
//! Service Provider Interface definitions for face detection, alignment, and recognition.

use image::DynamicImage;

use crate::models::{
    ComparisonConfig, DetectionConfig, FaceDetection, FaceEmbedding, FaceLandmarks,
    SimilarityResult,
};

use super::error::AiResult;

/// Face detection provider trait.
///
/// Implementations detect faces in images and optionally extract landmarks.
pub trait FaceDetector: Send + Sync {
    /// Get the unique identifier for this detector.
    fn id(&self) -> &str;

    /// Get the display name for this detector.
    fn name(&self) -> &str;

    /// Get the version of this detector.
    fn version(&self) -> &str;

    /// Check if the detector is initialized and ready.
    fn is_ready(&self) -> bool;

    /// Detect faces in an image (synchronous).
    fn detect_sync(
        &self,
        image: &DynamicImage,
        config: &DetectionConfig,
    ) -> AiResult<Vec<FaceDetection>>;
}

/// Face alignment provider trait.
///
/// Implementations align/normalize face images for embedding extraction.
pub trait FaceAligner: Send + Sync {
    /// Get the unique identifier for this aligner.
    fn id(&self) -> &str;

    /// Get the output image size.
    fn output_size(&self) -> (u32, u32);

    /// Align a face image using landmarks.
    ///
    /// Takes the original image and 5-point landmarks, returns an aligned face crop.
    fn align(
        &self,
        image: &DynamicImage,
        landmarks: &FaceLandmarks,
    ) -> AiResult<DynamicImage>;

    /// Align a face using the bounding box (fallback when landmarks unavailable).
    fn align_from_bbox(
        &self,
        image: &DynamicImage,
        detection: &FaceDetection,
    ) -> AiResult<DynamicImage>;
}

/// Face embedding provider trait.
///
/// Implementations generate feature vectors from aligned face images.
pub trait FaceEmbedder: Send + Sync {
    /// Get the unique identifier for this embedder.
    fn id(&self) -> &str;

    /// Get the display name for this embedder.
    fn name(&self) -> &str;

    /// Get the version of this embedder.
    fn version(&self) -> &str;

    /// Get the embedding dimension.
    fn dimension(&self) -> usize;

    /// Check if the embedder is initialized and ready.
    fn is_ready(&self) -> bool;

    /// Generate an embedding from an aligned face image (synchronous).
    fn embed_sync(&self, aligned_face: &DynamicImage) -> AiResult<FaceEmbedding>;
}

/// Similarity computation trait.
///
/// Implementations compare face embeddings to determine similarity.
pub trait SimilarityComputer: Send + Sync {
    /// Get the unique identifier for this computer.
    fn id(&self) -> &str;

    /// Compare two embeddings and return a similarity result.
    fn compare(
        &self,
        embedding1: &FaceEmbedding,
        embedding2: &FaceEmbedding,
        config: &ComparisonConfig,
    ) -> AiResult<SimilarityResult>;

    /// Find the most similar embedding from a list.
    fn find_most_similar(
        &self,
        query: &FaceEmbedding,
        candidates: &[FaceEmbedding],
        config: &ComparisonConfig,
    ) -> AiResult<Option<(usize, SimilarityResult)>> {
        if candidates.is_empty() {
            return Ok(None);
        }

        let mut best_idx = 0;
        let mut best_result = self.compare(query, &candidates[0], config)?;

        for (i, candidate) in candidates.iter().enumerate().skip(1) {
            let result = self.compare(query, candidate, config)?;
            if result.normalized_score > best_result.normalized_score {
                best_idx = i;
                best_result = result;
            }
        }

        Ok(Some((best_idx, best_result)))
    }

    /// Compare a query against all candidates, returning all results.
    fn compare_all(
        &self,
        query: &FaceEmbedding,
        candidates: &[FaceEmbedding],
        config: &ComparisonConfig,
    ) -> AiResult<Vec<SimilarityResult>> {
        candidates
            .iter()
            .map(|c| self.compare(query, c, config))
            .collect()
    }
}

/// Provider information for display purposes.
#[derive(Debug, Clone)]
pub struct ProviderInfo {
    /// Unique identifier.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Version string.
    pub version: String,
    /// License information.
    pub license: String,
    /// Whether the provider is open source.
    pub is_open_source: bool,
    /// Description of the provider.
    pub description: String,
}

impl ProviderInfo {
    /// Create new provider info.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            version: version.into(),
            license: "Unknown".to_string(),
            is_open_source: false,
            description: String::new(),
        }
    }

    /// Set the license.
    pub fn with_license(mut self, license: impl Into<String>, is_open_source: bool) -> Self {
        self.license = license.into();
        self.is_open_source = is_open_source;
        self
    }

    /// Set the description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_info() {
        let info = ProviderInfo::new("test-detector", "Test Detector", "1.0.0")
            .with_license("MIT", true)
            .with_description("A test face detector");

        assert_eq!(info.id, "test-detector");
        assert_eq!(info.name, "Test Detector");
        assert_eq!(info.version, "1.0.0");
        assert_eq!(info.license, "MIT");
        assert!(info.is_open_source);
        assert_eq!(info.description, "A test face detector");
    }
}
