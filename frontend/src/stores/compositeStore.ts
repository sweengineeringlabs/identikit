import { create } from 'zustand';
import { immer } from 'zustand/middleware/immer';
import { invoke } from '@tauri-apps/api/core';
import type { Composite, Layer, Transform } from '@/types';

interface CompositeState {
  composite: Composite | null;
  selectedLayerId: string | null;
  isModified: boolean;
  isLoading: boolean;
  error: string | null;

  // Actions
  createComposite: (name: string) => Promise<void>;
  loadComposite: (path: string) => Promise<void>;
  saveComposite: (path: string) => Promise<void>;
  clearComposite: () => void;

  addLayer: (featureId: string, name: string, transform?: Partial<Transform>) => Promise<void>;
  updateLayerTransform: (layerId: string, transform: Partial<Transform>) => void;
  removeLayer: (layerId: string) => void;
  reorderLayers: (layerOrder: string[]) => void;

  selectLayer: (layerId: string | null) => void;
  toggleLayerVisibility: (layerId: string) => void;
  toggleLayerLock: (layerId: string) => void;
  setLayerOpacity: (layerId: string, opacity: number) => void;

  setError: (error: string | null) => void;
}

export const useCompositeStore = create<CompositeState>()(
  immer((set, get) => ({
    composite: null,
    selectedLayerId: null,
    isModified: false,
    isLoading: false,
    error: null,

    createComposite: async (name) => {
      set({ isLoading: true, error: null });
      try {
        const composite = await invoke<Composite>('create_composite', { name });
        set({ composite, isModified: false, selectedLayerId: null, isLoading: false });
      } catch (e) {
        set({ error: String(e), isLoading: false });
      }
    },

    loadComposite: async (path) => {
      set({ isLoading: true, error: null });
      try {
        const composite = await invoke<Composite>('load_composite', { path });
        set({ composite, isModified: false, selectedLayerId: null, isLoading: false });
      } catch (e) {
        set({ error: String(e), isLoading: false });
      }
    },

    saveComposite: async (path) => {
      set({ isLoading: true, error: null });
      try {
        await invoke('save_composite', { path });
        set({ isModified: false, isLoading: false });
      } catch (e) {
        set({ error: String(e), isLoading: false });
      }
    },

    clearComposite: () => {
      set({ composite: null, selectedLayerId: null, isModified: false });
    },

    addLayer: async (featureId, name, transform) => {
      try {
        const layer = await invoke<Layer>('add_layer', {
          featureId,
          name,
          transform: transform ?? null,
        });

        set((state) => {
          if (state.composite) {
            state.composite.layers.push(layer);
            state.selectedLayerId = layer.id;
            state.isModified = true;
          }
        });
      } catch (e) {
        set({ error: String(e) });
      }
    },

    updateLayerTransform: (layerId, transform) => {
      set((state) => {
        if (state.composite) {
          const layer = state.composite.layers.find((l) => l.id === layerId);
          if (layer) {
            layer.transform = { ...layer.transform, ...transform };
            state.isModified = true;
          }
        }
      });

      // Sync to backend
      const { composite } = get();
      const layer = composite?.layers.find((l) => l.id === layerId);
      if (layer) {
        invoke('update_layer_transform', {
          layerId,
          transform: layer.transform,
        }).catch((e) => set({ error: String(e) }));
      }
    },

    removeLayer: (layerId) => {
      set((state) => {
        if (state.composite) {
          state.composite.layers = state.composite.layers.filter(
            (l) => l.id !== layerId
          );
          if (state.selectedLayerId === layerId) {
            state.selectedLayerId = null;
          }
          state.isModified = true;
        }
      });

      invoke('remove_layer', { layerId }).catch((e) => set({ error: String(e) }));
    },

    reorderLayers: (layerOrder) => {
      set((state) => {
        if (state.composite) {
          const layerMap = new Map(
            state.composite.layers.map((l) => [l.id, l])
          );
          state.composite.layers = layerOrder
            .map((id, index) => {
              const layer = layerMap.get(id);
              if (layer) {
                layer.zIndex = index;
              }
              return layer;
            })
            .filter((l): l is Layer => l !== undefined);
          state.isModified = true;
        }
      });

      invoke('reorder_layers', { layerOrder }).catch((e) =>
        set({ error: String(e) })
      );
    },

    selectLayer: (layerId) => {
      set({ selectedLayerId: layerId });
    },

    toggleLayerVisibility: (layerId) => {
      set((state) => {
        if (state.composite) {
          const layer = state.composite.layers.find((l) => l.id === layerId);
          if (layer) {
            layer.visible = !layer.visible;
            state.isModified = true;
          }
        }
      });

      const { composite } = get();
      const layer = composite?.layers.find((l) => l.id === layerId);
      if (layer) {
        invoke('set_layer_visibility', {
          layerId,
          visible: layer.visible,
        }).catch((e) => set({ error: String(e) }));
      }
    },

    toggleLayerLock: (layerId) => {
      set((state) => {
        if (state.composite) {
          const layer = state.composite.layers.find((l) => l.id === layerId);
          if (layer) {
            layer.locked = !layer.locked;
            state.isModified = true;
          }
        }
      });
    },

    setLayerOpacity: (layerId, opacity) => {
      set((state) => {
        if (state.composite) {
          const layer = state.composite.layers.find((l) => l.id === layerId);
          if (layer) {
            layer.opacity = Math.max(0, Math.min(1, opacity));
            state.isModified = true;
          }
        }
      });

      invoke('set_layer_opacity', { layerId, opacity }).catch((e) =>
        set({ error: String(e) })
      );
    },

    setError: (error) => {
      set({ error });
    },
  }))
);
