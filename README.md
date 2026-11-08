# Deflate

A Rust command-line file compressor that combines LZ77 matching with Huffman coding. It reads input as bytes, represents LZ77 output as literal and match tokens, maps lengths and distances to bucket symbols plus raw extra bits, and Huffman-encodes the symbols into a file. It works on any file type, not only text.

## Build

Requires Rust and Cargo. Build the project with:

```bash
cargo build --release
```

## Usage

Compress a file:

```bash
cargo run --release -- --compress --input input.txt --output output.dfl
```

Decompress it:

```bash
cargo run --release -- --decompress --input output.dfl --output restored.txt
```

Short flags are also available: `-c`, `-d`, `-i`, and `-o`. If omitted, the input path defaults to `resources/L.txt` and the output path defaults to `output.dfl`.

| Option                  | Description               |
| ----------------------- | ------------------------- |
| `-c`, `--compress`      | Compress the input file   |
| `-d`, `--decompress`    | Decompress the input file |
| `-i`, `--input <path>`  | Input file path           |
| `-o`, `--output <path>` | Output file path          |
| `-h`, `--help`          | Print command-line usage  |

Choose one operation (`--compress` or `--decompress`) per run.

## Compression result

On `mobby_dick.txt` (1,253,984 bytes):

| Compressed size | Reduction |
| --------------- | --------- |
| ~591,000 bytes  | ~53%      |

For reference, `gzip` produces about 508 KB on the same text.

## How it works

1. The LZ77 encoder searches up to 32 KiB behind the current position and emits either a literal byte or a `(length, distance)` match. Matches are limited to 258 bytes and are emitted when at least 3 bytes long. Matches may overlap the current position, so runs of one byte are encoded compactly.
2. The `bucket` module turns tokens into symbols. Literals use their byte value. A match becomes a length symbol and a distance symbol, each followed by raw "extra bits" that give the exact value inside its bucket. This is the same scheme DEFLATE uses (29 length buckets, 30 distance buckets).

   | Symbols | Meaning                            |
   | ------- | ---------------------------------- |
   | 0-255   | literal bytes                      |
   | 256     | unused (reserved for end-of-block) |
   | 257-285 | length buckets                     |
   | 286-315 | distance buckets                   |

3. The Huffman encoder builds a tree from the frequencies of the symbols only (never the extra bits). It writes each symbol's Huffman code and then its extra bits as plain bits, lowest bit first. The symbol frequency table is stored in the file header so the decoder can rebuild the same tree.
4. Decompression walks the Huffman tree bit by bit. After each symbol it reads the right number of extra bits for that symbol, rebuilds the tokens, and expands them into the original bytes. The output is raw bytes, so binary files round-trip exactly.

## Project layout

```text
src/
├── main.rs             # CLI compression and decompression flow
├── lib.rs              # Exposes the bucket module for integration tests
├── bucket.rs           # Token <-> symbol conversion (length/distance buckets, extra bits)
├── commandline.rs      # Command-line options
├── utils.rs            # Shared I/O helpers
├── lz77/               # LZ77 token encoding and decoding library
└── huffman/            # Huffman coding and compressed-file I/O library
tests/                  # Integration tests for the full pipeline
```

The `lz77` and `huffman` directories are local Cargo crates used by the command-line application.

## Testing

```bash
cargo test --workspace
```

Add `-p lz77` or `-p huffman` to test a single crate. Tests cover the bucket conversions for every length and distance, Huffman round trips with extra bits, and the full compress/decompress pipeline on text and binary data.

## Dependencies

- `anyhow` for error handling
- `bitvec` for Huffman bit streams
- `getopts` for command-line parsing

## TODOS

- [ ] This program is relatively slow, need to find a way to make it faster
- [x] Using buckets we can improve the effectiveness of the algorithm
