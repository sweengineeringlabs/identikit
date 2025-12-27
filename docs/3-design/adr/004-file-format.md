# ADR-004: Composite File Format

## Status

**Accepted**

## Context

Identikit needs a file format for saving and loading facial composite projects. Requirements:
- Store all composite data (layers, transforms, metadata)
- Human-readable for debugging
- Self-contained (no external references)
- Versioned for future compatibility
- Portable across platforms

## Options Considered

### Option 1: JSON (.idkit)

Plain JSON with custom extension.

**Pros:**
- Human-readable
- Native JavaScript/Rust support
- Easy to debug and inspect
- Simple to parse
- Git-friendly (diffable)

**Cons:**
- Larger file size than binary
- No compression by default
- No embedded binary data (images must be base64)

### Option 2: MessagePack (.idkit)

Binary JSON-like format.

**Pros:**
- Smaller than JSON
- Fast parsing
- Supports binary data

**Cons:**
- Not human-readable
- Requires library for viewing
- Less tooling support

### Option 3: SQLite (.idkit)

Embedded database file.

**Pros:**
- Structured queries
- Can store binary blobs
- ACID transactions

**Cons:**
- Overkill for single composite
- Not human-readable
- Heavier dependency

### Option 4: ZIP Archive (.idkit)

ZIP containing JSON + assets.

**Pros:**
- Compressed
- Can include binary assets
- Standard format

**Cons:**
- More complex to read/write
- Must extract to access
- Overhead for small files

### Option 5: Protocol Buffers (.idkit)

Binary serialization format.

**Pros:**
- Very compact
- Schema-enforced
- Fast serialization

**Cons:**
- Requires schema compilation
- Not human-readable
- Overkill for this use case

## Decision

**Option 1: JSON (.idkit)**

JSON provides the best balance of simplicity, readability, and tooling support.

Rationale:
1. Human-readable aids debugging and support
2. Native support in both TypeScript and Rust (serde_json)
3. Easy to version and extend
4. Git-friendly for version control
5. No external dependencies for viewing
6. Composite files are small (typically <100KB)

For future consideration: If file sizes become problematic, can add optional gzip compression (.idkit.gz).

## Implementation

### File Extension

`.idkit`

### MIME Type

`application/vnd.identikit.composite+json`

### Schema

```json
{
  "$schema": "https://identikit.dev/schema/v1.0.0",
  "version": "1.0.0",
  "id": "uuid-v4",
  "name": "Composite Name",
  "createdAt": "ISO8601",
  "modifiedAt": "ISO8601",
  "canvas": {
    "width": 800,
    "height": 1000,
    "backgroundColor": "#FFFFFF"
  },
  "layers": [
    {
      "id": "uuid-v4",
      "featureId": "hair-style-001",
      "name": "Hair",
      "visible": true,
      "locked": false,
      "opacity": 1.0,
      "zIndex": 0,
      "transform": {
        "x": 400,
        "y": 200,
        "scaleX": 1.0,
        "scaleY": 1.0,
        "rotation": 0,
        "flipX": false,
        "flipY": false
      },
      "colorOverrides": {}
    }
  ],
  "metadata": {
    "author": "Officer Name",
    "caseNumber": "2025-001",
    "description": "Witness description",
    "tags": ["male", "30-40"]
  }
}
```

### Rust Structures

```rust
#[derive(Serialize, Deserialize)]
pub struct CompositeFile {
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    pub version: String,
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub canvas: CanvasConfig,
    pub layers: Vec<Layer>,
    pub metadata: Option<Metadata>,
}
```

### Versioning Strategy

- **Major**: Breaking changes (old readers cannot open)
- **Minor**: New optional fields (backwards compatible)
- **Patch**: Bug fixes in format spec

Reader implementation:
```rust
fn load_composite(path: &Path) -> Result<Composite, Error> {
    let file = CompositeFile::from_path(path)?;

    match file.version.as_str() {
        "1.0.0" => parse_v1(file),
        v if v.starts_with("1.") => parse_v1(file), // Minor versions compatible
        _ => Err(Error::UnsupportedVersion(file.version)),
    }
}
```

## Consequences

### Positive
- Easy debugging with any text editor
- Simple implementation
- Version control friendly
- Cross-platform compatible

### Negative
- Larger than binary formats
- No compression by default
- Feature assets referenced by ID, not embedded

### Mitigations
- Add optional gzip support if size becomes issue
- Feature IDs validated against installed library
- Consider ZIP format for export/sharing (includes assets)

## References

- [JSON Schema](https://json-schema.org/)
- [serde_json](https://docs.rs/serde_json/)
- [Semantic Versioning](https://semver.org/)

---

**Date**: 2025-12-27
**Author**: Engineering Team
