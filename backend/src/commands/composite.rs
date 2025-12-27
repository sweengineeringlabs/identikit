//! Composite Commands
//!
//! CRUD operations for composites.

use std::sync::Mutex;
use tauri::State;
use uuid::Uuid;

use crate::models::{Composite, Layer, Transform};

/// Application state for the current composite.
pub struct CompositeState {
    pub current: Mutex<Option<Composite>>,
}

impl CompositeState {
    pub fn new() -> Self {
        Self {
            current: Mutex::new(None),
        }
    }
}

impl Default for CompositeState {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a new empty composite.
#[tauri::command]
pub fn create_composite(name: String, state: State<CompositeState>) -> Result<Composite, String> {
    let composite = Composite::new(name);
    let mut current = state.current.lock().map_err(|e| e.to_string())?;
    *current = Some(composite.clone());
    tracing::info!("Created new composite: {}", composite.name);
    Ok(composite)
}

/// Get the current composite.
#[tauri::command]
pub fn get_composite(state: State<CompositeState>) -> Result<Option<Composite>, String> {
    let current = state.current.lock().map_err(|e| e.to_string())?;
    Ok(current.clone())
}

/// Add a layer to the current composite.
#[tauri::command]
pub fn add_layer(
    feature_id: String,
    name: String,
    transform: Option<Transform>,
    state: State<CompositeState>,
) -> Result<Layer, String> {
    let mut current = state.current.lock().map_err(|e| e.to_string())?;
    let composite = current.as_mut().ok_or("No composite loaded")?;

    let z_index = composite.layers.len() as i32;
    let mut layer = Layer::new(feature_id.clone(), name, z_index);

    if let Some(t) = transform {
        layer.transform = t;
    } else {
        // Center the layer on the canvas
        layer.transform = Transform::centered(composite.canvas.width, composite.canvas.height);
    }

    composite.add_layer(layer.clone());
    tracing::info!("Added layer: {} ({})", layer.name, feature_id);
    Ok(layer)
}

/// Update a layer's transform.
#[tauri::command]
pub fn update_layer_transform(
    layer_id: String,
    transform: Transform,
    state: State<CompositeState>,
) -> Result<(), String> {
    let layer_uuid = Uuid::parse_str(&layer_id).map_err(|e| e.to_string())?;
    let mut current = state.current.lock().map_err(|e| e.to_string())?;
    let composite = current.as_mut().ok_or("No composite loaded")?;

    if let Some(layer) = composite.get_layer_mut(layer_uuid) {
        layer.transform = transform;
        composite.touch();
        Ok(())
    } else {
        Err("Layer not found".to_string())
    }
}

/// Remove a layer from the composite.
#[tauri::command]
pub fn remove_layer(layer_id: String, state: State<CompositeState>) -> Result<(), String> {
    let layer_uuid = Uuid::parse_str(&layer_id).map_err(|e| e.to_string())?;
    let mut current = state.current.lock().map_err(|e| e.to_string())?;
    let composite = current.as_mut().ok_or("No composite loaded")?;

    composite
        .remove_layer(layer_uuid)
        .ok_or("Layer not found")?;
    tracing::info!("Removed layer: {}", layer_id);
    Ok(())
}

/// Reorder layers by providing the new order of IDs.
#[tauri::command]
pub fn reorder_layers(layer_order: Vec<String>, state: State<CompositeState>) -> Result<(), String> {
    let mut current = state.current.lock().map_err(|e| e.to_string())?;
    let composite = current.as_mut().ok_or("No composite loaded")?;

    let order: Result<Vec<Uuid>, _> = layer_order.iter().map(|id| Uuid::parse_str(id)).collect();
    let order = order.map_err(|e| e.to_string())?;

    composite.reorder_layers(&order);
    tracing::info!("Reordered layers");
    Ok(())
}

/// Set layer visibility.
#[tauri::command]
pub fn set_layer_visibility(
    layer_id: String,
    visible: bool,
    state: State<CompositeState>,
) -> Result<(), String> {
    let layer_uuid = Uuid::parse_str(&layer_id).map_err(|e| e.to_string())?;
    let mut current = state.current.lock().map_err(|e| e.to_string())?;
    let composite = current.as_mut().ok_or("No composite loaded")?;

    if let Some(layer) = composite.get_layer_mut(layer_uuid) {
        layer.visible = visible;
        composite.touch();
        Ok(())
    } else {
        Err("Layer not found".to_string())
    }
}

/// Set layer opacity.
#[tauri::command]
pub fn set_layer_opacity(
    layer_id: String,
    opacity: f64,
    state: State<CompositeState>,
) -> Result<(), String> {
    let layer_uuid = Uuid::parse_str(&layer_id).map_err(|e| e.to_string())?;
    let mut current = state.current.lock().map_err(|e| e.to_string())?;
    let composite = current.as_mut().ok_or("No composite loaded")?;

    if let Some(layer) = composite.get_layer_mut(layer_uuid) {
        layer.opacity = opacity.clamp(0.0, 1.0);
        composite.touch();
        Ok(())
    } else {
        Err("Layer not found".to_string())
    }
}
