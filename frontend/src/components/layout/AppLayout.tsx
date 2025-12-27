import { useEffect } from 'react';
import { Sidebar } from './Sidebar';
import { RightPanel } from './RightPanel';
import { MainToolbar } from '@/components/toolbar/MainToolbar';
import { CompositeCanvas } from '@/components/canvas/CompositeCanvas';
import { StatusBar } from './StatusBar';
import { useCompositeStore, useFeatureLibraryStore, useUIStore } from '@/stores';

export function AppLayout() {
  const { composite, createComposite } = useCompositeStore();
  const { loadCategories, loadFeatures, activeCategory } = useFeatureLibraryStore();
  const { panelVisibility } = useUIStore();

  // Initialize on mount
  useEffect(() => {
    // Load feature categories
    loadCategories();
    loadFeatures(activeCategory);

    // Create a default composite if none exists
    if (!composite) {
      createComposite('Untitled Composite');
    }
  }, []);

  return (
    <div className="flex flex-col h-screen bg-gray-50">
      {/* Top Toolbar */}
      <MainToolbar />

      {/* Main Content */}
      <div className="flex flex-1 overflow-hidden">
        {/* Left Sidebar - Feature Library */}
        {panelVisibility.featureLibrary && <Sidebar />}

        {/* Center - Canvas */}
        <main className="flex-1 flex items-center justify-center bg-gray-100 overflow-auto p-4">
          <CompositeCanvas />
        </main>

        {/* Right Panel - Layers & Adjustment */}
        {(panelVisibility.layers || panelVisibility.adjustment) && (
          <RightPanel />
        )}
      </div>

      {/* Bottom Status Bar */}
      <StatusBar />
    </div>
  );
}
