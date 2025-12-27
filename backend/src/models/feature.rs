//! Feature Model
//!
//! Represents facial feature assets.

use serde::{Deserialize, Serialize};

/// Categories of facial features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FeatureCategory {
    Hair,
    FaceShape,
    Eyes,
    Eyebrows,
    Nose,
    Mouth,
    Chin,
    Ears,
    Accessories,
}

impl FeatureCategory {
    /// Get all feature categories.
    pub fn all() -> Vec<Self> {
        vec![
            Self::Hair,
            Self::FaceShape,
            Self::Eyes,
            Self::Eyebrows,
            Self::Nose,
            Self::Mouth,
            Self::Chin,
            Self::Ears,
            Self::Accessories,
        ]
    }

    /// Get the display name for this category.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Hair => "Hair",
            Self::FaceShape => "Face Shape",
            Self::Eyes => "Eyes",
            Self::Eyebrows => "Eyebrows",
            Self::Nose => "Nose",
            Self::Mouth => "Mouth",
            Self::Chin => "Chin",
            Self::Ears => "Ears",
            Self::Accessories => "Accessories",
        }
    }

    /// Get the folder name for this category.
    pub fn folder_name(&self) -> &'static str {
        match self {
            Self::Hair => "hair",
            Self::FaceShape => "face-shape",
            Self::Eyes => "eyes",
            Self::Eyebrows => "eyebrows",
            Self::Nose => "nose",
            Self::Mouth => "mouth",
            Self::Chin => "chin",
            Self::Ears => "ears",
            Self::Accessories => "accessories",
        }
    }
}

/// A 2D point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

/// A color zone that can be customized.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorZone {
    pub id: String,
    pub name: String,
    pub default_color: String,
}

/// Metadata for a feature asset.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureMetadata {
    pub width: f64,
    pub height: f64,
    pub anchor_point: Point2D,
    pub color_zones: Vec<ColorZone>,
}

/// A facial feature asset.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feature {
    /// Unique identifier
    pub id: String,
    /// Display name
    pub name: String,
    /// Category
    pub category: FeatureCategory,
    /// Path to SVG file
    pub svg_path: String,
    /// Path to thumbnail (optional)
    pub thumbnail_path: Option<String>,
    /// Searchable tags
    pub tags: Vec<String>,
    /// Asset metadata
    pub metadata: FeatureMetadata,
}

/// Category information for the UI.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryInfo {
    pub id: FeatureCategory,
    pub name: String,
    pub folder: String,
    pub count: usize,
}
