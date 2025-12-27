//! Identikit - Digital Facial Composite Platform
//!
//! Main entry point for the Tauri application.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ai;
mod commands;
mod models;
mod services;

use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Initialize logging with tracing
fn init_logging() {
    let subscriber = tracing_subscriber::registry()
        .with(EnvFilter::new("debug"))
        .with(fmt::layer().with_ansi(true));

    tracing::subscriber::set_global_default(subscriber).ok();
    tracing::info!("Identikit logging initialized");
}

fn main() {
    init_logging();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        // State management
        .manage(commands::composite::CompositeState::new())
        .manage(commands::history::HistoryState::new())
        .manage(commands::feature_library::FeatureLibraryState::new())
        .manage(ai::AiState::new())
        .invoke_handler(tauri::generate_handler![
            // Composite commands
            commands::composite::create_composite,
            commands::composite::get_composite,
            commands::composite::add_layer,
            commands::composite::update_layer_transform,
            commands::composite::remove_layer,
            commands::composite::reorder_layers,
            commands::composite::set_layer_visibility,
            commands::composite::set_layer_opacity,
            // Export commands
            commands::export::export_png,
            commands::export::export_png_advanced,
            commands::export::export_pdf,
            commands::export::export_pdf_advanced,
            commands::export::get_composite_preview,
            // File I/O commands
            commands::file_io::save_composite,
            commands::file_io::load_composite,
            // Feature library commands
            commands::feature_library::initialize_feature_library,
            commands::feature_library::is_feature_library_loaded,
            commands::feature_library::get_feature_categories,
            commands::feature_library::get_features_by_category,
            commands::feature_library::get_feature_by_id,
            commands::feature_library::search_features,
            commands::feature_library::get_feature_svg,
            // History commands
            commands::history::undo,
            commands::history::redo,
            commands::history::can_undo,
            commands::history::can_redo,
            // AI commands
            commands::ai::initialize_ai,
            commands::ai::is_ai_ready,
            commands::ai::get_ai_status,
            commands::ai::detect_faces,
            commands::ai::compare_faces,
            commands::ai::compare_to_composite,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
