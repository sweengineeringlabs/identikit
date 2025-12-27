import { useCompositeStore, useUIStore } from '@/stores';
import { ZoomIn, ZoomOut, Maximize } from 'lucide-react';

export function StatusBar() {
  const { composite, selectedLayerId, isModified } = useCompositeStore();
  const { zoom, zoomIn, zoomOut, resetZoom } = useUIStore();

  const selectedLayer = composite?.layers.find((l) => l.id === selectedLayerId);

  return (
    <footer className="h-8 bg-gray-800 text-gray-300 text-xs flex items-center justify-between px-4">
      {/* Left: Document info */}
      <div className="flex items-center gap-4">
        <span>
          {composite?.name || 'No document'}
          {isModified && ' *'}
        </span>
        {composite && (
          <span className="text-gray-500">
            {composite.canvas.width} x {composite.canvas.height}
          </span>
        )}
        <span className="text-gray-500">
          {composite?.layers.length || 0} layers
        </span>
      </div>

      {/* Center: Selection info */}
      <div className="flex items-center">
        {selectedLayer && (
          <span>
            Selected: {selectedLayer.name}
          </span>
        )}
      </div>

      {/* Right: Zoom controls */}
      <div className="flex items-center gap-2">
        <button
          onClick={zoomOut}
          className="p-1 hover:bg-gray-700 rounded"
          title="Zoom Out"
        >
          <ZoomOut size={14} />
        </button>
        <button
          onClick={resetZoom}
          className="px-2 hover:bg-gray-700 rounded"
          title="Reset Zoom"
        >
          {Math.round(zoom * 100)}%
        </button>
        <button
          onClick={zoomIn}
          className="p-1 hover:bg-gray-700 rounded"
          title="Zoom In"
        >
          <ZoomIn size={14} />
        </button>
        <button
          onClick={resetZoom}
          className="p-1 hover:bg-gray-700 rounded"
          title="Fit to Screen"
        >
          <Maximize size={14} />
        </button>
      </div>
    </footer>
  );
}
