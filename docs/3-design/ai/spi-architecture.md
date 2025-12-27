# AI Provider SPI Architecture

**Audience**: Developers, Integrators
**Status**: Planned

## Overview

Identikit uses a Service Provider Interface (SPI) pattern for AI capabilities. This allows:

1. **Default open source** - Ships with MIT/Apache licensed models
2. **Pluggable providers** - Users can swap implementations
3. **Commercial options** - Licensed models can be added
4. **Future-proof** - New providers without core changes

## Provider Interfaces

### FaceDetector

```rust
pub trait FaceDetector: Send + Sync {
    fn detect(&self, image: &DynamicImage) -> Result<Vec<FaceDetection>, Error>;
    fn name(&self) -> &str;
    fn version(&self) -> &str;
}

pub struct FaceDetection {
    pub bbox: BoundingBox,
    pub landmarks: Option<[Point; 5]>,
    pub confidence: f32,
}
```

### FaceEmbedder

```rust
pub trait FaceEmbedder: Send + Sync {
    fn embed(&self, aligned_face: &DynamicImage) -> Result<Vec<f32>, Error>;
    fn embedding_size(&self) -> usize;  // typically 512
    fn name(&self) -> &str;
    fn version(&self) -> &str;
}
```

### FaceAligner

```rust
pub trait FaceAligner: Send + Sync {
    fn align(&self, image: &DynamicImage, landmarks: &[Point; 5]) -> Result<DynamicImage, Error>;
    fn output_size(&self) -> (u32, u32);  // typically (112, 112)
}
```

## Default Providers (Open Source)

| Interface | Default Provider | License | Model Size |
|-----------|------------------|---------|------------|
| FaceDetector | OpenCV DNN (YuNet) | Apache 2.0 | ~200KB |
| FaceAligner | Built-in (Affine) | MIT | - |
| FaceEmbedder | AuraFace | MIT | ~4MB |

### Total Default Bundle: ~5MB

## Provider Registry

```rust
pub struct ProviderRegistry {
    detector: Box<dyn FaceDetector>,
    aligner: Box<dyn FaceAligner>,
    embedder: Box<dyn FaceEmbedder>,
}

impl ProviderRegistry {
    pub fn default() -> Self {
        Self {
            detector: Box::new(OpenCVYuNetDetector::new()),
            aligner: Box::new(AffineAligner::default()),
            embedder: Box::new(AuraFaceEmbedder::new()),
        }
    }

    pub fn with_detector(mut self, detector: impl FaceDetector + 'static) -> Self {
        self.detector = Box::new(detector);
        self
    }

    pub fn with_embedder(mut self, embedder: impl FaceEmbedder + 'static) -> Self {
        self.embedder = Box::new(embedder);
        self
    }
}
```

## Configuration

### Config File (identikit.toml)

```toml
[ai.providers]
detector = "opencv-yunet"      # default
embedder = "auraface"          # default

# Alternative: Use InsightFace (requires commercial license)
# detector = "insightface-scrfd"
# embedder = "insightface-arcface"

[ai.models]
# Custom model paths (optional)
# detector_path = "/path/to/custom/detector.onnx"
# embedder_path = "/path/to/custom/embedder.onnx"
```

### Environment Variables

```bash
# Override provider at runtime
IDENTIKIT_AI_DETECTOR=insightface-scrfd
IDENTIKIT_AI_EMBEDDER=insightface-arcface

# Custom model paths
IDENTIKIT_MODEL_DETECTOR=/path/to/model.onnx
IDENTIKIT_MODEL_EMBEDDER=/path/to/model.onnx
```

## Available Providers

### Tier 1: Bundled (Open Source)

| Provider ID | Interface | License | Notes |
|-------------|-----------|---------|-------|
| `opencv-yunet` | FaceDetector | Apache 2.0 | Fast, lightweight |
| `auraface` | FaceEmbedder | MIT | Commercial-friendly ArcFace |
| `affine` | FaceAligner | MIT | Standard 5-point alignment |

### Tier 2: Optional (Open Source)

| Provider ID | Interface | License | Notes |
|-------------|-----------|---------|-------|
| `openface` | FaceEmbedder | Apache 2.0 | CMU model |
| `mediapipe` | FaceDetector | Apache 2.0 | Google's solution |

