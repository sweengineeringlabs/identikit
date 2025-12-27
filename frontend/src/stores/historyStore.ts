import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

interface HistoryAction {
  id: string;
  actionType: string;
  description: string;
  timestamp: string;
}

interface HistoryState {
  canUndo: boolean;
  canRedo: boolean;
  lastAction: HistoryAction | null;

  undo: () => Promise<void>;
  redo: () => Promise<void>;
  checkState: () => Promise<void>;
}

export const useHistoryStore = create<HistoryState>((set) => ({
  canUndo: false,
  canRedo: false,
  lastAction: null,

  undo: async () => {
    try {
      const action = await invoke<HistoryAction | null>('undo');
      if (action) {
        set({ lastAction: action });
      }
      // Refresh state
      const canUndo = await invoke<boolean>('can_undo');
      const canRedo = await invoke<boolean>('can_redo');
      set({ canUndo, canRedo });
    } catch (e) {
      console.error('Undo failed:', e);
    }
  },

  redo: async () => {
    try {
      const action = await invoke<HistoryAction | null>('redo');
      if (action) {
        set({ lastAction: action });
      }
      // Refresh state
      const canUndo = await invoke<boolean>('can_undo');
      const canRedo = await invoke<boolean>('can_redo');
      set({ canUndo, canRedo });
    } catch (e) {
      console.error('Redo failed:', e);
    }
  },

  checkState: async () => {
    try {
      const canUndo = await invoke<boolean>('can_undo');
      const canRedo = await invoke<boolean>('can_redo');
      set({ canUndo, canRedo });
    } catch (e) {
      console.error('Check history state failed:', e);
    }
  },
}));
