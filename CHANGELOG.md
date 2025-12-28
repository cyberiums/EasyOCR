# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Rust CLI

#### Added
- Initial Rust CLI implementation (v0.2.0)
- Support for 80+ languages via Python bridge
- Multiple output formats (JSON, text, detailed)
- GPU/CPU mode selection
- Comprehensive documentation

#### Planned (v0.3.0)
- Server mode for persistent processing
- Batch processing with progress bars
- Configuration file support
- Performance improvements

---

## [rust-v0.2.0] - 2025-12-28

### Added
- Rust CLI wrapper using subprocess approach
- Python bridge script for EasyOCR integration
- CLI with clap for argument parsing
- JSON, text, and detailed output formats
- Language selection support
- GPU/CPU mode configuration
- Comprehensive README and documentation

### Technical
- Removed PyO3 dependency to avoid library linking issues
- Implemented subprocess-based Python calling
- Clean build with zero warnings
- Tested with English and Chinese images

---

## Python EasyOCR

For Python library changelog, see the original [EasyOCR repository](https://github.com/JaidedAI/EasyOCR).
