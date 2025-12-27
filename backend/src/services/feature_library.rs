//! Feature Library Service
//!
//! Load and manage facial feature assets from the filesystem.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use parking_lot::RwLock;
use walkdir::WalkDir;

use crate::models::{ColorZone, Feature, FeatureCategory, FeatureMetadata, Point2D};

/// Feature library configuration.
#[derive(Debug, Clone)]
pub struct FeatureLibraryConfig {
    /// Root directory containing feature assets.
    pub root_path: PathBuf,
    /// Whether to generate thumbnails on load.
    pub generate_thumbnails: bool,
    /// Thumbnail size in pixels.
    pub thumbnail_size: u32,
}

impl Default for FeatureLibraryConfig {
    fn default() -> Self {
        Self {
            root_path: PathBuf::from("features"),
            generate_thumbnails: true,
            thumbnail_size: 64,
        }
    }
}

/// Feature library service for loading and managing feature assets.
pub struct FeatureLibraryService {
    config: FeatureLibraryConfig,
    /// Cached features by category.
    features: Arc<RwLock<HashMap<FeatureCategory, Vec<Feature>>>>,
    /// Flag indicating if library has been loaded.
    loaded: Arc<RwLock<bool>>,
}

impl FeatureLibraryService {
    /// Create a new feature library service.
    pub fn new(config: FeatureLibraryConfig) -> Self {
        Self {
            config,
            features: Arc::new(RwLock::new(HashMap::new())),
            loaded: Arc::new(RwLock::new(false)),
        }
    }

    /// Create with default configuration.
    pub fn with_default_config() -> Self {
        Self::new(FeatureLibraryConfig::default())
    }

    /// Set the root path for the feature library.
    pub fn set_root_path(&mut self, path: PathBuf) {
        self.config.root_path = path;
        *self.loaded.write() = false;
    }

    /// Load all features from the filesystem.
    pub fn load(&self) -> Result<(), String> {
        let root = &self.config.root_path;

        if !root.exists() {
            return Err(format!("Feature library path does not exist: {:?}", root));
        }

        let mut features_map: HashMap<FeatureCategory, Vec<Feature>> = HashMap::new();

        // Initialize empty vectors for each category
        for category in FeatureCategory::all() {
            features_map.insert(category, Vec::new());
        }

        // Walk through each category folder
        for category in FeatureCategory::all() {
            let category_path = root.join(category.folder_name());

            if !category_path.exists() {
                tracing::warn!("Category folder does not exist: {:?}", category_path);
                continue;
            }

            let features = self.load_category(&category_path, category)?;
            features_map.insert(category, features);
        }

        // Update cache
        *self.features.write() = features_map;
        *self.loaded.write() = true;

        tracing::info!("Feature library loaded from {:?}", root);
        Ok(())
    }

    /// Load features from a category folder.
    fn load_category(&self, path: &Path, category: FeatureCategory) -> Result<Vec<Feature>, String> {
        let mut features = Vec::new();

        for entry in WalkDir::new(path)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let entry_path = entry.path();

            // Only process SVG files
            if entry_path.extension().map_or(false, |ext| ext == "svg") {
                match self.load_feature(entry_path, category) {
                    Ok(feature) => features.push(feature),
                    Err(e) => {
                        tracing::warn!("Failed to load feature {:?}: {}", entry_path, e);
                    }
                }
            }
        }

        // Sort features by name
        features.sort_by(|a, b| a.name.cmp(&b.name));

