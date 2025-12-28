# EasyOCR

Ready-to-use OCR with 80+ supported languages and all popular writing scripts including Latin, Chinese, Arabic, Devanagari, Cyrillic and more.

## 🌟 Two Implementations

This repository contains both:

### 1. **Python Library** (Original EasyOCR)
Ready-to-use OCR powered by deep learning models.

📖 **Documentation:** [Python Documentation](./python/README.md)

### 2. **Rust CLI** 🦀 **NEW!**
High-performance command-line interface with advanced features:
- 🚀 Server mode for persistent processing (5-10x faster)
- 📦 Batch processing with progress indicators
- 🔧 Configuration file support
- 🌐 REST API mode
- 🐳 Docker support

📖 **Documentation:** [Rust Documentation](./rust/README.md)

## Quick Start

### Python
```bash
pip install easyocr
```

```python
import easyocr
reader = easyocr.Reader(['en'])
result = reader.readtext('image.jpg')
```

### Rust CLI
```bash
# Install from releases
# Or build from source
cd rust
cargo build --release

# Use it
./target/release/rustocr -i image.jpg -l en
```

## Features

- Support for 80+ languages
- GPU acceleration
- Bounding box and confidence scores
- Paragraph grouping
- Custom model support

## Examples

![Example](examples/example.png)

## License

Apache 2.0

## Credits

- Original EasyOCR by [JaiedAI](https://www.jaided.ai/easyocr)
- Rust CLI by cyberiums contributors
