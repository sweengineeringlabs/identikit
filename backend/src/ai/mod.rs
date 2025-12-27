//! AI Module
//!
//! Face detection, alignment, embedding, and comparison functionality.
//!
//! This module implements the SPI (Service Provider Interface) pattern,
//! allowing pluggable providers for each AI capability. Default providers
//! use open-source models (MIT/Apache licensed) for commercial friendliness.

pub mod alignment;
pub mod comparison;
pub mod detection;
pub mod embedding;
pub mod error;
pub mod state;
pub mod traits;

pub use error::{AiError, AiResult};
pub use state::AiState;
pub use traits::{
    FaceAligner, FaceDetector, FaceEmbedder, ProviderInfo, SimilarityComputer,
};
