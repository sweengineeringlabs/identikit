//! Layer Model
//!
//! Represents a single layer in a composite.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::Transform;

/// A single layer in a facial composite.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Layer {
    /// Unique identifier for this layer
    pub id: Uuid,
    /// Reference to the feature asset
    pub feature_id: String,
    /// User-visible name
    pub name: String,
    /// Whether the layer is visible
    pub visible: bool,
    /// Whether the layer is locked for editing
    pub locked: bool,
    /// Opacity (0.0 - 1.0)
    pub opacity: f64,
    /// 2D transformation
    pub transform: Transform,
    /// Color overrides for specific zones
    pub color_overrides: HashMap<String, String>,
    /// Z-index for layer ordering
    pub z_index: i32,
}

impl Layer {
    /// Create a new layer with a feature.
    pub fn new(feature_id: String, name: String, z_index: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            feature_id,
            name,
            visible: true,
            locked: false,
            opacity: 1.0,
            transform: Transform::default(),
            color_overrides: HashMap::new(),
            z_index,
        }
    }

    /// Create a new layer with a centered transform.
    pub fn centered(
        feature_id: String,
        name: String,
        z_index: i32,
        canvas_width: u32,
        canvas_height: u32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            feature_id,
            name,
            visible: true,
            locked: false,
            opacity: 1.0,
            transform: Transform::centered(canvas_width, canvas_height),
            color_overrides: HashMap::new(),
            z_index,
        }
    }
}
