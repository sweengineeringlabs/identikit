# ADR-002: Canvas Library Selection

## Status

**Accepted**

## Context

Identikit requires a canvas solution for:
- Rendering facial feature layers (SVG-based)
- Object selection and manipulation (move, scale, rotate)
- Layer compositing with z-ordering
- Serialization for save/load operations
- Export to raster formats (PNG)

## Options Considered

### Option 1: Fabric.js

Mature JavaScript canvas library with object model.

**Pros:**
- Built-in object model (shapes, images, groups)
- Interactive manipulation (select, drag, resize, rotate)
- SVG import/export
- JSON serialization
- Active maintenance (v7.0, Dec 2025)
- Large community, good documentation
- ~96KB gzipped

**Cons:**
- JavaScript-only (no Rust bindings)
- Some learning curve for advanced features

### Option 2: Konva.js

HTML5 Canvas framework for desktop and mobile.

**Pros:**
- Good performance
- React bindings (react-konva)
- Layering support

**Cons:**
- Less mature object model than Fabric.js
- Fewer built-in features
- Smaller community

### Option 3: Paper.js

Vector graphics scripting framework.

**Pros:**
- Excellent for vector graphics
- Scene graph architecture

**Cons:**
- More vector-focused, less object manipulation
- Steeper learning curve
- Less suited for image compositing

### Option 4: PixiJS

2D WebGL renderer.

**Pros:**
- Fastest rendering (WebGL)
- Great for animations

**Cons:**
- Gaming-focused, overkill for this use case
- No built-in manipulation controls
- Would need to build selection/transform UI

### Option 5: Raw Canvas API

Use HTML5 Canvas directly.

**Pros:**
- No dependencies
- Full control

**Cons:**
- Must build entire object model from scratch
- Must implement selection, manipulation, serialization
- Significant development effort

## Decision

**Option 1: Fabric.js**

Fabric.js provides the best balance of features and development speed.

Rationale:
1. Built-in object model matches our layer concept
2. Interactive manipulation out of the box
3. SVG import for facial features
4. JSON serialization aligns with .idkit format
5. Proven in production applications
6. Active development and community

## Implementation

```typescript
// Layer as Fabric.js object
const layer = new fabric.Image(svgElement, {
  left: transform.x,
  top: transform.y,
  scaleX: transform.scaleX,
  scaleY: transform.scaleY,
  angle: transform.rotation,
  flipX: transform.flipX,
  flipY: transform.flipY,
  selectable: !layer.locked,
  opacity: layer.opacity,
});

canvas.add(layer);
```

```typescript
// Serialization
const json = canvas.toJSON();
const dataUrl = canvas.toDataURL({ format: 'png' });
```

## Consequences

### Positive
- Rapid development with built-in features
- Consistent manipulation UX
- Easy serialization for save/load
- SVG support for feature library

### Negative
- Coupled to Fabric.js API
- Must keep up with Fabric.js updates

### Mitigations
- Abstract canvas operations behind interface
- Pin Fabric.js version, test before upgrades

## References

- [Fabric.js](http://fabricjs.com/)
- [Fabric.js GitHub](https://github.com/fabricjs/fabric.js)
- [Konva.js](https://konvajs.org/)
- [Paper.js](http://paperjs.org/)
- [PixiJS](https://pixijs.com/)

---

**Date**: 2025-12-27
**Author**: Engineering Team
