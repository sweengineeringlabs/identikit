//! AI Data Models
//!
//! Data structures for face detection, alignment, and recognition.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A bounding box for face detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    /// X coordinate of top-left corner.
    pub x: f32,
    /// Y coordinate of top-left corner.
    pub y: f32,
    /// Width of the bounding box.
    pub width: f32,
    /// Height of the bounding box.
    pub height: f32,
}

impl BoundingBox {
    /// Create a new bounding box.
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    /// Get the center point of the bounding box.
    pub fn center(&self) -> (f32, f32) {
        (self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    /// Get the area of the bounding box.
    pub fn area(&self) -> f32 {
        self.width * self.height
    }

    /// Check if a point is inside the bounding box.
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    /// Calculate intersection over union (IoU) with another bounding box.
    pub fn iou(&self, other: &BoundingBox) -> f32 {
        let x1 = self.x.max(other.x);
        let y1 = self.y.max(other.y);
        let x2 = (self.x + self.width).min(other.x + other.width);
        let y2 = (self.y + self.height).min(other.y + other.height);

        if x2 <= x1 || y2 <= y1 {
            return 0.0;
        }

        let intersection = (x2 - x1) * (y2 - y1);
        let union = self.area() + other.area() - intersection;

        if union > 0.0 {
            intersection / union
        } else {
            0.0
        }
    }
}

/// Facial landmark points.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FaceLandmarks {
    /// All landmark points.
    pub points: Vec<(f32, f32)>,

    /// Left eye center (if available).
    pub left_eye: Option<(f32, f32)>,

    /// Right eye center (if available).
    pub right_eye: Option<(f32, f32)>,

    /// Nose tip (if available).
    pub nose_tip: Option<(f32, f32)>,

    /// Left mouth corner (if available).
    pub mouth_left: Option<(f32, f32)>,

    /// Right mouth corner (if available).
    pub mouth_right: Option<(f32, f32)>,
}

impl FaceLandmarks {
    /// Create landmarks from 5-point format (standard for alignment).
    pub fn from_5_points(points: [(f32, f32); 5]) -> Self {
        Self {
            points: points.to_vec(),
            left_eye: Some(points[0]),
            right_eye: Some(points[1]),
            nose_tip: Some(points[2]),
            mouth_left: Some(points[3]),
            mouth_right: Some(points[4]),
        }
    }

    /// Get the 5 key points for alignment if available.
    pub fn to_5_points(&self) -> Option<[(f32, f32); 5]> {
        Some([
            self.left_eye?,
            self.right_eye?,
            self.nose_tip?,
            self.mouth_left?,
            self.mouth_right?,
        ])
    }
}

impl Default for FaceLandmarks {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            left_eye: None,
            right_eye: None,
            nose_tip: None,
            mouth_left: None,
            mouth_right: None,
        }
    }
}

/// A detected face with bounding box and optional landmarks.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FaceDetection {
    /// Unique identifier for this detection.
    pub id: Uuid,

    /// Bounding box around the face.
    pub bounding_box: BoundingBox,

    /// Detection confidence score (0.0 - 1.0).
    pub confidence: f32,

    /// Facial landmarks (if detected).
    pub landmarks: Option<FaceLandmarks>,

    /// ID of the detector that produced this detection.
    pub detector_id: String,
}

impl FaceDetection {
    /// Create a new face detection.
    pub fn new(
        bounding_box: BoundingBox,
        confidence: f32,
        detector_id: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            bounding_box,
            confidence,
            landmarks: None,
            detector_id: detector_id.into(),
        }
    }

    /// Create with landmarks.
    pub fn with_landmarks(mut self, landmarks: FaceLandmarks) -> Self {
        self.landmarks = Some(landmarks);
        self
    }
}

/// A face embedding (feature vector).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FaceEmbedding {
    /// The embedding vector.
    pub vector: Vec<f32>,

    /// ID of the model that generated this embedding.
    pub model_id: String,

    /// Quality score of the face image used (0.0 - 1.0).
    pub quality_score: f32,

    /// Source face detection ID.
    pub face_id: Option<Uuid>,
}

impl FaceEmbedding {
    /// Create a new face embedding.
    pub fn new(vector: Vec<f32>, model_id: impl Into<String>) -> Self {
        Self {
            vector,
            model_id: model_id.into(),
            quality_score: 1.0,
            face_id: None,
        }
    }

    /// Get the dimensionality of the embedding.
    pub fn dimension(&self) -> usize {
        self.vector.len()
    }

    /// Normalize the embedding to unit length.
    pub fn normalize(&mut self) {
        let norm: f32 = self.vector.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for v in &mut self.vector {
                *v /= norm;
            }
        }
    }

    /// Get a normalized copy of the embedding.
    pub fn normalized(&self) -> Self {
        let mut copy = self.clone();
        copy.normalize();
        copy
    }
}

/// Similarity metric used for comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SimilarityMetric {
    /// Cosine similarity (-1.0 to 1.0, higher is more similar).
    Cosine,
    /// Euclidean distance (0.0+, lower is more similar).
    Euclidean,
    /// L1/Manhattan distance (0.0+, lower is more similar).
    Manhattan,
}

impl Default for SimilarityMetric {
    fn default() -> Self {
        SimilarityMetric::Cosine
    }
}

/// Result of comparing two face embeddings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimilarityResult {
    /// Raw similarity/distance score.
    pub score: f32,

    /// Normalized score (0.0 - 1.0, higher is more similar).
    pub normalized_score: f32,

    /// Metric used for comparison.
    pub metric: SimilarityMetric,

    /// Whether the faces are considered a match.
    pub is_match: bool,

    /// Threshold used for matching decision.
    pub threshold: f32,

    /// Confidence in the match decision (0.0 - 1.0).
    pub confidence: f32,
}

