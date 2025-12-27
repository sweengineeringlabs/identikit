import { create } from 'zustand';

interface PanelVisibility {
  featureLibrary: boolean;
  layers: boolean;
  adjustment: boolean;
}

interface UIState {
  zoom: number;
  panelVisibility: PanelVisibility;
  isDragging: boolean;

  setZoom: (zoom: number) => void;
  zoomIn: () => void;
  zoomOut: () => void;
  resetZoom: () => void;
  togglePanel: (panel: keyof PanelVisibility) => void;
  setIsDragging: (isDragging: boolean) => void;
}

const MIN_ZOOM = 0.25;
const MAX_ZOOM = 4;
const ZOOM_STEP = 0.25;

export const useUIStore = create<UIState>((set) => ({
  zoom: 1,
  panelVisibility: {
    featureLibrary: true,
    layers: true,
    adjustment: true,
  },
  isDragging: false,

  setZoom: (zoom) => {
    set({ zoom: Math.max(MIN_ZOOM, Math.min(MAX_ZOOM, zoom)) });
  },

  zoomIn: () => {
    set((state) => ({
      zoom: Math.min(MAX_ZOOM, state.zoom + ZOOM_STEP),
    }));
  },

  zoomOut: () => {
    set((state) => ({
      zoom: Math.max(MIN_ZOOM, state.zoom - ZOOM_STEP),
    }));
  },

  resetZoom: () => {
    set({ zoom: 1 });
  },

  togglePanel: (panel) => {
    set((state) => ({
      panelVisibility: {
        ...state.panelVisibility,
        [panel]: !state.panelVisibility[panel],
      },
    }));
  },

  setIsDragging: (isDragging) => {
    set({ isDragging });
  },
}));
