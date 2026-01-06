# Huffman

A Rust library for Huffman coding - a lossless data compression algorithm using variable-length prefix codes.

## Overview

This library provides:
- Huffman tree construction from byte frequency counts
- Encoding bytes to variable-length bit sequences
- Decoding bit sequences back to bytes
- File I/O with embedded frequency tables for standalone decompression

## Usage

```rust
use huffman::{encode, decode};

// Compress data to file
let data: Vec<u8> = b"hello world".to_vec();
encode(&data, &"output.huf".to_string());

// Decompress from file
let decoded = decode(&"output.huf".to_string());
```

## API

### High-level functions

```rust
// Encode bytes and write to file (includes frequency header)
pub fn encode(input: &Vec<u8>, output_path: &String)

// Read file and decode back to bytes
pub fn decode(input_path: &String) -> Vec<u8>
```

### Huffman struct

```rust
use huffman::Huffman;

// Build from raw data (computes frequencies internally)
let mut huffman = Huffman::from_decoded(&data);

// Build from pre-computed frequency counts
let mut huffman = Huffman::from_encoded(counts);

// Encode/decode
let encoded = huffman.encode(&data);
let decoded = huffman.decode(&encoded);
```

## File Format

Huffman-encoded files contain:
1. **Header** - Byte frequency table for tree reconstruction
2. **Payload** - Huffman-encoded bit stream

This allows decompression without external metadata.

## Algorithm

1. Count byte frequencies in input
2. Build a min-heap of leaf nodes (one per unique byte)
3. Repeatedly merge two lowest-frequency nodes until one tree remains
4. Traverse tree to assign prefix codes (left=0, right=1)
5. Replace each input byte with its code

Deterministic tree construction is ensured by sorting nodes by byte value when frequencies are equal.
