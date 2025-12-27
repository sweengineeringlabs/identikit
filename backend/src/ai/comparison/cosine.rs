//! Cosine Similarity
//!
//! Compare face embeddings using cosine similarity.
//! License: MIT (built-in)

use crate::ai::error::{AiError, AiResult};
use crate::ai::traits::SimilarityComputer;
use crate::models::{ComparisonConfig, FaceEmbedding, SimilarityMetric, SimilarityResult};

/// Default similarity threshold for face matching.
pub const DEFAULT_THRESHOLD: f32 = 0.6;

/// Cosine similarity computer.
pub struct CosineSimilarity {
    /// Default threshold if not specified in config.
    default_threshold: f32,
}

impl CosineSimilarity {
    /// Create a new cosine similarity computer.
    pub fn new() -> Self {
        Self {
            default_threshold: DEFAULT_THRESHOLD,
        }
    }

    /// Create with custom default threshold.
    pub fn with_threshold(threshold: f32) -> Self {
        Self {
            default_threshold: threshold,
        }
    }

    /// Compute cosine similarity between two vectors.
    ///
    /// Returns a value between -1.0 and 1.0, where 1.0 means identical.
    pub fn cosine_similarity(v1: &[f32], v2: &[f32]) -> f32 {
        if v1.len() != v2.len() || v1.is_empty() {
            return 0.0;
        }

        let dot: f32 = v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum();
        let norm1: f32 = v1.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm2: f32 = v2.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm1 > 0.0 && norm2 > 0.0 {
            dot / (norm1 * norm2)
        } else {
            0.0
        }
    }

    /// Compute Euclidean distance between two vectors.
    pub fn euclidean_distance(v1: &[f32], v2: &[f32]) -> f32 {
        if v1.len() != v2.len() {
            return f32::MAX;
        }

        v1.iter()
            .zip(v2.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f32>()
            .sqrt()
    }

    /// Compute Manhattan distance between two vectors.
    pub fn manhattan_distance(v1: &[f32], v2: &[f32]) -> f32 {
        if v1.len() != v2.len() {
            return f32::MAX;
        }

        v1.iter()
            .zip(v2.iter())
            .map(|(a, b)| (a - b).abs())
            .sum()
    }

    /// Convert distance to similarity (0.0 - 1.0 range).
    fn distance_to_similarity(distance: f32, max_distance: f32) -> f32 {
        if distance >= max_distance {
            0.0
        } else {
            1.0 - (distance / max_distance)
        }
    }
}

impl Default for CosineSimilarity {
    fn default() -> Self {
        Self::new()
    }
}

impl SimilarityComputer for CosineSimilarity {
    fn id(&self) -> &str {
        "cosine"
    }

