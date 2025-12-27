//! AI Error Types
//!
//! Error handling for AI operations.

use std::fmt;
use thiserror::Error;

/// AI-related errors.
#[derive(Debug, Error)]
pub enum AiError {
    /// Failed to initialize AI provider.
    #[error("Initialization failed: {0}")]
    InitializationFailed(String),

    /// Model file not found.
    #[error("Model not found: {0}")]
    ModelNotFound(String),

    /// Failed to load model.
    #[error("Model load failed: {0}")]
    ModelLoadFailed(String),

    /// No faces detected in the image.
    #[error("No faces detected in image")]
    NoFacesDetected,

    /// Multiple faces detected when one was expected.
    #[error("Multiple faces detected ({0}), expected single face")]
    MultipleFacesDetected(usize),

    /// Face detection failed.
    #[error("Face detection failed: {0}")]
    DetectionFailed(String),

    /// Face alignment failed.
    #[error("Face alignment failed: {0}")]
    AlignmentFailed(String),

    /// Embedding generation failed.
    #[error("Embedding generation failed: {0}")]
    EmbeddingFailed(String),

    /// Embedding dimension mismatch.
    #[error("Embedding dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },

    /// Image processing error.
    #[error("Image processing error: {0}")]
    ImageError(String),

    /// Invalid image data.
    #[error("Invalid image data: {0}")]
    InvalidImageData(String),

    /// Image too small for processing.
    #[error("Image too small: {width}x{height}, minimum is {min_size}x{min_size}")]
    ImageTooSmall {
        width: u32,
        height: u32,
        min_size: u32,
    },

    /// File I/O error.
    #[error("File error: {0}")]
    FileError(String),

    /// Provider not available.
    #[error("Provider not available: {0}")]
    ProviderNotAvailable(String),

    /// Provider not initialized.
    #[error("Provider not initialized: {0}")]
    ProviderNotInitialized(String),

    /// Configuration error.
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    /// Operation timeout.
    #[error("Operation timed out after {0}ms")]
    Timeout(u64),

    /// Internal error.
    #[error("Internal error: {0}")]
    Internal(String),
}

impl AiError {
    /// Check if error is recoverable.
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            AiError::NoFacesDetected
                | AiError::MultipleFacesDetected(_)
                | AiError::ImageTooSmall { .. }
                | AiError::Timeout(_)
        )
    }

    /// Check if error is related to initialization.
    pub fn is_initialization_error(&self) -> bool {
        matches!(
            self,
            AiError::InitializationFailed(_)
                | AiError::ModelNotFound(_)
                | AiError::ModelLoadFailed(_)
                | AiError::ProviderNotAvailable(_)
                | AiError::ConfigurationError(_)
        )
    }
}

impl From<std::io::Error> for AiError {
    fn from(err: std::io::Error) -> Self {
        AiError::FileError(err.to_string())
    }
}

impl From<image::ImageError> for AiError {
    fn from(err: image::ImageError) -> Self {
        AiError::ImageError(err.to_string())
    }
}

/// Result type for AI operations.
pub type AiResult<T> = Result<T, AiError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = AiError::NoFacesDetected;
        assert_eq!(err.to_string(), "No faces detected in image");

        let err = AiError::MultipleFacesDetected(3);
        assert_eq!(
            err.to_string(),
            "Multiple faces detected (3), expected single face"
        );
    }

    #[test]
    fn test_recoverable_errors() {
        assert!(AiError::NoFacesDetected.is_recoverable());
        assert!(AiError::MultipleFacesDetected(2).is_recoverable());
        assert!(AiError::Timeout(5000).is_recoverable());

        assert!(!AiError::ModelNotFound("test".into()).is_recoverable());
        assert!(!AiError::InitializationFailed("test".into()).is_recoverable());
    }

    #[test]
    fn test_initialization_errors() {
        assert!(AiError::InitializationFailed("test".into()).is_initialization_error());
        assert!(AiError::ModelNotFound("test".into()).is_initialization_error());
        assert!(AiError::ConfigurationError("test".into()).is_initialization_error());

        assert!(!AiError::NoFacesDetected.is_initialization_error());
        assert!(!AiError::EmbeddingFailed("test".into()).is_initialization_error());
    }
}
