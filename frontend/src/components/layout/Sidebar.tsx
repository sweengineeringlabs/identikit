import { FeatureBrowser } from '@/components/feature-library/FeatureBrowser';

export function Sidebar() {
  return (
    <aside className="w-72 bg-white border-r border-gray-200 flex flex-col">
      <div className="p-3 border-b border-gray-200">
        <h2 className="font-semibold text-gray-700">Feature Library</h2>
      </div>
      <div className="flex-1 overflow-hidden">
        <FeatureBrowser />
      </div>
    </aside>
  );
}
