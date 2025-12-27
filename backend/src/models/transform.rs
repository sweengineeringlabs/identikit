//! Transform Model
//!
//! Represents position, scale, and rotation transformations.

use serde::{Deserialize, Serialize};

/// 2D transformation data for a layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transform {
    /// X position (pixels from left)
    pub x: f64,
    /// Y position (pixels from top)
    pub y: f64,
    /// Horizontal scale factor (1.0 = 100%)
    pub scale_x: f64,
    /// Vertical scale factor (1.0 = 100%)
    pub scale_y: f64,
    /// Rotation in degrees
    pub rotation: f64,
    /// Horizontal flip
    pub flip_x: bool,
    /// Vertical flip
    pub flip_y: bool,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
        }
    }
}

impl Transform {
    /// Create a new transform with the given position.
    pub fn at(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            ..Default::default()
        }
    }

    /// Create a centered transform for a canvas.
    pub fn centered(canvas_width: u32, canvas_height: u32) -> Self {
        Self {
            x: canvas_width as f64 / 2.0,
            y: canvas_height as f64 / 2.0,
            ..Default::default()
        }
    }
}