    fn compare(
        &self,
        embedding1: &FaceEmbedding,
        embedding2: &FaceEmbedding,
        config: &ComparisonConfig,
    ) -> AiResult<SimilarityResult> {
        // Validate embeddings
        if embedding1.dimension() != embedding2.dimension() {
            return Err(AiError::DimensionMismatch {
                expected: embedding1.dimension(),
                actual: embedding2.dimension(),
            });
        }

        // Get vectors (normalize if requested)
        let v1 = if config.normalize {
            embedding1.normalized().vector
        } else {
            embedding1.vector.clone()
        };

        let v2 = if config.normalize {
            embedding2.normalized().vector
        } else {
            embedding2.vector.clone()
        };

        // Compute score based on metric
        let (score, normalized_score) = match config.metric {
            SimilarityMetric::Cosine => {
                let sim = Self::cosine_similarity(&v1, &v2);
                // Cosine similarity ranges from -1 to 1
                // Normalize to 0 to 1 for display
                let normalized = (sim + 1.0) / 2.0;
                (sim, normalized)
            }
            SimilarityMetric::Euclidean => {
                let dist = Self::euclidean_distance(&v1, &v2);
                // For normalized vectors, max Euclidean distance is 2.0
                let normalized = Self::distance_to_similarity(dist, 2.0);
                (dist, normalized)
            }
            SimilarityMetric::Manhattan => {
                let dist = Self::manhattan_distance(&v1, &v2);
                // Approximate max for normalized 128-dim vectors
                let max_dist = (embedding1.dimension() as f32).sqrt() * 2.0;
                let normalized = Self::distance_to_similarity(dist, max_dist);
                (dist, normalized)
            }
        };

        Ok(SimilarityResult::new(
            score,
            normalized_score,
            config.metric,
            config.threshold,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_embedding(vector: Vec<f32>) -> FaceEmbedding {
        FaceEmbedding::new(vector, "test")
    }

    #[test]
    fn test_cosine_similarity_identical() {
        let v = vec![0.5, 0.5, 0.5, 0.5];
        let sim = CosineSimilarity::cosine_similarity(&v, &v);
        assert!((sim - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_cosine_similarity_opposite() {
        let v1 = vec![1.0, 0.0, 0.0];
        let v2 = vec![-1.0, 0.0, 0.0];
        let sim = CosineSimilarity::cosine_similarity(&v1, &v2);
        assert!((sim - (-1.0)).abs() < 0.0001);
    }

    #[test]
    fn test_cosine_similarity_orthogonal() {
        let v1 = vec![1.0, 0.0, 0.0];
        let v2 = vec![0.0, 1.0, 0.0];
        let sim = CosineSimilarity::cosine_similarity(&v1, &v2);
        assert!(sim.abs() < 0.0001);
    }

    #[test]
    fn test_euclidean_distance() {
        let v1 = vec![0.0, 0.0, 0.0];
        let v2 = vec![3.0, 4.0, 0.0];
        let dist = CosineSimilarity::euclidean_distance(&v1, &v2);
        assert!((dist - 5.0).abs() < 0.0001);
    }

    #[test]
    fn test_compare_identical_embeddings() {
        let computer = CosineSimilarity::new();
        let emb = create_embedding(vec![0.5, 0.5, 0.5, 0.5]);
        let config = ComparisonConfig::default();

        let result = computer.compare(&emb, &emb, &config).unwrap();

        assert!(result.is_match);
        assert!((result.score - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_compare_different_embeddings() {
        let computer = CosineSimilarity::new();
        let emb1 = create_embedding(vec![1.0, 0.0, 0.0, 0.0]);
        let emb2 = create_embedding(vec![0.0, 1.0, 0.0, 0.0]);
        let config = ComparisonConfig::default();

        let result = computer.compare(&emb1, &emb2, &config).unwrap();

        assert!(!result.is_match);
        assert!(result.score.abs() < 0.0001);
    }

    #[test]
    fn test_dimension_mismatch() {
        let computer = CosineSimilarity::new();
        let emb1 = create_embedding(vec![1.0, 0.0, 0.0]);
        let emb2 = create_embedding(vec![1.0, 0.0, 0.0, 0.0]);
        let config = ComparisonConfig::default();

        let result = computer.compare(&emb1, &emb2, &config);
        assert!(matches!(result, Err(AiError::DimensionMismatch { .. })));
    }

    #[test]
    fn test_custom_threshold() {
        let computer = CosineSimilarity::new();
        let emb1 = create_embedding(vec![1.0, 0.0, 0.0, 0.0]);
        let emb2 = create_embedding(vec![0.7, 0.7, 0.0, 0.0]);  // Cosine similarity ~0.707

        // With high threshold, should not match
        let mut config = ComparisonConfig::default();
        config.threshold = 0.9;
        let result = computer.compare(&emb1, &emb2, &config).unwrap();
        assert!(!result.is_match);

        // With low threshold, should match
        config.threshold = 0.5;
        let result = computer.compare(&emb1, &emb2, &config).unwrap();
        assert!(result.is_match);
    }
}
