# AI Face Recognition Capabilities

**Audience**: Developers, Technical Architects
**Status**: Planned

## Overview

AI-powered face detection, alignment, and comparison to enable accurate photo-to-photo matching regardless of lighting, angle, or source.

Uses **SPI (Service Provider Interface)** pattern - see [SPI Architecture](spi-architecture.md).

## Default Providers (Open Source)

| Capability | Provider | License | Size |
|------------|----------|---------|------|
| Face detection | OpenCV YuNet | Apache 2.0 | ~200KB |
| Facial landmarks | Built into detector | - | - |
| Face alignment | Affine transform | MIT | - |
| Face embeddings | AuraFace | MIT | ~4MB |
| Similarity score | Cosine similarity | MIT | - |

**Total bundle: ~5MB** (fully open source, commercial-friendly)

## Optional Commercial Providers

| Capability | Provider | License | Notes |
|------------|----------|---------|-------|
| Face detection | InsightFace SCRFD | Commercial | Higher accuracy |
| Face embeddings | InsightFace ArcFace | Commercial | State-of-the-art |

Contact InsightFace for commercial licensing.

## Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                         Frontend (React)                            │
│   ┌─────────────┐    ┌─────────────┐    ┌─────────────────────────┐ │
│   │ Upload Image│───►│ Compare UI  │◄───│ Similarity Results      │ │
│   └─────────────┘    └─────────────┘    └─────────────────────────┘ │
├─────────────────────────────────────────────────────────────────────┤
│                        Tauri IPC Bridge                             │
├─────────────────────────────────────────────────────────────────────┤
│                      Backend (Rust + ONNX)                          │
│   ┌─────────────┐    ┌─────────────┐    ┌─────────────────────────┐ │
│   │Face Detector│───►│  Aligner    │───►│ Embedding Extractor     │ │
│   │   (SCRFD)   │    │ (Landmarks) │    │     (ArcFace)           │ │
│   └─────────────┘    └─────────────┘    └─────────────────────────┘ │
│                                                    │                │
│                                                    ▼                │
│                                         ┌─────────────────────────┐ │
│                                         │  Cosine Similarity      │ │
│                                         │  (0.0 - 1.0 score)      │ │
│                                         └─────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────┘
```

## Pipeline

### 1. Face Detection (SCRFD)

Locate face bounding box and 5-point landmarks in image.

```
Input:  Raw image (any size)
Output:
  - Bounding box [x, y, width, height]
  - Landmarks [left_eye, right_eye, nose, left_mouth, right_mouth]
  - Confidence score
```

### 2. Face Alignment

Normalize face to standard 112x112 crop using affine transformation.

```
Input:  Original image + 5 landmarks
Output: Aligned face (112x112 RGB)

Process:
  1. Calculate affine transform from detected landmarks to reference landmarks
  2. Apply transform to crop and align face
  3. Normalize pixel values
```

### 3. Embedding Extraction (ArcFace)

Generate 512-dimensional feature vector representing face identity.

```
Input:  Aligned face (112x112 RGB)
Output: 512-float embedding vector

Model options:
  - MobileFace (4-8MB) - fast, good for real-time
  - ResNet50 (166MB) - higher accuracy
```

### 4. Similarity Calculation

Compare two embeddings using cosine similarity.

```
cosine_similarity = dot(embedding_a, embedding_b) /
                    (norm(embedding_a) * norm(embedding_b))

Score range: -1.0 to 1.0 (typically 0.0 to 1.0 for faces)

Thresholds:
  > 0.6  : Likely same person
  > 0.7  : High confidence same person
  > 0.8  : Very high confidence
```

## Rust Implementation

### Dependencies (Cargo.toml)

```toml
[dependencies]
ort = "2.0"                    # ONNX Runtime wrapper
ndarray = "0.15"               # N-dimensional arrays
image = "0.24"                 # Image processing
imageproc = "0.23"             # Image transformations
```

### Tauri Commands

| Command | Input | Output |
|---------|-------|--------|
| `detect_faces` | image_path | Vec<FaceDetection> |
| `extract_embedding` | image_path | Vec<f32> (512) |
| `compare_faces` | image_a, image_b | SimilarityResult |
| `compare_to_composite` | composite_png, reference_path | SimilarityResult |

### Data Structures

```rust
pub struct FaceDetection {
    pub bbox: BoundingBox,
    pub landmarks: [Point; 5],
    pub confidence: f32,
}

