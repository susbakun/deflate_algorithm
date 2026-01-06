# LZ77

A Rust library implementing the LZ77 lossless compression algorithm.

## Overview

LZ77 compresses data by replacing repeated sequences with back-references to earlier occurrences. Output is a sequence of triples: `(offset, length, next_char)`.

## Usage

```rust
use lz77::{compress, decompress};

let input = "ABABABCABC".to_string();
let compressed = compress(&input);
let decompressed = decompress(&compressed);

assert_eq!(input, decompressed);
```

## API

```rust
// Compress a string into LZ77 triples
pub fn compress(input: &String) -> Vec<Triple>

// Decompress triples back to string
pub fn decompress(compressed_input: &Vec<Triple>) -> String

// Check if a triple is a literal (no back-reference)
pub fn is_literal(triple: Triple) -> bool
```

## Triple Format

```rust
pub type Triple = (u16, u16, Option<char>);
//                 offset, length, next_char
```

| Type | Offset | Length | Next Char |
|------|--------|--------|-----------|
| Literal | 0 | 0 | Some(char) |
| Match | >0 | >0 | Some(char) or None |

- **Literal**: No match found, emit the character directly
- **Match**: Copy `length` bytes from `offset` positions back, then emit `next_char`

## Example

Input: `"ABABABC"`

```
'A' -> (0, 0, Some('A'))     # literal
'B' -> (0, 0, Some('B'))     # literal
'ABAB' -> (2, 4, Some('C'))  # match: go back 2, copy 4, then 'C'
```

## Constants

- `MAX_SIZE: usize = 4096` - Maximum search buffer size
