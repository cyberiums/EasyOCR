# EasyOCR2

**Fastly Built by [FastBuilder.AI](https://fastbuilder.ai)** 🚀

Next-generation ready-to-use OCR with 80+ supported languages and all popular writing scripts.

## Installation

```bash
pip install easyocr2
```

## Quick Start

```python
import easyocr2

# Create reader
reader = easyocr2.Reader(['en'])

# Perform OCR
result = reader.readtext('image.jpg')
print(result)
```

## Features

- 🌍 **80+ Languages** Support
- 🚀 **GPU Acceleration** (CUDA/MPS)
- 📦 **Easy Installation** via pip
- 🎯 **High Accuracy** with deep learning
- 🔧 **Simple API** - just 3 lines of code

## Supported Languages

Latin, Chinese (Simplified & Traditional), Japanese, Korean, Thai, Arabic, Cyrillic, Devanagari, and many more!

See full list at: https://github.com/cyberiums/EasyOCR

## Advanced Usage

```python
import easyocr2

# GPU mode (default)
reader = easyocr2.Reader(['en'], gpu=True)

# CPU mode
reader = easyocr2.Reader(['en'], gpu=False)

# Multiple languages
reader = easyocr2.Reader(['en', 'ch_sim', 'ja'])

# Perform OCR with details
result = reader.readtext('image.jpg', detail=1)
for bbox, text, confidence in result:
    print(f"Text: {text}, Confidence: {confidence:.2f}")
```

## Integration with RustOCR

EasyOCR2 works seamlessly with RustOCR CLI for high-performance batch processing:

```bash
# Install easyocr2
pip install easyocr2

# Use with RustOCR server mode
rustocr --server
rustocr -i image.jpg --use-server  # 5-10x faster!
```

## Requirements

- Python >= 3.7
- PyTorch >= 1.6

## License

Apache-2.0

## Credits

- Fork/continuation of the original EasyOCR by JaidedAI
- Maintained by Cyberiums
- See: https://github.com/cyberiums/EasyOCR
