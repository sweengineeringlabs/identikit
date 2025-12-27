//! Feature Library Commands
//!
//! Access facial feature assets.

use std::path::PathBuf;
use std::sync::Arc;

use parking_lot::RwLock;
use tauri::State;

use crate::models::{CategoryInfo, Feature, FeatureCategory, FeatureMetadata, Point2D};
use crate::services::{FeatureLibraryConfig, FeatureLibraryService};

/// Shared feature library state.
pub struct FeatureLibraryState {
    pub service: Arc<RwLock<FeatureLibraryService>>,
}

impl FeatureLibraryState {
    /// Create new feature library state.
    pub fn new() -> Self {
        Self {
            service: Arc::new(RwLock::new(FeatureLibraryService::with_default_config())),
        }
    }

    /// Create with custom config.
    pub fn with_config(config: FeatureLibraryConfig) -> Self {
        Self {
            service: Arc::new(RwLock::new(FeatureLibraryService::new(config))),
        }
    }
}

impl Default for FeatureLibraryState {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize the feature library.
#[tauri::command]
pub fn initialize_feature_library(
    root_path: String,
    state: State<'_, FeatureLibraryState>,
) -> Result<(), String> {
    let mut service = state.service.write();
    service.set_root_path(PathBuf::from(root_path));
    service.load()
}

/// Check if feature library is loaded.
#[tauri::command]
pub fn is_feature_library_loaded(state: State<'_, FeatureLibraryState>) -> bool {
    state.service.read().is_loaded()
}

/// Get all feature categories.
#[tauri::command]
pub fn get_feature_categories(state: State<'_, FeatureLibraryState>) -> Vec<CategoryInfo> {
    let service = state.service.read();

    // If library is loaded, use actual counts
    if service.is_loaded() {
        FeatureCategory::all()
            .into_iter()
            .map(|cat| CategoryInfo {
                id: cat,
                name: cat.display_name().to_string(),
                folder: cat.folder_name().to_string(),
                count: service.get_feature_count(cat),
            })
            .collect()
    } else {
        // Return placeholder data
        FeatureCategory::all()
            .into_iter()
            .map(|cat| CategoryInfo {
                id: cat,
                name: cat.display_name().to_string(),
                folder: cat.folder_name().to_string(),
                count: get_placeholder_count(cat),
            })
            .collect()
    }
}

/// Get features by category.
#[tauri::command]
pub fn get_features_by_category(
    category: FeatureCategory,
    state: State<'_, FeatureLibraryState>,
) -> Vec<Feature> {
    let service = state.service.read();

    if service.is_loaded() {
        service.get_features(category)
    } else {
        get_placeholder_features(category)
    }
}

/// Get a specific feature by ID.
#[tauri::command]
pub fn get_feature_by_id(
    id: String,
    state: State<'_, FeatureLibraryState>,
) -> Option<Feature> {
    let service = state.service.read();
    service.get_feature_by_id(&id)
}

/// Search features by query.
#[tauri::command]
pub fn search_features(
    query: String,
    state: State<'_, FeatureLibraryState>,
) -> Vec<Feature> {
    let service = state.service.read();

    if service.is_loaded() {
        service.search(&query)
    } else {
        // Search placeholder features
        let query_lower = query.to_lowercase();
        FeatureCategory::all()
            .into_iter()
            .flat_map(get_placeholder_features)
            .filter(|f| {
                f.name.to_lowercase().contains(&query_lower)
                    || f.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
            })
            .collect()
    }
}

/// Get SVG content for a feature.
#[tauri::command]
pub fn get_feature_svg(
    feature_id: String,
    state: State<'_, FeatureLibraryState>,
) -> Result<String, String> {
    let service = state.service.read();
    service.read_svg(&feature_id)
}

/// Get the count of features in a category (placeholder).
fn get_placeholder_count(category: FeatureCategory) -> usize {
    match category {
        FeatureCategory::Hair => 5,
        FeatureCategory::FaceShape => 5,
        FeatureCategory::Eyes => 5,
        FeatureCategory::Eyebrows => 5,
        FeatureCategory::Nose => 5,
        FeatureCategory::Mouth => 5,
        FeatureCategory::Chin => 5,
        FeatureCategory::Ears => 5,
        FeatureCategory::Accessories => 5,
    }
}

/// Generate placeholder features for a category.
fn get_placeholder_features(category: FeatureCategory) -> Vec<Feature> {
    let folder = category.folder_name();
    let prefix = match category {
        FeatureCategory::Hair => "hair",
        FeatureCategory::FaceShape => "face",
        FeatureCategory::Eyes => "eyes",
        FeatureCategory::Eyebrows => "eyebrows",
        FeatureCategory::Nose => "nose",
        FeatureCategory::Mouth => "mouth",
        FeatureCategory::Chin => "chin",
        FeatureCategory::Ears => "ears",
        FeatureCategory::Accessories => "accessory",
    };

    (1..=5)
        .map(|i| Feature {
            id: format!("{}-{:03}", prefix, i),
            name: format!("{} Style {}", category.display_name(), i),
            category,
            svg_path: format!("/features/{}/{}-{:03}.svg", folder, prefix, i),
            thumbnail_path: None,
            tags: vec![category.display_name().to_lowercase()],
            metadata: FeatureMetadata {
                width: 200.0,
                height: 200.0,
                anchor_point: Point2D { x: 100.0, y: 100.0 },
                color_zones: vec![],
            },
        })
        .collect()
}