        tracing::debug!("Loaded {} features for category {:?}", features.len(), category);
        Ok(features)
    }

    /// Load a single feature from an SVG file.
    fn load_feature(&self, svg_path: &Path, category: FeatureCategory) -> Result<Feature, String> {
        let file_name = svg_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or("Invalid file name")?;

        // Generate feature ID from file name
        let id = format!("{}-{}", category.folder_name(), file_name);

        // Generate display name from file name
        let name = file_name
            .replace('-', " ")
            .replace('_', " ")
            .split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        // Read SVG content to extract metadata
        let svg_content = fs::read_to_string(svg_path)
            .map_err(|e| format!("Failed to read SVG: {}", e))?;

        let metadata = self.extract_svg_metadata(&svg_content)?;

        // Check for sidecar metadata JSON file
        let metadata_path = svg_path.with_extension("json");
        let metadata = if metadata_path.exists() {
            self.load_feature_metadata(&metadata_path).unwrap_or(metadata)
        } else {
            metadata
        };

        // Generate tags from name and category
        let mut tags: Vec<String> = name
            .to_lowercase()
            .split_whitespace()
            .map(String::from)
            .collect();
        tags.push(category.display_name().to_lowercase());

        // Check for thumbnail
        let thumbnail_path = self.find_thumbnail(svg_path);

        Ok(Feature {
            id,
            name,
            category,
            svg_path: svg_path.to_string_lossy().to_string(),
            thumbnail_path,
            tags,
            metadata,
        })
    }

    /// Extract metadata from SVG content.
    fn extract_svg_metadata(&self, svg_content: &str) -> Result<FeatureMetadata, String> {
        // Parse SVG to get dimensions
        let options = usvg::Options::default();
        let tree = usvg::Tree::from_str(svg_content, &options)
            .map_err(|e| format!("Failed to parse SVG: {}", e))?;

        let size = tree.size();
        let width = size.width() as f64;
        let height = size.height() as f64;

        // Default anchor point is center
        let anchor_point = Point2D {
            x: width / 2.0,
            y: height / 2.0,
        };

        // Extract color zones from SVG (look for elements with data-color-zone attribute)
        let color_zones = self.extract_color_zones(svg_content);

        Ok(FeatureMetadata {
            width,
            height,
            anchor_point,
            color_zones,
        })
    }

    /// Extract color zones from SVG content.
    fn extract_color_zones(&self, svg_content: &str) -> Vec<ColorZone> {
        let mut zones = Vec::new();

        // Simple regex-like search for data-color-zone attributes
        // In a real implementation, you'd use an XML parser
        for line in svg_content.lines() {
            if line.contains("data-color-zone") {
                // Extract zone info (simplified)
                if let Some(start) = line.find("data-color-zone=\"") {
                    let rest = &line[start + 17..];
                    if let Some(end) = rest.find('"') {
                        let zone_id = &rest[..end];

                        // Try to extract fill color
                        let default_color = if let Some(fill_start) = line.find("fill=\"") {
                            let fill_rest = &line[fill_start + 6..];
                            if let Some(fill_end) = fill_rest.find('"') {
                                fill_rest[..fill_end].to_string()
                            } else {
                                "#000000".to_string()
                            }
                        } else {
                            "#000000".to_string()
                        };

                        zones.push(ColorZone {
                            id: zone_id.to_string(),
                            name: zone_id.replace('-', " ").replace('_', " "),
                            default_color,
                        });
                    }
                }
            }
        }

        zones
    }

    /// Load feature metadata from a sidecar JSON file.
    fn load_feature_metadata(&self, path: &Path) -> Result<FeatureMetadata, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read metadata: {}", e))?;

        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse metadata: {}", e))
    }

    /// Find thumbnail for an SVG file.
    fn find_thumbnail(&self, svg_path: &Path) -> Option<String> {
        let parent = svg_path.parent()?;
        let stem = svg_path.file_stem()?.to_str()?;

        // Check for various thumbnail formats
        for ext in &["png", "jpg", "jpeg", "webp"] {
            let thumb_name = format!("{}.thumb.{}", stem, ext);
            let thumb_path = parent.join(&thumb_name);
            if thumb_path.exists() {
                return Some(thumb_path.to_string_lossy().to_string());
            }

            // Also check thumbnails subfolder
            let thumb_subfolder = parent.join("thumbnails").join(format!("{}.{}", stem, ext));
            if thumb_subfolder.exists() {
                return Some(thumb_subfolder.to_string_lossy().to_string());
            }
        }

        None
    }

    /// Check if the library has been loaded.
    pub fn is_loaded(&self) -> bool {
        *self.loaded.read()
    }

    /// Get all features for a category.
    pub fn get_features(&self, category: FeatureCategory) -> Vec<Feature> {
        self.features
            .read()
            .get(&category)
            .cloned()
            .unwrap_or_default()
    }

    /// Get all features across all categories.
    pub fn get_all_features(&self) -> Vec<Feature> {
        self.features
            .read()
            .values()
            .flatten()
            .cloned()
            .collect()
    }

    /// Get feature count for a category.
    pub fn get_feature_count(&self, category: FeatureCategory) -> usize {
        self.features
            .read()
            .get(&category)
            .map_or(0, |v| v.len())
    }

    /// Get a feature by ID.
    pub fn get_feature_by_id(&self, id: &str) -> Option<Feature> {
        self.features
            .read()
            .values()
            .flatten()
            .find(|f| f.id == id)
            .cloned()
    }

    /// Search features by query string.
    pub fn search(&self, query: &str) -> Vec<Feature> {
        let query_lower = query.to_lowercase();

        self.features
            .read()
            .values()
            .flatten()
            .filter(|f| {
                f.name.to_lowercase().contains(&query_lower)
                    || f.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
            })
            .cloned()
            .collect()
    }

    /// Read raw SVG content for a feature.
    pub fn read_svg(&self, feature_id: &str) -> Result<String, String> {
        let feature = self
            .get_feature_by_id(feature_id)
            .ok_or_else(|| format!("Feature not found: {}", feature_id))?;

        fs::read_to_string(&feature.svg_path)
            .map_err(|e| format!("Failed to read SVG: {}", e))
    }
}

