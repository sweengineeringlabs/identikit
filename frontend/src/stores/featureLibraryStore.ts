import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type { Feature, FeatureCategory, CategoryInfo } from '@/types';

interface FeatureLibraryState {
  categories: CategoryInfo[];
  activeCategory: FeatureCategory;
  features: Feature[];
  searchQuery: string;
  isLoading: boolean;
  error: string | null;

  loadCategories: () => Promise<void>;
  setActiveCategory: (category: FeatureCategory) => void;
  loadFeatures: (category: FeatureCategory) => Promise<void>;
  setSearchQuery: (query: string) => void;
  searchFeatures: (query: string) => Promise<void>;
}

export const useFeatureLibraryStore = create<FeatureLibraryState>((set, get) => ({
  categories: [],
  activeCategory: 'faceShape',
  features: [],
  searchQuery: '',
  isLoading: false,
  error: null,

  loadCategories: async () => {
    set({ isLoading: true, error: null });
    try {
      const categories = await invoke<CategoryInfo[]>('get_feature_categories');
      set({ categories, isLoading: false });
    } catch (e) {
      set({ error: String(e), isLoading: false });
    }
  },

  setActiveCategory: (category) => {
    set({ activeCategory: category });
    get().loadFeatures(category);
  },

  loadFeatures: async (category) => {
    set({ isLoading: true, error: null });
    try {
      const features = await invoke<Feature[]>('get_features_by_category', {
        category,
      });
      set({ features, isLoading: false });
    } catch (e) {
      set({ error: String(e), isLoading: false });
    }
  },

  setSearchQuery: (query) => {
    set({ searchQuery: query });
    if (query.trim()) {
      get().searchFeatures(query);
    } else {
      get().loadFeatures(get().activeCategory);
    }
  },

  searchFeatures: async (query) => {
    set({ isLoading: true, error: null });
    try {
      const features = await invoke<Feature[]>('search_features', { query });
      set({ features, isLoading: false });
    } catch (e) {
      set({ error: String(e), isLoading: false });
    }
  },
}));
