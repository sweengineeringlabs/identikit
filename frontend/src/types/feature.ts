/**
 * Categories of facial features.
 */
export type FeatureCategory =
  | 'hair'
  | 'faceShape'
  | 'eyes'
  | 'eyebrows'
  | 'nose'
  | 'mouth'
  | 'chin'
  | 'ears'
  | 'accessories';

/**
 * All feature categories in display order.
 */
export const FEATURE_CATEGORIES: FeatureCategory[] = [
  'faceShape',
  'hair',
  'eyes',
  'eyebrows',
  'nose',
  'mouth',
  'chin',
  'ears',
  'accessories',
];

/**
 * Category display information.
 */
export const CATEGORY_INFO: Record<
  FeatureCategory,
  { label: string; icon: string }
> = {
  hair: { label: 'Hair', icon: 'scissors' },
  faceShape: { label: 'Face', icon: 'user' },
  eyes: { label: 'Eyes', icon: 'eye' },
  eyebrows: { label: 'Brows', icon: 'minus' },
  nose: { label: 'Nose', icon: 'triangle' },
  mouth: { label: 'Mouth', icon: 'smile' },
  chin: { label: 'Chin', icon: 'chevron-down' },
  ears: { label: 'Ears', icon: 'headphones' },
  accessories: { label: 'Extras', icon: 'glasses' },
};

/**
 * A 2D point.
 */
export interface Point2D {
  x: number;
  y: number;
}

/**
 * A color zone that can be customized.
 */
export interface ColorZone {
  id: string;
  name: string;
  defaultColor: string;
}

/**
 * Metadata for a feature asset.
 */
export interface FeatureMetadata {
  width: number;
  height: number;
  anchorPoint: Point2D;
  colorZones: ColorZone[];
}

/**
 * A facial feature asset.
 */
export interface Feature {
  /** Unique identifier */
  id: string;
  /** Display name */
  name: string;
  /** Category */
  category: FeatureCategory;
  /** Path to SVG file */
  svgPath: string;
  /** Path to thumbnail (optional) */
  thumbnailPath?: string;
  /** Searchable tags */
  tags: string[];
  /** Asset metadata */
  metadata: FeatureMetadata;
}

/**
 * Category information for the UI.
 */
export interface CategoryInfo {
  id: FeatureCategory;
  name: string;
  folder: string;
  count: number;
}
