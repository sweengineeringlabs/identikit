//! Composite Model
//!
//! Represents a complete facial composite.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Layer;

/// File format version.
pub const FORMAT_VERSION: &str = "1.0.0";

/// Metadata for a composite.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompositeMetadata {
    /// Format version
    pub version: String,
    /// Author/creator name
    pub author: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Case number for law enforcement
    pub case_number: Option<String>,
    /// Searchable tags
    pub tags: Vec<String>,
}

impl CompositeMetadata {
    pub fn new() -> Self {
        Self {
            version: FORMAT_VERSION.to_string(),
            ..Default::default()
        }
    }
}

/// Canvas configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CanvasConfig {
    pub width: u32,
    pub height: u32,
    pub background_color: String,
}

impl Default for CanvasConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 1000,
            background_color: "#FFFFFF".to_string(),
        }
    }
}

/// A complete facial composite.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Composite {
    /// JSON schema reference
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// Unique identifier
    pub id: Uuid,
    /// User-defined name
    pub name: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp
    pub modified_at: DateTime<Utc>,
    /// Canvas configuration
    pub canvas: CanvasConfig,
    /// Layers in render order
    pub layers: Vec<Layer>,
    /// Optional metadata
    pub metadata: CompositeMetadata,
}

impl Composite {
    /// Create a new empty composite.
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        Self {
            schema: Some("https://identikit.dev/schema/v1.0.0".to_string()),
            id: Uuid::new_v4(),
            name,
            created_at: now,
            modified_at: now,
            canvas: CanvasConfig::default(),
            layers: Vec::new(),
            metadata: CompositeMetadata::new(),
        }
    }

    /// Create a composite with custom canvas size.
    pub fn with_canvas(name: String, width: u32, height: u32) -> Self {
        let mut composite = Self::new(name);
        composite.canvas.width = width;
        composite.canvas.height = height;
        composite
    }

    /// Add a layer to the composite.
    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
        self.touch();
    }

    /// Remove a layer by ID.
    pub fn remove_layer(&mut self, layer_id: Uuid) -> Option<Layer> {
        if let Some(index) = self.layers.iter().position(|l| l.id == layer_id) {
            self.touch();
            Some(self.layers.remove(index))
        } else {
            None
        }
    }

    /// Get a mutable reference to a layer by ID.
    pub fn get_layer_mut(&mut self, layer_id: Uuid) -> Option<&mut Layer> {
        self.layers.iter_mut().find(|l| l.id == layer_id)
    }

    /// Reorder layers by providing the new order of IDs.
    pub fn reorder_layers(&mut self, order: &[Uuid]) {
        for (new_index, id) in order.iter().enumerate() {
            if let Some(layer) = self.layers.iter_mut().find(|l| &l.id == id) {
                layer.z_index = new_index as i32;
            }
        }
        self.layers.sort_by_key(|l| l.z_index);
        self.touch();
    }

    /// Update the modified timestamp.
    pub fn touch(&mut self) {
        self.modified_at = Utc::now();
    }
}
