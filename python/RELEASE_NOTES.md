# EasyOCR2 v1.0.0 - Python Package Release

## Release Information

**Package:** easyocr2  
**Version:** 1.0.0  
**Release Date:** 2025-12-28  
**Repository:** https://github.com/cyberiums/EasyOCR

## What's New

EasyOCR2 v1.0.0 is a fork/continuation of the original EasyOCR library, maintained by Cyberiums.

### Key Features

- 🌍 **80+ Languages** - All major writing systems supported
- 🚀 **Deep Learning** - State-of-the-art OCR accuracy
- 📦 **Easy to Use** - Simple 3-line API
- 🎯 **GPU Acceleration** - CUDA and MPS support
- 🔧 **Integration Ready** - Works with RustOCR CLI

### Installation

```bash
pip install easyocr2
```

### Quick Start

```python
import easyocr2

reader = easyocr2.Reader(['en'])
result = reader.readtext('image.jpg')
```

### Integration with RustOCR

EasyOCR2 is designed to work seamlessly with the RustOCR CLI:

```bash
pip install easyocr2
rustocr --server              # Start server
rustocr -i img.jpg --use-server  # 5-10x faster!
```

## Technical Details

### Package Structure

- **Package Name:** `easyocr2`
- **Import:** `import easyocr2`
- **Entry Point:** `easyocr2` CLI command
- **License:** Apache-2.0

### Requirements

- Python >= 3.7
- PyTorch >= 1.6
- See `requirements.txt` for full list

### Distribution Files

- Source distribution: `easyocr2-1.0.0.tar.gz`
- Wheel: `easyocr2-1.0.0-py3-none-any.whl`

## Changes from Original EasyOCR

1. **Package renamed:** `easyocr` → `easyocr2`
2. **New repository:** cyberiums/EasyOCR
3. **Integrated:** Works with RustOCR server mode
4. **Fresh version:** Starting at 1.0.0

## Testing

Package has been tested for:
- ✓ Import functionality
- ✓ Reader class availability
- ✓ Version reporting

## Installation Methods

### From PyPI (after release)
```bash
pip install easyocr2
```

### From Source
```bash
git clone https://github.com/cyberiums/EasyOCR
cd EasyOCR/python
pip install -e .
```

## Credits

- Original EasyOCR by JaidedAI
- Maintained by Cyberiums
- Apache-2.0 License

## Next Steps

- Upload to PyPI
- Tag release: `python-v1.0.0`
- Update RustOCR server to use `easyocr2`
- Publish release notes
