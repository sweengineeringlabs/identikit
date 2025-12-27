/**
 * 2D transformation for a layer.
 */
export interface Transform {
  /** X position (pixels from left) */
  x: number;
  /** Y position (pixels from top) */
  y: number;
  /** Horizontal scale factor (1.0 = 100%) */
  scaleX: number;
  /** Vertical scale factor (1.0 = 100%) */
  scaleY: number;
  /** Rotation in degrees */
  rotation: number;
  /** Horizontal flip */
  flipX: boolean;
  /** Vertical flip */
  flipY: boolean;
}

/**
 * Create a default transform.
 */
export function createDefaultTransform(): Transform {
  return {
    x: 0,
    y: 0,
    scaleX: 1,
    scaleY: 1,
    rotation: 0,
    flipX: false,
    flipY: false,
  };
}

/**
 * Create a centered transform for a canvas.
 */
export function createCenteredTransform(
  canvasWidth: number,
  canvasHeight: number
): Transform {
  return {
    ...createDefaultTransform(),
    x: canvasWidth / 2,
    y: canvasHeight / 2,
  };
}
