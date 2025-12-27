import { Eye, EyeOff, Lock, Unlock, Trash2, GripVertical } from 'lucide-react';
import { useCompositeStore } from '@/stores';
import { clsx } from 'clsx';

export function LayerManager() {
  const {
    composite,
    selectedLayerId,
    selectLayer,
    toggleLayerVisibility,
    toggleLayerLock,
    removeLayer,
  } = useCompositeStore();

  if (!composite || composite.layers.length === 0) {
    return (
      <div className="p-4 text-center text-gray-500 text-sm">
        No layers yet. Add features from the library.
      </div>
    );
  }

  // Sort layers by z-index (highest first for display)
  const sortedLayers = [...composite.layers].sort((a, b) => b.zIndex - a.zIndex);

  return (
    <div className="p-2">
      {sortedLayers.map((layer) => (
        <div
          key={layer.id}
          className={clsx(
            'flex items-center gap-2 p-2 rounded cursor-pointer hover:bg-gray-100',
            selectedLayerId === layer.id && 'bg-blue-50 border border-blue-200'
          )}
          onClick={() => selectLayer(layer.id)}
        >
          {/* Drag handle */}
          <GripVertical
            size={14}
            className="text-gray-400 cursor-grab"
          />

          {/* Layer thumbnail placeholder */}
          <div
            className="w-8 h-8 rounded border border-gray-200 flex-shrink-0"
            style={{
              backgroundColor: layer.visible ? '#f0f0f0' : '#e0e0e0',
              opacity: layer.opacity,
            }}
          />

          {/* Layer name */}
          <span
            className={clsx(
              'flex-1 text-sm truncate',
              !layer.visible && 'text-gray-400'
            )}
          >
            {layer.name}
          </span>

          {/* Actions */}
          <div className="flex items-center gap-1">
            {/* Visibility toggle */}
            <button
              onClick={(e) => {
                e.stopPropagation();
                toggleLayerVisibility(layer.id);
              }}
              className="p-1 hover:bg-gray-200 rounded"
              title={layer.visible ? 'Hide layer' : 'Show layer'}
            >
              {layer.visible ? (
                <Eye size={14} className="text-gray-600" />
              ) : (
                <EyeOff size={14} className="text-gray-400" />
              )}
            </button>

            {/* Lock toggle */}
            <button
              onClick={(e) => {
                e.stopPropagation();
                toggleLayerLock(layer.id);
              }}
              className="p-1 hover:bg-gray-200 rounded"
              title={layer.locked ? 'Unlock layer' : 'Lock layer'}
            >
              {layer.locked ? (
                <Lock size={14} className="text-gray-600" />
              ) : (
                <Unlock size={14} className="text-gray-400" />
              )}
            </button>

            {/* Delete */}
            <button
              onClick={(e) => {
                e.stopPropagation();
                removeLayer(layer.id);
              }}
              className="p-1 hover:bg-red-100 rounded text-gray-400 hover:text-red-500"
              title="Delete layer"
            >
              <Trash2 size={14} />
            </button>
          </div>
        </div>
      ))}
    </div>
  );
}
