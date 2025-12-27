import { LayerManager } from '@/components/canvas/LayerManager';
import { AdjustmentPanel } from '@/components/adjustment/AdjustmentPanel';
import { useUIStore } from '@/stores';

export function RightPanel() {
  const { panelVisibility } = useUIStore();

  return (
    <aside className="w-72 bg-white border-l border-gray-200 flex flex-col">
      {panelVisibility.adjustment && (
        <div className="border-b border-gray-200">
          <div className="p-3 border-b border-gray-100">
            <h2 className="font-semibold text-gray-700">Adjustments</h2>
          </div>
          <AdjustmentPanel />
        </div>
      )}

      {panelVisibility.layers && (
        <div className="flex-1 overflow-hidden flex flex-col">
          <div className="p-3 border-b border-gray-200">
            <h2 className="font-semibold text-gray-700">Layers</h2>
          </div>
          <div className="flex-1 overflow-auto">
            <LayerManager />
          </div>
        </div>
      )}
    </aside>
  );
}
