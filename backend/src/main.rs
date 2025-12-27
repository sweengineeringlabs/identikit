//! Identikit - Digital Facial Composite Platform
//!
//! Main entry point for the Tauri application.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
        .manage(commands::composite::CompositeState::new())
        .manage(commands::history::HistoryState::new())
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
            commands::export::export_pdf,
            // File I/O commands
            commands::file_io::save_composite,
            commands::file_io::load_composite,
            // Feature library commands
            commands::feature_library::get_feature_categories,
            commands::feature_library::get_features_by_category,
            commands::feature_library::search_features,
            // History commands
            commands::history::undo,
            commands::history::redo,
            commands::history::can_undo,
            commands::history::can_redo,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
