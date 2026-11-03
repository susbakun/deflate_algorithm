# LZ77

A Rust library that compresses byte slices into LZ77 literal and back-reference tokens.

## Usage

```rust
use lz77::{compress, decompress};

let input = b"ABABABC";
let tokens = compress(input);
let restored = decompress(&tokens).expect("input is valid UTF-8");

assert_eq!(restored.as_bytes(), input);
```

`compress` accepts arbitrary bytes. `decompress` currently returns a `String`, so it returns an error if the expanded bytes are not valid UTF-8.

## Token format

The encoder produces a sequence of:

```rust
pub enum Token {
    Lit(u8),
    Match { len: u16, dist: u16 },
}
```

- `Lit(byte)` stores one input byte directly.
- `Match { len, dist }` copies `len` bytes from `dist` bytes before the current output position. Copies can overlap, as in standard LZ77 decoding.

The encoder searches the previous 32 KiB for the longest match. It emits matches of at least 3 bytes, with a maximum length of 258 bytes. Otherwise it emits a literal.

## Public API

```rust
pub const MAX_SIZE: usize = 32768;

pub fn compress(input: &[u8]) -> Vec<Token>;
pub fn decompress(compressed_input: &Vec<Token>) -> Result<String, String>;

pub fn tokens_to_symbols(tokens: &Vec<Token>) -> Vec<u16>;
pub fn symbols_to_tokens(symbols: &Vec<u16>) -> Vec<Token>;
```

The symbol conversion helpers support the parent application’s Huffman stage. Literals map to their byte values; matches map to two symbols, one for length and one for distance. This representation is intended for the project’s compressor format.
