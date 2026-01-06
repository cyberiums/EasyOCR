# EasyOCR2

Next-generation OCR with 80+ supported languages and all popular writing scripts including Latin, Chinese, Arabic, Devanagari, Cyrillic and more.

## 🌟 Two Implementations

This repository contains:

### 1. **Python Library (easyocr2)**
Next-generation Python OCR powered by deep learning models.

📖 **Documentation:** [Python Documentation](./python/README.md)

```bash
pip install easyocr2
```

### 2. **Rust CLI (rustocr)** 🦀
High-performance command-line interface with advanced features:
- 🚀 Server mode for persistent processing (5-10x faster)
-  Batch processing with progress indicators
- 🔧 Configuration file support
- 🌐 REST API mode
- 🐳 Docker support

📖 **Documentation:** [Rust Documentation](./rust/README.md)

## Quick Start

### Python (easyocr2)
```bash
pip install easyocr2
```

```python
import easyocr2
reader = easyocr2.Reader(['en'])
result = reader.readtext('image.jpg')
```

### Rust CLI (rustocr) - Python Bridge Mode (Recommended)

The Rust CLI provides a Python bridge script for maximum compatibility:

```bash
# Install Python dependencies
pip install easyocr

# Use the Python bridge directly (works on all architectures)
python3 rust/easyocr_bridge.py \
    --languages en \
    --image image.jpg \
    --gpu false \
    --detail 1
```

**Output (JSON):**
```json
[
  {
    "bbox": [[10, 20], [100, 20], [100, 50], [10, 50]],
    "text": "Hello World",
    "confidence": 0.9872
  }
]
```

### Using the Compiled Binary (x86_64 only)

> ⚠️ **Note:** Pre-compiled binaries may have CPU architecture issues on ARM64/Apple Silicon. Use Python bridge instead.

```bash
cd rust
cargo build --release
./rustocr.sh -i image.jpg -l en
```

## Features

- Support for 80+ languages
- GPU and CPU acceleration  
- Bounding box and confidence scores
- Paragraph grouping
- Custom model support
- JSON output format
- Cross-platform compatibility

## Architecture

### Method 1: Python Bridge (Recommended for Automation)
```
┌──────────────┐
│  Your Code   │ (Any language: Rust, Python, Shell, etc.)
└──────┬───────┘
       │
       ↓ subprocess
┌──────────────┐
│easyocr_bridge│ (Python CLI script)
│    .py       │
└──────┬───────┘
       │
       ↓ import
┌──────────────┐
│  easyocr2    │ (Python Library with ML Models)
│  (PyTorch)   │
└──────────────┘
```

### Method 2: Compiled Binary
```
┌──────────────┐
│   rustocr    │ (Rust CLI binary)
│  (Fast I/O)  │
└──────┬───────┘
       │
       ↓ subprocess
┌──────────────┐
│easyocr_bridge│ (Python script)
│    .py       │
└──────┬───────┘
       │
       ↓ import
┌──────────────┐
│  easyocr2    │ (Python Library)
│  (ML Models) │
└──────────────┘
```

## Integration Guide

### FastBuilder.AI / Automated Systems

For automated OCR detection (button clicking, UI automation), use the Python bridge:

```rust
// Example: Rust integration
use std::process::Command;

let output = Command::new("python3")
    .arg("/path/to/easyocr_bridge.py")
    .arg("--languages").arg("en")
    .arg("--image").arg(screenshot_path)
    .arg("--gpu").arg("false")  // CPU mode for faster startup
    .arg("--detail").arg("1")   // Include bounding boxes
    .output()?;

let json_results = String::from_utf8_lossy(&output.stdout);
// Parse JSON and extract button locations
```

### Shell Scripts

```bash
#!/bin/bash
BUTTONS=$(python3 easyocr_bridge.py \
    --languages en \
    --image screenshot.png \
    --gpu false \
    --detail 1 | \
    jq '.[] | select(.text | test("Retry|Allow|Wait"; "i"))')

echo "Found buttons: $BUTTONS"
```

### Python Applications

```python
import subprocess, json

def detect_text(image_path):
    result = subprocess.run([
        "python3", "easyocr_bridge.py",
        "--languages", "en",
        "--image", image_path,
        "--gpu", "false",
        "--detail", "1"
    ], capture_output=True, text=True)
    
    return json.loads(result.stdout)

# Usage
detections = detect_text("screenshot.png")
for item in detections:
    print(f"{item['text']} at {item['bbox']} (confidence: {item['confidence']:.2f})")
```

## Performance Considerations

- **First Run:** Downloads models (~100MB) on first use
- **GPU vs CPU:** 
  - GPU: Faster processing but requires CUDA/MPS setup
  - CPU: Slower but more compatible, faster startup
- **Server Mode:** For high throughput, use server mode to keep models loaded
- **Language Selection:** Only load languages you need for faster init

## Troubleshooting

### "Bad CPU type in executable"
Use Python bridge instead of compiled binary:
```bash
python3 rust/easyocr_bridge.py --languages en --image image.png --gpu false --detail 1
```

### "ModuleNotFoundError: No module named 'easyocr'"
Install EasyOCR:
```bash
pip install easyocr
```

### GPU errors
Switch to CPU mode:
```bash
python3 easyocr_bridge.py --languages en --image image.png --gpu false --detail 1
```

## License

Apache 2.0

## Links

- **Python Library:** [./python/README.md](./python/README.md)
- **Rust CLI:** [./rust/README.md](./rust/README.md)
- **Upstream EasyOCR:** [https://github.com/JaidedAI/EasyOCR](https://github.com/JaidedAI/EasyOCR)
- **FastBuilder.AI:** [https://fastbuilder.ai](https://fastbuilder.ai)
