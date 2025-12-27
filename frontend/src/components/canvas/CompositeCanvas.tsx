import { useEffect, useRef, useCallback } from 'react';
import { Canvas, FabricImage, FabricObject } from 'fabric';
import { useCompositeStore, useUIStore } from '@/stores';
import type { Transform } from '@/types';

export function CompositeCanvas() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const fabricRef = useRef<Canvas | null>(null);

  const { composite, selectedLayerId, selectLayer, updateLayerTransform } =
    useCompositeStore();
  const { zoom } = useUIStore();

  // Initialize Fabric.js canvas
  useEffect(() => {
    if (canvasRef.current && !fabricRef.current) {
      const canvas = new Canvas(canvasRef.current, {
        width: composite?.canvas.width || 800,
        height: composite?.canvas.height || 1000,
        backgroundColor: composite?.canvas.backgroundColor || '#FFFFFF',
        selection: true,
        preserveObjectStacking: true,
      });

      fabricRef.current = canvas;

      // Selection event
      canvas.on('selection:created', (e) => {
        const selected = e.selected?.[0];
        if (selected && 'data' in selected) {
          const data = selected.data as { layerId?: string };
          if (data?.layerId) {
            selectLayer(data.layerId);
          }
        }
      });

      canvas.on('selection:cleared', () => {
        selectLayer(null);
      });

      // Object modified event
      canvas.on('object:modified', (e) => {
        const obj = e.target;
        if (obj && 'data' in obj) {
          const data = obj.data as { layerId?: string };
          if (data?.layerId) {
            const transform: Partial<Transform> = {
              x: obj.left || 0,
              y: obj.top || 0,
              scaleX: obj.scaleX || 1,
              scaleY: obj.scaleY || 1,
              rotation: obj.angle || 0,
              flipX: obj.flipX || false,
              flipY: obj.flipY || false,
            };
            updateLayerTransform(data.layerId, transform);
          }
        }
      });
    }

    return () => {
      fabricRef.current?.dispose();
      fabricRef.current = null;
    };
  }, []);

  // Sync layers to canvas
  useEffect(() => {
    if (!fabricRef.current || !composite) return;

    const canvas = fabricRef.current;

    // Clear existing objects
    canvas.clear();
    canvas.backgroundColor = composite.canvas.backgroundColor;

    // Sort layers by z-index and render
    const sortedLayers = [...composite.layers].sort(
      (a, b) => a.zIndex - b.zIndex
    );

    sortedLayers.forEach(async (layer) => {
      if (!layer.visible) return;

      try {
        // For now, create a placeholder rectangle since we don't have real SVGs
        const rect = new FabricObject({
          left: layer.transform.x,
          top: layer.transform.y,
          width: 150,
          height: 150,
          fill: getPlaceholderColor(layer.featureId),
          opacity: layer.opacity,
          scaleX: layer.transform.scaleX,
          scaleY: layer.transform.scaleY,
          angle: layer.transform.rotation,
          flipX: layer.transform.flipX,
          flipY: layer.transform.flipY,
          selectable: !layer.locked,
          data: { layerId: layer.id },
          originX: 'center',
          originY: 'center',
        });

        canvas.add(rect);
      } catch (e) {
        console.error('Failed to load feature:', layer.featureId, e);
      }
    });

    canvas.renderAll();
  }, [composite?.layers, composite?.canvas.backgroundColor]);

  // Handle zoom
  useEffect(() => {
    if (fabricRef.current) {
      fabricRef.current.setZoom(zoom);
      fabricRef.current.renderAll();
    }
  }, [zoom]);

  // Highlight selected layer
  useEffect(() => {
    if (!fabricRef.current) return;

    const canvas = fabricRef.current;
    const objects = canvas.getObjects();

    if (selectedLayerId) {
      const selectedObject = objects.find((obj) => {
        if ('data' in obj) {
          const data = obj.data as { layerId?: string };
          return data?.layerId === selectedLayerId;
        }
        return false;
      });

      if (selectedObject) {
        canvas.setActiveObject(selectedObject);
      }
    } else {
      canvas.discardActiveObject();
    }

    canvas.renderAll();
  }, [selectedLayerId]);

  return (
    <div className="canvas-container bg-gray-200 rounded-lg shadow-inner p-4">
      <canvas ref={canvasRef} />
    </div>
  );
}

// Helper function to get a placeholder color based on feature ID
function getPlaceholderColor(featureId: string): string {
  const colors: Record<string, string> = {
    hair: '#4A3728',
    face: '#E8BEAC',
    eyes: '#6B8E23',
    eyebrows: '#3D2314',
    nose: '#DDA0A0',
    mouth: '#CD5C5C',
    chin: '#E8BEAC',
    ears: '#E8BEAC',
    accessory: '#708090',
  };

  for (const [key, color] of Object.entries(colors)) {
    if (featureId.includes(key)) {
      return color;
    }
  }

  return '#CCCCCC';
}
