import {
  FileText,
  FolderOpen,
  Save,
  Download,
  Undo2,
  Redo2,
  Settings,
} from 'lucide-react';
import { open, save } from '@tauri-apps/plugin-dialog';
import { useCompositeStore, useHistoryStore } from '@/stores';

export function MainToolbar() {
  const {
    composite,
    isModified,
    createComposite,
    saveComposite,
    loadComposite,
  } = useCompositeStore();
  const { canUndo, canRedo, undo, redo } = useHistoryStore();

  const handleNew = async () => {
    if (isModified) {
      // TODO: Add confirmation dialog
    }
    await createComposite('Untitled Composite');
  };

  const handleOpen = async () => {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: 'Identikit Composite',
          extensions: ['idkit'],
        },
      ],
    });

    if (selected) {
      await loadComposite(selected as string);
    }
  };

  const handleSave = async () => {
    const path = await save({
      filters: [
        {
          name: 'Identikit Composite',
          extensions: ['idkit'],
        },
      ],
      defaultPath: `${composite?.name || 'composite'}.idkit`,
    });

    if (path) {
      await saveComposite(path);
    }
  };

  const handleExport = async () => {
    // TODO: Implement export dialog
    console.log('Export clicked');
  };

  return (
    <header className="h-12 bg-white border-b border-gray-200 flex items-center justify-between px-4">
      {/* Left: File operations */}
      <div className="flex items-center gap-1">
        <ToolbarButton icon={FileText} label="New" onClick={handleNew} />
        <ToolbarButton icon={FolderOpen} label="Open" onClick={handleOpen} />
        <ToolbarButton
          icon={Save}
          label="Save"
          onClick={handleSave}
          disabled={!composite}
        />
        <div className="w-px h-6 bg-gray-200 mx-2" />
        <ToolbarButton
          icon={Download}
          label="Export"
          onClick={handleExport}
          disabled={!composite}
        />
      </div>

      {/* Center: Document name */}
      <div className="flex items-center">
        <span className="text-sm text-gray-600">
          {composite?.name || 'No document'}
          {isModified && <span className="text-orange-500 ml-1">*</span>}
        </span>
      </div>

      {/* Right: History & Settings */}
      <div className="flex items-center gap-1">
        <ToolbarButton
          icon={Undo2}
          label="Undo"
          onClick={undo}
          disabled={!canUndo}
        />
        <ToolbarButton
          icon={Redo2}
          label="Redo"
          onClick={redo}
          disabled={!canRedo}
        />
        <div className="w-px h-6 bg-gray-200 mx-2" />
        <ToolbarButton icon={Settings} label="Settings" onClick={() => {}} />
      </div>
    </header>
  );
}

interface ToolbarButtonProps {
  icon: React.ComponentType<{ size?: number; className?: string }>;
  label: string;
  onClick: () => void;
  disabled?: boolean;
}

function ToolbarButton({
  icon: Icon,
  label,
  onClick,
  disabled,
}: ToolbarButtonProps) {
  return (
    <button
      onClick={onClick}
      disabled={disabled}
      className="flex items-center gap-1.5 px-3 py-1.5 text-sm text-gray-700 hover:bg-gray-100 rounded disabled:opacity-50 disabled:cursor-not-allowed"
      title={label}
    >
      <Icon size={16} />
      <span className="hidden sm:inline">{label}</span>
    </button>
  );
}
