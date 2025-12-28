# RustOCR

A fast Rust CLI wrapper for EasyOCR providing OCR capabilities with 80+ language support.

## Features

- 🚀 **Fast CLI interface** built with Rust
- 🌍 **80+ languages** supported (via EasyOCR)
- 🎯 **GPU acceleration** support (CUDA/MPS)
- 📊 **Multiple output formats**: JSON, text, detailed
- 🔧 **Easy to use** with sensible defaults

## Prerequisites

Before using RustOCR, ensure you have:

1. **Python 3.7+** installed
2. **EasyOCR** Python library:
   ```bash
   pip install easyocr
   ```

   > **Note:** If you encounter import errors with EasyOCR (e.g., `bidi` module issues), try reinstalling:
   > ```bash
   > pip install --upgrade --force-reinstall python-bidi easyocr
   > ```

3. **Rust toolchain** (for building from source):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

## Installation

### Build from source

```bash
cd /home/user/EasyOCR/rustOCR
cargo build --release
```

The binary will be available at `target/release/rustocr`.

### Install system-wide (optional)

```bash
cargo install --path .
```

## Usage

### Basic Examples

```bash
# Simple OCR on an English image
./target/release/rustocr -i image.jpg

# OCR with Chinese and English
./target/release/rustocr -i chinese.jpg -l ch_sim,en

# Use CPU instead of GPU
./target/release/rustocr -i document.png -g false

# Get simple text output (no bounding boxes)
./target/release/rustocr -i image.jpg -d 0 -o text

# Detailed human-readable output
./target/release/rustocr -i image.jpg -o detailed
```

### Command-Line Options

```
Options:
  -i, --input <FILE>         Input image file path
  -l, --languages <LANGS>    Languages to recognize (comma-separated) [default: en]
  -g, --gpu <BOOL>          Enable GPU acceleration [default: true]
  -d, --detail <LEVEL>       Detail level: 0 (text only) or 1 (full) [default: 1]
  -o, --output <FORMAT>      Output format: json, text, or detailed [default: json]
  -h, --help                Print help
  -V, --version             Print version
```

### Supported Languages

EasyOCR supports 80+ languages including:

- **Latin**: English (en), French (fr), German (de), Spanish (es), Portuguese (pt), Italian (it), etc.
- **Chinese**: Simplified (ch_sim), Traditional (ch_tra)
- **Japanese** (ja), **Korean** (ko), **Thai** (th)
- **Arabic** (ar), **Persian** (fa), **Urdu** (ur)
- **Cyrillic**: Russian (ru), Ukrainian (uk), Bulgarian (bg), etc.
- **Devanagari**: Hindi (hi), Marathi (mr), Nepali (ne)
- And many more...

See the [full list of supported languages](https://www.jaided.ai/easyocr).

## Output Formats

### JSON (default)
```json
[
  {
    "bbox": [[10, 20], [100, 20], [100, 50], [10, 50]],
    "text": "Hello World",
    "confidence": 0.9872
  }
]
```

### Text
```
Hello World
Sample Text
```

### Detailed
```
--- Result 1 ---
Text: Hello World
Confidence: 0.9872
Bounding Box: [[10, 20], [100, 20], [100, 50], [10, 50]]
```

## Performance Tips

- **GPU**: For best performance, use GPU mode (default). Requires CUDA or MPS support.
- **CPU**: Use `-g false` if you don't have GPU support. Performance will be slower but still functional.
- **Batch Processing**: For multiple images, call the binary in a loop or script.

## Troubleshooting

### "Failed to import easyocr"
Make sure EasyOCR is installed:
```bash
pip install easyocr
```

### GPU-related errors
If you encounter GPU errors, try CPU mode:
```bash
./target/release/rustocr -i image.jpg -g false
```

### "Input file does not exist"
Ensure the image path is correct and the file exists.

## Architecture

RustOCR uses PyO3 to bridge Rust and Python, calling the EasyOCR library at runtime:

```
┌─────────────┐
│  Rust CLI   │ (Argument parsing, error handling)
│   (clap)    │
└──────┬──────┘
       │
       ↓
┌─────────────┐
│    PyO3     │ (Rust ↔ Python bridge)
└──────┬──────┘
       │
       ↓
┌─────────────┐
│  EasyOCR    │ (Python library)
│  (PyTorch)  │
└─────────────┘
```

## License

Apache-2.0 (matching the parent EasyOCR project)

## Credits

- [EasyOCR](https://github.com/JaidedAI/EasyOCR) - The underlying OCR engine
- Built with [PyO3](https://pyo3.rs/) and [Clap](https://docs.rs/clap/)
