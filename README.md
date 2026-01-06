# Deflate

A Rust implementation of DEFLATE-style compression combining LZ77 and Huffman coding.

## Overview

This project implements a two-stage compression pipeline:
1. **LZ77** - Replaces repeated sequences with back-references (offset, length, next_char)
2. **Huffman Coding** - Encodes the serialized LZ77 output using variable-length prefix codes

## Usage

```bash
# Compress a file
cargo run -- -c -i input.txt -o output.dfl

# Decompress a file
cargo run -- -d -i output.dfl -o decompressed.txt
```

## Options

| Flag | Description |
|------|-------------|
| `-c` | Compress mode |
| `-d` | Decompress mode |
| `-i <path>` | Input file path (default: `resources/L.txt`) |
| `-o <path>` | Output file path (default: `output.dfl`) |

## Project Structure

```
deflate/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── serializer.rs    # LZ77 triple serialization/deserialization
│   ├── commandline.rs   # Argument parsing
│   ├── utils.rs         # Helper functions
│   ├── huffman/         # Huffman coding library
│   └── lz77/            # LZ77 compression library
└── resources/           # Sample input files
```

## How It Works

### Compression
1. Read input file as text
2. LZ77 encodes the text into triples `(offset, length, next_char)`
3. Serialize triples into a byte stream
4. Huffman encode the byte stream and write to output

### Decompression
1. Read Huffman-encoded file (includes frequency table in header)
2. Decode using Huffman tree reconstructed from frequencies
3. Deserialize byte stream back into LZ77 triples
4. Decompress triples to recover original text

## Compression Results

Tested on `resources/L.text`:

| Metric | Value |
|--------|-------|
| Original size | 1,638 bytes |
| Compressed size | 1,087 bytes |
| Compression ratio | 33.6% reduction |

## Dependencies

- `anyhow` - Error handling
- `bitvec` - Bit-level operations for Huffman coding
- `getopts` - Command-line argument parsing
