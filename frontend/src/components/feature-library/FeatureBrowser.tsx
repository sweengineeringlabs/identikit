import { useEffect } from 'react';
import { Search } from 'lucide-react';
import { useFeatureLibraryStore, useCompositeStore } from '@/stores';
import { FEATURE_CATEGORIES, CATEGORY_INFO } from '@/types';
import { clsx } from 'clsx';

export function FeatureBrowser() {
  const {
    activeCategory,
    setActiveCategory,
    features,
    searchQuery,
    setSearchQuery,
    isLoading,
  } = useFeatureLibraryStore();
  const { addLayer } = useCompositeStore();

  const handleFeatureClick = async (featureId: string, featureName: string) => {
    await addLayer(featureId, featureName);
  };

  return (
    <div className="flex flex-col h-full">
      {/* Search */}
      <div className="p-3 border-b border-gray-100">
        <div className="relative">
          <Search
            size={16}
            className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
          />
          <input
            type="text"
            placeholder="Search features..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="w-full pl-9 pr-3 py-2 text-sm border border-gray-200 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          />
        </div>
      </div>

      {/* Category Tabs */}
      <div className="flex overflow-x-auto border-b border-gray-100 px-2">
        {FEATURE_CATEGORIES.map((cat) => (
          <button
            key={cat}
            onClick={() => setActiveCategory(cat)}
            className={clsx(
              'px-3 py-2 text-xs font-medium whitespace-nowrap transition-colors',
              activeCategory === cat
                ? 'text-blue-600 border-b-2 border-blue-600'
                : 'text-gray-500 hover:text-gray-700'
            )}
          >
            {CATEGORY_INFO[cat].label}
          </button>
        ))}
      </div>

      {/* Feature Grid */}
      <div className="flex-1 overflow-y-auto p-3">
        {isLoading ? (
          <div className="flex items-center justify-center h-32">
            <span className="text-gray-400">Loading...</span>
          </div>
        ) : features.length === 0 ? (
          <div className="flex items-center justify-center h-32">
            <span className="text-gray-400">No features found</span>
          </div>
        ) : (
          <div className="grid grid-cols-3 gap-2">
            {features.map((feature) => (
              <button
                key={feature.id}
                onClick={() => handleFeatureClick(feature.id, feature.name)}
                className="aspect-square rounded-lg border border-gray-200 hover:border-blue-400 hover:shadow-sm transition-all bg-gray-50 flex items-center justify-center group"
                title={feature.name}
              >
                {/* Placeholder for feature thumbnail */}
                <div className="w-12 h-12 rounded bg-gray-200 group-hover:bg-gray-300 transition-colors" />
              </button>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
