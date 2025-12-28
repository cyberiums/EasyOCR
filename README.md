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
- 📦 Batch processing with progress indicators
- 🔧 Configuration file support
- 🌐 REST API mode
- 🐳 Docker support

📖 **Documentation:** [Rust Documentation](./rust/README.md)

```bash
cd rust
cargo build --release
```

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

### Rust CLI (rustocr)
```bash
# The Rust CLI uses easyocr2 Python library internally
pip install easyocr2

# Build and use rustocr
cd rust
cargo build --release
./target/release/rustocr -i image.jpg -l en
```

## Features

- Support for 80+ languages
- GPU acceleration  
- Bounding box and confidence scores
- Paragraph grouping
- Custom model support

## Architecture

```
┌──────────────┐
│   rustocr    │ (Rust CLI)
│  (Fast I/O)  │
└──────┬───────┘
       │
       ↓ subprocess
┌──────────────┐
│  easyocr2    │ (Python Library)
│  (ML Models) │
└──────────────┘
```

## License

Apache 2.0