impl SimilarityResult {
    /// Create a new similarity result.
    pub fn new(
        score: f32,
        normalized_score: f32,
        metric: SimilarityMetric,
        threshold: f32,
    ) -> Self {
        // For cosine similarity, higher is more similar
        // For distance metrics, lower is more similar
        let is_match = match metric {
            SimilarityMetric::Cosine => score >= threshold,
            SimilarityMetric::Euclidean | SimilarityMetric::Manhattan => score <= threshold,
        };

        // Calculate confidence based on how far from threshold
        let confidence = match metric {
            SimilarityMetric::Cosine => {
                if is_match {
                    ((score - threshold) / (1.0 - threshold)).clamp(0.0, 1.0)
                } else {
                    ((threshold - score) / threshold).clamp(0.0, 1.0)
                }
            }
            _ => {
                if is_match {
                    ((threshold - score) / threshold).clamp(0.0, 1.0)
                } else {
                    // Hard to define confidence for non-match with distance
                    0.5
                }
            }
        };

        Self {
            score,
            normalized_score,
            metric,
            is_match,
            threshold,
            confidence,
        }
    }

    /// Create a high-confidence match result.
    pub fn match_result(score: f32, threshold: f32) -> Self {
        Self::new(score, score, SimilarityMetric::Cosine, threshold)
    }

    /// Create a high-confidence non-match result.
    pub fn no_match(score: f32, threshold: f32) -> Self {
        Self::new(score, score, SimilarityMetric::Cosine, threshold)
    }
}

/// Source of an image for AI processing.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ImageSource {
    /// Image from a file path.
    FilePath { path: String },

    /// Image from base64-encoded data.
    Base64 { data: String, mime_type: Option<String> },

    /// Image from raw bytes.
    #[serde(skip)]
    Bytes { data: Vec<u8> },

    /// Use the current composite rendered as an image.
    CurrentComposite,
}

impl ImageSource {
    /// Create from file path.
    pub fn from_path(path: impl Into<String>) -> Self {
        ImageSource::FilePath { path: path.into() }
    }

    /// Create from base64 data.
    pub fn from_base64(data: impl Into<String>) -> Self {
        ImageSource::Base64 {
            data: data.into(),
            mime_type: None,
        }
    }

    /// Create from raw bytes.
    pub fn from_bytes(data: Vec<u8>) -> Self {
        ImageSource::Bytes { data }
    }
}

/// Configuration for face detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetectionConfig {
    /// Minimum confidence threshold (0.0 - 1.0).
    pub min_confidence: f32,

    /// Minimum face size in pixels.
    pub min_face_size: u32,

    /// Maximum number of faces to detect (0 = unlimited).
    pub max_faces: usize,

    /// Whether to detect landmarks.
    pub detect_landmarks: bool,

    /// Whether to use GPU acceleration if available.
    pub use_gpu: bool,
}

impl Default for DetectionConfig {
    fn default() -> Self {
        Self {
            min_confidence: 0.5,
            min_face_size: 20,
            max_faces: 0,
            detect_landmarks: true,
            use_gpu: false,
        }
    }
}

/// Configuration for face comparison.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComparisonConfig {
    /// Similarity threshold for matching.
    pub threshold: f32,

    /// Similarity metric to use.
    pub metric: SimilarityMetric,

    /// Whether to normalize embeddings before comparison.
    pub normalize: bool,
}

impl Default for ComparisonConfig {
    fn default() -> Self {
        Self {
            threshold: 0.6,
            metric: SimilarityMetric::Cosine,
            normalize: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounding_box_center() {
        let bbox = BoundingBox::new(10.0, 20.0, 100.0, 80.0);
        let (cx, cy) = bbox.center();
        assert_eq!(cx, 60.0);
        assert_eq!(cy, 60.0);
    }

    #[test]
    fn test_bounding_box_iou() {
        let box1 = BoundingBox::new(0.0, 0.0, 100.0, 100.0);
        let box2 = BoundingBox::new(50.0, 50.0, 100.0, 100.0);

        let iou = box1.iou(&box2);
        assert!(iou > 0.0 && iou < 1.0);

        // Same box should have IoU of 1.0
        assert!((box1.iou(&box1) - 1.0).abs() < 0.001);

        // Non-overlapping boxes should have IoU of 0.0
        let box3 = BoundingBox::new(200.0, 200.0, 50.0, 50.0);
        assert_eq!(box1.iou(&box3), 0.0);
    }

    #[test]
    fn test_face_landmarks_5_points() {
        let points = [
            (30.0, 40.0),
            (70.0, 40.0),
            (50.0, 60.0),
            (35.0, 80.0),
            (65.0, 80.0),
        ];
        let landmarks = FaceLandmarks::from_5_points(points);

        assert_eq!(landmarks.left_eye, Some((30.0, 40.0)));
        assert_eq!(landmarks.right_eye, Some((70.0, 40.0)));
        assert_eq!(landmarks.nose_tip, Some((50.0, 60.0)));

        let recovered = landmarks.to_5_points().unwrap();
        assert_eq!(recovered, points);
    }

    #[test]
    fn test_embedding_normalize() {
        let mut embedding = FaceEmbedding::new(vec![3.0, 4.0], "test");
        embedding.normalize();

        let norm: f32 = embedding.vector.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_similarity_result_match() {
        // High similarity should be a match
        let result = SimilarityResult::new(0.8, 0.8, SimilarityMetric::Cosine, 0.6);
        assert!(result.is_match);

        // Low similarity should not be a match
        let result = SimilarityResult::new(0.4, 0.4, SimilarityMetric::Cosine, 0.6);
        assert!(!result.is_match);
    }
}
