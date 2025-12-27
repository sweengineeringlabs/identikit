# ADR-003: State Management Approach

## Status

**Accepted**

## Context

Identikit frontend requires state management for:
- Composite data (layers, transforms, metadata)
- UI state (selected layer, zoom, panel visibility)
- History (undo/redo stacks)
- Feature library (categories, loaded features)

The solution must work well with React and Fabric.js.

## Options Considered

### Option 1: Zustand

Lightweight state management with hooks.

**Pros:**
- Minimal boilerplate (~1KB)
- Simple API with hooks
- No providers/context needed
- Excellent TypeScript support
- Middleware support (persist, devtools)
- Can update state from outside React

**Cons:**
- Less structured than Redux
- No built-in async handling (but easy to add)

### Option 2: Redux Toolkit

Industry standard with toolkit simplification.

**Pros:**
- Well-established patterns
- Redux DevTools
- RTK Query for async
- Large ecosystem

**Cons:**
- More boilerplate than Zustand
- Heavier bundle (~7KB)
- Steeper learning curve
- Overkill for desktop app

### Option 3: MobX

Reactive state management.

**Pros:**
- Automatic reactivity
- Less boilerplate than Redux
- Observable patterns

**Cons:**
- Larger bundle (~16KB)
- Magic can be confusing
- Decorator syntax complexity

### Option 4: Jotai

Atomic state management.

**Pros:**
- Primitive and flexible
- Small bundle
- Good for derived state

**Cons:**
- Less structured for complex state
- Newer, smaller community

### Option 5: React Context + useReducer

Built-in React solution.

**Pros:**
- No dependencies
- Built into React

**Cons:**
- Performance issues with frequent updates
- Prop drilling or many contexts
- No devtools

## Decision

**Option 1: Zustand**

Zustand provides the best simplicity-to-power ratio.

Rationale:
1. Minimal boilerplate for faster development
2. Works outside React (useful for Fabric.js callbacks)
3. TypeScript support excellent
4. Small bundle size suits desktop app
5. Easy to understand and maintain

## Implementation

### Store Structure

```typescript
// stores/compositeStore.ts
interface CompositeStore {
  composite: Composite | null;
  selectedLayerId: string | null;
  isModified: boolean;

  // Actions
  setComposite: (composite: Composite) => void;
  addLayer: (feature: Feature) => void;
  updateLayerTransform: (layerId: string, transform: Partial<Transform>) => void;
  removeLayer: (layerId: string) => void;
  selectLayer: (layerId: string | null) => void;
}

export const useCompositeStore = create<CompositeStore>((set, get) => ({
  composite: null,
  selectedLayerId: null,
  isModified: false,

  addLayer: (feature) => set((state) => ({
    composite: {
      ...state.composite!,
      layers: [...state.composite!.layers, createLayer(feature)],
    },
    isModified: true,
  })),
  // ...
}));
```

```typescript
// stores/historyStore.ts
interface HistoryStore {
  undoStack: HistoryAction[];
  redoStack: HistoryAction[];

  pushAction: (action: HistoryAction) => void;
  undo: () => void;
  redo: () => void;
  canUndo: () => boolean;
  canRedo: () => boolean;
}
```

```typescript
// stores/uiStore.ts
interface UIStore {
  zoom: number;
  mode: 'edit' | 'compare';
  panels: { layers: boolean; adjustment: boolean };

  setZoom: (zoom: number) => void;
  setMode: (mode: 'edit' | 'compare') => void;
  togglePanel: (panel: keyof UIStore['panels']) => void;
}
```

### Usage in Components

```typescript
function LayerPanel() {
  const { composite, selectedLayerId, selectLayer } = useCompositeStore();

  return (
    <ul>
      {composite?.layers.map(layer => (
        <li
          key={layer.id}
          onClick={() => selectLayer(layer.id)}
          className={layer.id === selectedLayerId ? 'selected' : ''}
        >
          {layer.name}
        </li>
      ))}
    </ul>
  );
}
```

### Usage Outside React (Fabric.js)

```typescript
// Can access store directly
const state = useCompositeStore.getState();
const { selectLayer } = state;

canvas.on('selection:created', (e) => {
  const layerId = e.selected[0]?.data?.layerId;
  selectLayer(layerId);
});
```

## Consequences

### Positive
- Simple, readable code
- Fast development iteration
- Easy testing (stores are plain functions)
- Works with Fabric.js callbacks

### Negative
- Less enforced structure than Redux
- Team must follow conventions

### Mitigations
- Define clear store boundaries
- Code review for store changes
- Document conventions

## References

- [Zustand](https://github.com/pmndrs/zustand)
- [Redux Toolkit](https://redux-toolkit.js.org/)
- [MobX](https://mobx.js.org/)
- [Jotai](https://jotai.org/)

---

**Date**: 2025-12-27
**Author**: Engineering Team
