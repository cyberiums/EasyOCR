"""
EasyOCR2 - Next-generation OCR with 80+ Languages
"""
from io import open
from setuptools import setup, find_packages
import os

# Read requirements directly
requirements = [
    'torch',
    'torchvision>=0.5',
    'opencv-python-headless',
    'scipy',
    'numpy',
    'Pillow',
    'scikit-image',
    'python-bidi',
    'PyYAML',
    'Shapely',
    'pyclipper',
]

def readme():
    readme_path = os.path.join(os.path.dirname(__file__), 'README.md')
    if os.path.exists(readme_path):
        with open(readme_path, encoding="utf-8-sig") as f:
            return f.read()
    return "Next-generation OCR with 80+ languages"

setup(
    name='easyocr2',
    packages=find_packages(),
    include_package_data=True,
    version='1.0.1',
    install_requires=requirements,
    entry_points={"console_scripts": ["easyocr2=easyocr2.cli:main"]},
    license='Apache License 2.0',
    description='Next-generation OCR with 80+ languages - Ready-to-use OCR powered by deep learning',
    long_description=readme(),
    long_description_content_type="text/markdown",
    author='Cyberiums',
    author_email='',
    url='https://github.com/cyberiums/EasyOCR',
    download_url='https://github.com/cyberiums/EasyOCR.git',
    keywords=['ocr', 'optical character recognition', 'deep learning', 'neural network', 'easyocr2'],
    classifiers=[
        'Development Status :: 5 - Production/Stable',
        'Intended Audience :: Developers',
        'License :: OSI Approved :: Apache Software License',
        'Programming Language :: Python :: 3',
        'Programming Language :: Python :: 3.7',
        'Programming Language :: Python :: 3.8',
        'Programming Language :: Python :: 3.9',
        'Programming Language :: Python :: 3.10',
        'Programming Language :: Python :: 3.11',
    ],
    python_requires='>=3.7',
)
