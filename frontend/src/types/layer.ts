import type { Transform } from './transform';

/**
 * A single layer in a facial composite.
 */
export interface Layer {
  /** Unique identifier */
  id: string;
  /** Reference to the feature asset */
  featureId: string;
  /** User-visible name */
  name: string;
  /** Whether the layer is visible */
  visible: boolean;
  /** Whether the layer is locked for editing */
  locked: boolean;
  /** Opacity (0.0 - 1.0) */
  opacity: number;
  /** 2D transformation */
  transform: Transform;
  /** Color overrides for specific zones */
  colorOverrides: Record<string, string>;
  /** Z-index for layer ordering */
  zIndex: number;
}
