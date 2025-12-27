# .idkit File Format Specification

**Audience**: Developers, Integrators

## WHAT: File Format

The `.idkit` file format is a JSON-based format for storing facial composite projects.

## WHY: Design Goals

1. **Human-readable** - JSON for easy debugging and inspection
2. **Self-contained** - All composite data in one file
3. **Extensible** - Version field for future compatibility
4. **Portable** - No external dependencies or references

## HOW: Format Specification

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
  "name": "string",
  "createdAt": "ISO8601-datetime",
  "modifiedAt": "ISO8601-datetime",
  "canvas": {
    "width": "integer",
    "height": "integer",
    "backgroundColor": "hex-color"
  },
  "layers": [
    {
      "id": "uuid-v4",
      "featureId": "string",
      "name": "string",
      "visible": "boolean",
      "locked": "boolean",
      "opacity": "float (0-1)",
      "zIndex": "integer",
      "transform": {
        "x": "float",
        "y": "float",
        "scaleX": "float",
        "scaleY": "float",
        "rotation": "float (degrees)",
        "flipX": "boolean",
        "flipY": "boolean"
      },
      "colorOverrides": {
        "zoneId": "hex-color"
      }
    }
  ],
  "metadata": {
    "author": "string (optional)",
    "description": "string (optional)",
    "caseNumber": "string (optional)",
    "tags": ["string"]
  }
}
```

### Example File

```json
{
  "$schema": "https://identikit.dev/schema/v1.0.0",
  "version": "1.0.0",
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "Suspect Composite 001",
  "createdAt": "2025-01-15T10:30:00Z",
  "modifiedAt": "2025-01-15T14:45:00Z",
  "canvas": {
    "width": 800,
    "height": 1000,
    "backgroundColor": "#FFFFFF"
  },
  "layers": [
    {
      "id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
      "featureId": "face-shape-oval-001",
      "name": "Face Shape",
      "visible": true,
      "locked": false,
      "opacity": 1.0,
      "zIndex": 0,
      "transform": {
        "x": 400,
        "y": 500,
        "scaleX": 1.0,
        "scaleY": 1.0,
        "rotation": 0,
        "flipX": false,
        "flipY": false
      },
      "colorOverrides": {
        "skin": "#E8BEAC"
      }
    },
    {
      "id": "b2c3d4e5-f6a7-8901-bcde-f23456789012",
      "featureId": "eyes-almond-003",
      "name": "Eyes",
      "visible": true,
      "locked": false,
      "opacity": 1.0,
      "zIndex": 1,
      "transform": {
        "x": 400,
        "y": 420,
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
    "author": "Officer Smith",
    "description": "Witness description from incident #12345",
    "caseNumber": "2025-001-123",
    "tags": ["male", "caucasian", "30-40"]
  }
}
```

### Field Descriptions

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `$schema` | string | No | JSON schema URL for validation |
| `version` | string | Yes | Format version (semver) |
| `id` | string | Yes | Unique composite identifier (UUID v4) |
| `name` | string | Yes | User-defined composite name |
| `createdAt` | string | Yes | ISO 8601 creation timestamp |
| `modifiedAt` | string | Yes | ISO 8601 last modified timestamp |
| `canvas` | object | Yes | Canvas configuration |
| `layers` | array | Yes | Array of layer objects |
| `metadata` | object | No | Optional metadata |

### Versioning

Format versions follow semantic versioning:
- **Major**: Breaking changes (incompatible with older readers)
- **Minor**: New features (backwards compatible)
- **Patch**: Bug fixes

### Validation

1. Check `version` field exists and is supported
2. Validate required fields present
3. Validate UUID format for `id` and layer IDs
4. Validate `featureId` references exist in feature library
5. Validate transform values are within bounds

## Related

- [Architecture](architecture.md)

---

**Last Updated**: 2025-01-01
**Format Version**: 1.0.0
