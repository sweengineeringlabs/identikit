//! History Commands
//!
//! Undo/redo functionality.

use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use tauri::State;

/// A single history action.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryAction {
    pub id: String,
    pub action_type: String,
    pub description: String,
    pub timestamp: String,
    /// Serialized state before the action
    pub state_before: String,
}

/// Application state for history management.
pub struct HistoryState {
    pub undo_stack: Mutex<Vec<HistoryAction>>,
    pub redo_stack: Mutex<Vec<HistoryAction>>,
}

impl HistoryState {
    pub fn new() -> Self {
        Self {
            undo_stack: Mutex::new(Vec::new()),
            redo_stack: Mutex::new(Vec::new()),
        }
    }
}

impl Default for HistoryState {
    fn default() -> Self {
        Self::new()
    }
}

/// Undo the last action.
#[tauri::command]
pub fn undo(state: State<HistoryState>) -> Result<Option<HistoryAction>, String> {
    let mut undo_stack = state.undo_stack.lock().map_err(|e| e.to_string())?;
    let mut redo_stack = state.redo_stack.lock().map_err(|e| e.to_string())?;

    if let Some(action) = undo_stack.pop() {
        tracing::info!("Undo: {}", action.description);
        redo_stack.push(action.clone());
        Ok(Some(action))
    } else {
        Ok(None)
    }
}

/// Redo the last undone action.
#[tauri::command]
pub fn redo(state: State<HistoryState>) -> Result<Option<HistoryAction>, String> {
    let mut undo_stack = state.undo_stack.lock().map_err(|e| e.to_string())?;
    let mut redo_stack = state.redo_stack.lock().map_err(|e| e.to_string())?;

    if let Some(action) = redo_stack.pop() {
        tracing::info!("Redo: {}", action.description);
        undo_stack.push(action.clone());
        Ok(Some(action))
    } else {
        Ok(None)
    }
}

/// Check if undo is available.
#[tauri::command]
pub fn can_undo(state: State<HistoryState>) -> Result<bool, String> {
    let undo_stack = state.undo_stack.lock().map_err(|e| e.to_string())?;
    Ok(!undo_stack.is_empty())
}

/// Check if redo is available.
#[tauri::command]
pub fn can_redo(state: State<HistoryState>) -> Result<bool, String> {
    let redo_stack = state.redo_stack.lock().map_err(|e| e.to_string())?;
    Ok(!redo_stack.is_empty())
}