pub struct SimilarityResult {
    pub score: f32,           // 0.0 - 1.0
    pub confidence: String,   // "low", "medium", "high", "very_high"
    pub face_a_detected: bool,
    pub face_b_detected: bool,
}
```

## Models

### Default Models (Bundled - Open Source)

| Model | File | Size | License | Source |
|-------|------|------|---------|--------|
| YuNet | yunet.onnx | 200KB | Apache 2.0 | [OpenCV](https://github.com/opencv/opencv_zoo) |
| AuraFace | auraface.onnx | 4MB | MIT | [HuggingFace](https://huggingface.co/fal/AuraFace-v1) |

### Optional Models (Commercial License Required)

| Model | File | Size | License | Source |
|-------|------|------|---------|--------|
| SCRFD-10G | scrfd_10g.onnx | 4.2MB | Commercial | [InsightFace](https://github.com/deepinsight/insightface) |
| ArcFace-MobileFace | mobilefacenet.onnx | 4.0MB | Commercial | [InsightFace](https://github.com/deepinsight/insightface) |
| ArcFace-R50 | arcface_r50.onnx | 166MB | Commercial | [InsightFace](https://github.com/deepinsight/insightface) |

### Model Storage

```
backend/
└── models/
    ├── yunet.onnx          # Bundled (Apache 2.0)
    └── auraface.onnx       # Bundled (MIT)
```

Default bundle: ~5MB (fully open source, commercial-friendly)

## Performance Targets

| Operation | Target | Hardware |
|-----------|--------|----------|
| Face detection | <50ms | CPU |
| Embedding extraction | <30ms | CPU |
| Full comparison | <100ms | CPU |

## Use Cases

### 1. Compare Two Uploaded Photos
```
User uploads Photo A (e.g., YouTube screenshot)
User uploads Photo B (e.g., Instagram photo)
System: Detect → Align → Embed → Compare
Output: "78% similarity - High confidence same person"
```

### 2. Compare Composite to Reference
```
User builds composite in editor
User uploads reference photo
System: Render composite to PNG → Embed → Compare to reference
Output: "45% similarity - Low confidence"
```

### 3. Multi-Face Comparison
```
Photo contains multiple faces
System: Detect all faces → Show selection UI
User selects which face to compare
```

## Privacy & Legal Considerations

- **No cloud processing** - All inference runs locally
- **No face database** - Embeddings not stored permanently
- **User consent** - Clear UI indicating AI analysis
- **Audit logging** - Optional logging for law enforcement compliance
- **Export restrictions** - Consider export controls for face recognition tech

## Limitations

1. **Accuracy varies** with image quality, pose, occlusion
2. **Not forensic-grade** - Should not be sole evidence
3. **Bias concerns** - Models may perform differently across demographics
4. **Composite limitations** - Stylized composites may not embed well

## Future Enhancements

- [ ] GPU acceleration (CUDA/Metal)
- [ ] Age progression models
- [ ] Expression normalization
- [ ] Batch processing
- [ ] Face database search (with appropriate safeguards)

## References

### Open Source (Default)
- [OpenCV Zoo](https://github.com/opencv/opencv_zoo) - YuNet face detector (Apache 2.0)
- [AuraFace](https://huggingface.co/fal/AuraFace-v1) - Face embeddings (MIT)
- [ort (ONNX Runtime for Rust)](https://ort.pyke.io/) - Inference engine (MIT)

### Commercial (Optional)
- [InsightFace](https://github.com/deepinsight/insightface) - SCRFD, ArcFace (Commercial license)

### Papers
- [ArcFace Paper](https://arxiv.org/abs/1801.07698) - Additive Angular Margin Loss
- [SCRFD Paper](https://arxiv.org/abs/2105.04714) - Sample and Computation Redistribution
- [YuNet Paper](https://arxiv.org/abs/2108.10203) - Lightweight face detection

## Related

- [SPI Architecture](spi-architecture.md)
- [Architecture](../architecture.md)
- [Toolchain](../toolchain.md)

---

**Last Updated**: 2025-12-27