impl Default for FeatureLibraryService {
    fn default() -> Self {
        Self::with_default_config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_svg() -> String {
        String::from(r##"<?xml version="1.0" encoding="UTF-8"?><svg width="100" height="100" xmlns="http://www.w3.org/2000/svg"><rect width="100" height="100" fill="#FFFFFF"/><path d="M10 10 L90 90" stroke="#000000" fill="#FF0000"/></svg>"##)
    }

    #[test]
    fn test_load_feature_library() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        // Create category folders with test SVGs
        let hair_dir = root.join("hair");
        fs::create_dir_all(&hair_dir).unwrap();
        fs::write(hair_dir.join("style-001.svg"), create_test_svg()).unwrap();
        fs::write(hair_dir.join("style-002.svg"), create_test_svg()).unwrap();

        let eyes_dir = root.join("eyes");
        fs::create_dir_all(&eyes_dir).unwrap();
        fs::write(eyes_dir.join("round.svg"), create_test_svg()).unwrap();

        let config = FeatureLibraryConfig {
            root_path: root.to_path_buf(),
            generate_thumbnails: false,
            thumbnail_size: 64,
        };

        let service = FeatureLibraryService::new(config);
        service.load().unwrap();

        assert!(service.is_loaded());
        assert_eq!(service.get_feature_count(FeatureCategory::Hair), 2);
        assert_eq!(service.get_feature_count(FeatureCategory::Eyes), 1);
    }

    #[test]
    fn test_search_features() {
        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        let hair_dir = root.join("hair");
        fs::create_dir_all(&hair_dir).unwrap();
        fs::write(hair_dir.join("curly-long.svg"), create_test_svg()).unwrap();
        fs::write(hair_dir.join("straight-short.svg"), create_test_svg()).unwrap();

        let config = FeatureLibraryConfig {
            root_path: root.to_path_buf(),
            generate_thumbnails: false,
            thumbnail_size: 64,
        };

        let service = FeatureLibraryService::new(config);
        service.load().unwrap();

        let results = service.search("curly");
        assert_eq!(results.len(), 1);
        assert!(results[0].name.to_lowercase().contains("curly"));
    }

    #[test]
    fn test_extract_svg_metadata() {
        let service = FeatureLibraryService::with_default_config();
        let svg = create_test_svg();

        let metadata = service.extract_svg_metadata(&svg).unwrap();
        assert_eq!(metadata.width, 100.0);
        assert_eq!(metadata.height, 100.0);
        assert_eq!(metadata.anchor_point.x, 50.0);
        assert_eq!(metadata.anchor_point.y, 50.0);
    }
}
