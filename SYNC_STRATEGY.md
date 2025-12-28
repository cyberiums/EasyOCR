# Repository Sync Strategy

## Overview

RustOCR is maintained in **two repositories** for different audiences:

1. **cyberiums/RustOCR** - Dedicated Rust development
2. **cyberiums/EasyOCR/rust/** - Unified with Python library

## Why Two Repositories?

- **Rust developers:** Easier discovery and Rust-focused tooling
- **Unified users:** One-stop shop for all EasyOCR features  
- **Clear focus:** Each repo serves distinct use cases

## Primary Repository: RustOCR

**Development happens here first**
- URL: https://github.com/cyberiums/RustOCR
- Rust-specific CI/CD
- Cargo registry publishing
- Rust community engagement

## Mirror: EasyOCR/rust/

**Synced from RustOCR**
- URL: https://github.com/cyberiums/EasyOCR
- Provides context with Python library
- Unified documentation
- Docker images combining both

## Sync Process

### Manual Sync (Current)
```bash
# Copy changes from RustOCR to EasyOCR
cp -r /path/to/RustOCR/* /path/to/EasyOCR/rust/
cd /path/to/EasyOCR
git add rust/
git commit -m "Sync from RustOCR v0.x.x"
git push
```

### Automated Sync (Future)
GitHub Action to sync on every RustOCR release:
```yaml
name: Sync to EasyOCR
on:
  release:
    types: [published]
jobs:
  sync:
    steps:
      - name: Checkout RustOCR
      - name: Clone EasyOCR
      - name: Copy files to rust/
      - name: Commit and push
```

## Release Strategy

### RustOCR Releases
- Tags: `v0.2.0`, `v0.3.0`, `v1.0.0`
- Changelog in RustOCR repo
- Pre-built binaries attached
- Cargo crate publishing

### EasyOCR Releases  
- Tags: `rust-v0.2.0`, `rust-v0.3.0`, etc.
- Combined changelog for Python + Rust
- Docker images: `cyberiums/easyocr:rust-latest`

## Update Process

1. **Develop in RustOCR**
2. **Test and release in RustOCR**
3. **Sync to EasyOCR/rust/**
4. **Tag unified release**
5. **Update documentation**

## Current Status

- ✅ RustOCR repo: Active at https://github.com/cyberiums/RustOCR
- ✅ EasyOCR repo: Active at https://github.com/cyberiums/EasyOCR  
- ✅ Both repos functional
- ⏳ Automated sync: To be implemented in v0.3.0
