# ADR-005: AI Provider SPI Architecture

## Status

**Accepted**

## Context

Identikit includes AI-powered face recognition for comparing photos. Requirements:
- Face detection (find faces in images)
- Face alignment (normalize to standard crop)
- Face embeddings (generate identity vectors)
- Similarity scoring (compare two faces)

Key constraints:
- Must support open source models (MIT/Apache licensed) for commercial use
- Should allow optional commercial providers for higher accuracy
- All processing must run locally (no cloud APIs)
- Must be extensible for future providers

## Options Considered

### Option 1: Hardcoded InsightFace

Use InsightFace models directly.

**Pros:**
- State-of-the-art accuracy
- Well-documented

**Cons:**
- Non-commercial license for pre-trained models
- No flexibility to swap providers
- Vendor lock-in

### Option 2: Hardcoded Open Source Only

Use only MIT/Apache licensed models.

**Pros:**
- Clear commercial licensing
- No legal concerns

**Cons:**
- May sacrifice accuracy
- No option for users who have commercial licenses
- Inflexible

### Option 3: Service Provider Interface (SPI)

Define traits for each capability, allow pluggable implementations.

**Pros:**
- Default to open source (commercial-friendly)
- Users can plug in commercial providers
- Extensible for future models
- Clear separation of concerns

**Cons:**
- More upfront design work
- Must maintain interface stability

### Option 4: External Process

Shell out to Python for AI processing.

**Pros:**
- Access to full Python ML ecosystem
- Easier model experimentation

**Cons:**
- Deployment complexity (bundle Python)
- Performance overhead
- Cross-platform issues

## Decision

**Option 3: Service Provider Interface (SPI)**

Implement a trait-based SPI allowing pluggable providers.

Rationale:
1. Ships with open source models by default (commercial-friendly)
2. Users with commercial licenses can upgrade
3. Future-proof for new models
4. Clean architecture separates AI from business logic

## Implementation

### Traits

```rust
pub trait FaceDetector: Send + Sync {
    fn detect(&self, image: &DynamicImage) -> Result<Vec<FaceDetection>, Error>;
    fn name(&self) -> &str;
    fn version(&self) -> &str;
}

pub trait FaceEmbedder: Send + Sync {
    fn embed(&self, aligned_face: &DynamicImage) -> Result<Vec<f32>, Error>;
    fn embedding_size(&self) -> usize;
    fn name(&self) -> &str;
    fn version(&self) -> &str;
}

pub trait FaceAligner: Send + Sync {
    fn align(&self, image: &DynamicImage, landmarks: &[Point; 5]) -> Result<DynamicImage, Error>;
    fn output_size(&self) -> (u32, u32);
}
```

### Default Providers (Open Source)

| Interface | Provider | License | Model |
|-----------|----------|---------|-------|
| FaceDetector | OpenCV YuNet | Apache 2.0 | yunet.onnx (~200KB) |
| FaceAligner | Affine | MIT | Built-in |
| FaceEmbedder | AuraFace | MIT | auraface.onnx (~4MB) |

### Optional Providers (Commercial)

| Interface | Provider | License |
|-----------|----------|---------|
| FaceDetector | InsightFace SCRFD | Commercial |
| FaceEmbedder | InsightFace ArcFace | Commercial |

### Provider Registry

```rust
pub struct ProviderRegistry {
    detector: Box<dyn FaceDetector>,
    aligner: Box<dyn FaceAligner>,
    embedder: Box<dyn FaceEmbedder>,
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self {
            detector: Box::new(YuNetDetector::new()),
            aligner: Box::new(AffineAligner::default()),
            embedder: Box::new(AuraFaceEmbedder::new()),
        }
    }
}
```

### Configuration

```toml
# identikit.toml
[ai.providers]
detector = "opencv-yunet"
embedder = "auraface"

# Or for commercial users:
# detector = "insightface-scrfd"
# embedder = "insightface-arcface"
```

### Directory Structure

```
backend/src/ai/
├── mod.rs
├── traits.rs           # SPI trait definitions
├── registry.rs         # Provider registry
├── providers/
│   ├── mod.rs
│   ├── yunet.rs        # Default detector (Apache 2.0)
│   ├── auraface.rs     # Default embedder (MIT)
│   ├── affine.rs       # Default aligner (MIT)
│   └── insightface.rs  # Optional commercial
└── comparison.rs       # Cosine similarity
```

## Consequences

### Positive
- Clear licensing: ships 100% open source
- Flexible: users can upgrade to commercial
- Extensible: new providers without core changes
- Testable: can mock providers

### Negative
- Interface must remain stable
- More code than hardcoded approach
- Must document provider installation

### Mitigations
- Version provider interfaces
- Provide migration guides for interface changes
- Bundle default models, document commercial setup

## References

- [OpenCV Zoo - YuNet](https://github.com/opencv/opencv_zoo)
- [AuraFace](https://huggingface.co/fal/AuraFace-v1)
- [InsightFace](https://insightface.ai/)
- [ONNX Runtime (ort)](https://ort.pyke.io/)
- [SPI Pattern](https://en.wikipedia.org/wiki/Service_provider_interface)

---

**Date**: 2025-12-27
**Author**: Engineering Team
