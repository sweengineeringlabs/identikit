//! File I/O Commands
//!
//! Save and load composite files.

use tauri::State;

use crate::models::Composite;

use super::composite::CompositeState;

/// Save the current composite to a file.
#[tauri::command]
pub async fn save_composite(path: String, state: State<'_, CompositeState>) -> Result<(), String> {
    // Extract JSON while holding the lock, then drop it before await
    let json = {
        let current = state.current.lock().map_err(|e| e.to_string())?;
        let composite = current.as_ref().ok_or("No composite loaded")?;
        serde_json::to_string_pretty(composite).map_err(|e| e.to_string())?
    };

    tokio::fs::write(&path, json)
        .await
        .map_err(|e| e.to_string())?;

    tracing::info!("Saved composite to: {}", path);
    Ok(())
}

/// Load a composite from a file.
#[tauri::command]
pub async fn load_composite(
    path: String,
    state: State<'_, CompositeState>,
) -> Result<Composite, String> {
    let content = tokio::fs::read_to_string(&path)
        .await
        .map_err(|e| e.to_string())?;
    let composite: Composite = serde_json::from_str(&content).map_err(|e| e.to_string())?;

    let mut current = state.current.lock().map_err(|e| e.to_string())?;
    *current = Some(composite.clone());

    tracing::info!("Loaded composite from: {}", path);
    Ok(composite)
}
