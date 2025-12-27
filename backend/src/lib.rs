//! Identikit Library
//!
//! Core library for the Identikit application.

pub mod ai;
pub mod commands;
pub mod models;
pub mod services;

// Re-exports for convenience
pub use ai::{AiError, AiResult, AiState};
pub use models::*;