### Tier 3: Commercial (Requires License)

| Provider ID | Interface | License | Notes |
|-------------|-----------|---------|-------|
| `insightface-scrfd` | FaceDetector | Commercial | Higher accuracy |
| `insightface-arcface` | FaceEmbedder | Commercial | State-of-the-art |

## Implementing a Custom Provider

### 1. Implement the Trait

```rust
use identikit_ai::{FaceDetector, FaceDetection, Error};

pub struct MyCustomDetector {
    model: ort::Session,
}

impl FaceDetector for MyCustomDetector {
    fn detect(&self, image: &DynamicImage) -> Result<Vec<FaceDetection>, Error> {
        // Your implementation
    }

    fn name(&self) -> &str { "my-custom-detector" }
    fn version(&self) -> &str { "1.0.0" }
}
```

### 2. Register the Provider

```rust
let registry = ProviderRegistry::default()
    .with_detector(MyCustomDetector::new());
```

### 3. Configure via Config

```toml
[ai.providers]
detector = "my-custom-detector"

[ai.models]
detector_path = "/path/to/my-model.onnx"
```

## Directory Structure

```
backend/
├── src/
│   └── ai/
│       ├── mod.rs
│       ├── traits.rs           # SPI trait definitions
│       ├── registry.rs         # Provider registry
│       ├── providers/
│       │   ├── mod.rs
│       │   ├── opencv_yunet.rs # Default detector
│       │   ├── auraface.rs     # Default embedder
│       │   ├── affine.rs       # Default aligner
│       │   └── insightface.rs  # Optional commercial
│       └── comparison.rs       # Similarity calculation
│
└── models/
    ├── yunet.onnx              # Bundled (Apache 2.0)
    └── auraface.onnx           # Bundled (MIT)
```

## License Compliance

### Bundled Models

All bundled models must be:
- MIT, Apache 2.0, or BSD licensed
- No non-commercial restrictions
- Redistributable

### User-Provided Models

Users are responsible for:
- Obtaining proper licenses for commercial models
- Complying with InsightFace license if using their models
- Any legal obligations from their chosen providers

### License Notice (in app)

```
Face recognition powered by:
- OpenCV (Apache 2.0)
- AuraFace (MIT)
- ONNX Runtime (MIT)

Optional commercial providers available.
See documentation for licensing details.
```

## Comparison Pipeline

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Image A   │     │   Image B   │     │             │     │             │
└──────┬──────┘     └──────┬──────┘     │             │     │             │
       │                   │            │             │     │             │
       ▼                   ▼            │             │     │             │
┌─────────────┐     ┌─────────────┐     │             │     │             │
│  Detector   │     │  Detector   │     │             │     │             │
│    (SPI)    │     │    (SPI)    │     │             │     │             │
└──────┬──────┘     └──────┬──────┘     │             │     │             │
       │                   │            │             │     │             │
       ▼                   ▼            │             │     │             │
┌─────────────┐     ┌─────────────┐     │             │     │             │
│   Aligner   │     │   Aligner   │     │             │     │             │
│    (SPI)    │     │    (SPI)    │     │             │     │             │
└──────┬──────┘     └──────┬──────┘     │             │     │             │
       │                   │            │             │     │             │
       ▼                   ▼            │             │     │             │
┌─────────────┐     ┌─────────────┐     │             │     │             │
│  Embedder   │     │  Embedder   │     │             │     │             │
│    (SPI)    │     │    (SPI)    │     │             │     │             │
└──────┬──────┘     └──────┬──────┘     │             │     │             │
       │                   │            │             │     │             │
       ▼                   ▼            │             │     │             │
┌─────────────────────────────────┐     │             │     │             │
│       Cosine Similarity         │     │             │     │             │
│         (built-in)              │     │             │     │             │
└──────────────┬──────────────────┘     │             │     │             │
               │                        │             │     │             │
               ▼                        │             │     │             │
        ┌─────────────┐                 │             │     │             │
        │   Score     │                 │             │     │             │
        │  0.0 - 1.0  │                 │             │     │             │
        └─────────────┘                 │             │     │             │
```

## Related

- [Face Recognition Capabilities](face-recognition.md)
- [Architecture](../architecture.md)
- [Toolchain](../toolchain.md)

---

**Last Updated**: 2025-12-27
