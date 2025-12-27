//! Feature Library Commands
//!
//! Access facial feature assets.

use crate::models::{CategoryInfo, Feature, FeatureCategory, FeatureMetadata, Point2D};

/// Get all feature categories.
#[tauri::command]
pub fn get_feature_categories() -> Vec<CategoryInfo> {
    FeatureCategory::all()
        .into_iter()
        .map(|cat| CategoryInfo {
            id: cat,
            name: cat.display_name().to_string(),
            folder: cat.folder_name().to_string(),
            count: get_feature_count(cat),
        })
        .collect()
}

/// Get features by category.
#[tauri::command]
pub fn get_features_by_category(category: FeatureCategory) -> Vec<Feature> {
    // TODO: Load actual features from the feature library
    // For now, return placeholder features
    get_placeholder_features(category)
}

/// Search features by query.
#[tauri::command]
pub fn search_features(query: String) -> Vec<Feature> {
    let query_lower = query.to_lowercase();

    // Search across all categories
    FeatureCategory::all()
        .into_iter()
        .flat_map(get_placeholder_features)
        .filter(|f| {
            f.name.to_lowercase().contains(&query_lower)
                || f.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
        })
        .collect()
}

/// Get the count of features in a category.
fn get_feature_count(category: FeatureCategory) -> usize {
    // TODO: Count actual features
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
