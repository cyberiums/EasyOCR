#!/usr/bin/env python3
"""
EasyOCR Python bridge script for RustOCR
This script is called by the Rust binary and handles the actual OCR processing
"""
import argparse
import json
import sys
import easyocr


def main():
    parser = argparse.ArgumentParser(description='EasyOCR Python bridge')
    parser.add_argument('--languages', required=True, help='Comma-separated language codes')
    parser.add_argument('--image', required=True, help='Path to image file')
    parser.add_argument('--gpu', type=lambda x: x.lower() == 'true', default=True, help='Use GPU')
    parser.add_argument('--detail', type=int, default=1, choices=[0, 1], help='Detail level')
    
    args = parser.parse_args()
    
    # Parse languages
    languages = [lang.strip() for lang in args.languages.split(',')]
    
    try:
        # Initialize reader
        reader = easyocr.Reader(languages, gpu=args.gpu, verbose=False)
        
        # Perform OCR
        results = reader.readtext(args.image, detail=args.detail)
        
        # Output results as JSON
        if args.detail == 0:
            # Simple mode: just text strings
            output = [{"text": text} for text in results]
        else:
            # Detailed mode: (bbox, text, confidence) tuples
            output = [
                {
                    "bbox": [[int(x), int(y)] for x, y in bbox],
                    "text": text,
                    "confidence": float(confidence)
                }
                for bbox, text, confidence in results
            ]
        
        print(json.dumps(output, ensure_ascii=False))
        return 0
        
    except Exception as e:
        print(json.dumps({"error": str(e)}), file=sys.stderr)
        return 1


if __name__ == '__main__':
    sys.exit(main())
