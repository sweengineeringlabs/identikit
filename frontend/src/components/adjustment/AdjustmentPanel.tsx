import { useCompositeStore } from '@/stores';
import { FlipHorizontal, FlipVertical, RotateCcw } from 'lucide-react';

export function AdjustmentPanel() {
  const { composite, selectedLayerId, updateLayerTransform, setLayerOpacity } =
    useCompositeStore();

  const selectedLayer = composite?.layers.find((l) => l.id === selectedLayerId);

  if (!selectedLayer) {
    return (
      <div className="p-4 text-center text-gray-500 text-sm">
        Select a layer to adjust its properties
      </div>
    );
  }

  const handleTransformChange = (key: string, value: number | boolean) => {
    updateLayerTransform(selectedLayerId!, { [key]: value });
  };

  const handleReset = () => {
    updateLayerTransform(selectedLayerId!, {
      scaleX: 1,
      scaleY: 1,
      rotation: 0,
      flipX: false,
      flipY: false,
    });
  };

  return (
    <div className="p-4 space-y-4">
      {/* Layer Name */}
      <div className="font-medium text-sm text-gray-700 truncate">
        {selectedLayer.name}
      </div>

      {/* Position */}
      <div className="space-y-2">
        <label className="text-xs font-medium text-gray-500 uppercase">
          Position
        </label>
        <div className="grid grid-cols-2 gap-2">
          <div>
            <label className="text-xs text-gray-400">X</label>
            <input
              type="number"
              value={Math.round(selectedLayer.transform.x)}
              onChange={(e) =>
                handleTransformChange('x', parseFloat(e.target.value) || 0)
              }
              className="w-full px-2 py-1 text-sm border border-gray-200 rounded"
            />
          </div>
          <div>
            <label className="text-xs text-gray-400">Y</label>
            <input
              type="number"
              value={Math.round(selectedLayer.transform.y)}
              onChange={(e) =>
                handleTransformChange('y', parseFloat(e.target.value) || 0)
              }
              className="w-full px-2 py-1 text-sm border border-gray-200 rounded"
            />
          </div>
        </div>
      </div>

      {/* Scale */}
      <div className="space-y-2">
        <label className="text-xs font-medium text-gray-500 uppercase">
          Scale
        </label>
        <div className="grid grid-cols-2 gap-2">
          <div>
            <label className="text-xs text-gray-400">W</label>
            <input
              type="number"
              step="0.1"
              min="0.1"
              max="5"
              value={selectedLayer.transform.scaleX.toFixed(1)}
              onChange={(e) =>
                handleTransformChange(
                  'scaleX',
                  parseFloat(e.target.value) || 1
                )
              }
              className="w-full px-2 py-1 text-sm border border-gray-200 rounded"
            />
          </div>
          <div>
            <label className="text-xs text-gray-400">H</label>
            <input
              type="number"
              step="0.1"
              min="0.1"
              max="5"
              value={selectedLayer.transform.scaleY.toFixed(1)}
              onChange={(e) =>
                handleTransformChange(
                  'scaleY',
                  parseFloat(e.target.value) || 1
                )
              }
              className="w-full px-2 py-1 text-sm border border-gray-200 rounded"
            />
          </div>
        </div>
      </div>

      {/* Rotation */}
      <div className="space-y-2">
        <label className="text-xs font-medium text-gray-500 uppercase">
          Rotation
        </label>
        <div className="flex items-center gap-2">
          <input
            type="range"
            min="-180"
            max="180"
            value={selectedLayer.transform.rotation}
            onChange={(e) =>
              handleTransformChange('rotation', parseFloat(e.target.value))
            }
            className="flex-1"
          />
          <span className="text-sm text-gray-600 w-12 text-right">
            {Math.round(selectedLayer.transform.rotation)}°
          </span>
        </div>
      </div>

      {/* Opacity */}
      <div className="space-y-2">
        <label className="text-xs font-medium text-gray-500 uppercase">
          Opacity
        </label>
        <div className="flex items-center gap-2">
          <input
            type="range"
            min="0"
            max="100"
            value={selectedLayer.opacity * 100}
            onChange={(e) =>
              setLayerOpacity(selectedLayerId!, parseFloat(e.target.value) / 100)
            }
            className="flex-1"
          />
          <span className="text-sm text-gray-600 w-12 text-right">
            {Math.round(selectedLayer.opacity * 100)}%
          </span>
        </div>
      </div>

      {/* Flip & Reset */}
      <div className="flex items-center gap-2 pt-2">
        <button
          onClick={() =>
            handleTransformChange('flipX', !selectedLayer.transform.flipX)
          }
          className={`flex-1 flex items-center justify-center gap-1 px-3 py-2 text-sm rounded border ${
            selectedLayer.transform.flipX
              ? 'bg-blue-50 border-blue-200 text-blue-700'
              : 'bg-gray-50 border-gray-200 text-gray-700 hover:bg-gray-100'
          }`}
        >
          <FlipHorizontal size={14} />
          Flip H
        </button>
        <button
          onClick={() =>
            handleTransformChange('flipY', !selectedLayer.transform.flipY)
          }
          className={`flex-1 flex items-center justify-center gap-1 px-3 py-2 text-sm rounded border ${
            selectedLayer.transform.flipY
              ? 'bg-blue-50 border-blue-200 text-blue-700'
              : 'bg-gray-50 border-gray-200 text-gray-700 hover:bg-gray-100'
          }`}
        >
          <FlipVertical size={14} />
          Flip V
        </button>
        <button
          onClick={handleReset}
          className="flex items-center justify-center p-2 text-gray-500 hover:text-gray-700 hover:bg-gray-100 rounded border border-gray-200"
          title="Reset transforms"
        >
          <RotateCcw size={14} />
        </button>
      </div>
    </div>
  );
}
