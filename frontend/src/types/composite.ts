import type { Layer } from './layer';

/**
 * Metadata for a composite.
 */
export interface CompositeMetadata {
  /** Format version */
  version: string;
  /** Author/creator name */
  author?: string;
  /** Description */
  description?: string;
  /** Case number for law enforcement */
  caseNumber?: string;
  /** Searchable tags */
  tags: string[];
}

/**
 * Canvas configuration.
 */
export interface CanvasConfig {
  width: number;
  height: number;
  backgroundColor: string;
}

/**
 * A complete facial composite.
 */
export interface Composite {
  /** Unique identifier */
  id: string;
  /** User-defined name */
  name: string;
  /** Creation timestamp (ISO 8601) */
  createdAt: string;
  /** Last modification timestamp (ISO 8601) */
  modifiedAt: string;
  /** Canvas configuration */
  canvas: CanvasConfig;
  /** Layers in render order */
  layers: Layer[];
  /** Optional metadata */
  metadata: CompositeMetadata;
}

/**
 * Default canvas configuration.
 */
export const DEFAULT_CANVAS_CONFIG: CanvasConfig = {
  width: 800,
  height: 1000,
  backgroundColor: '#FFFFFF',
};
