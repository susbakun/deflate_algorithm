# Deflate

A Rust command-line file compressor that combines LZ77 matching with Huffman coding. It reads input as bytes, represents LZ77 output as literal and match tokens, then Huffman-encodes the resulting symbols into a file.

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

| Option | Description |
| --- | --- |
| `-c`, `--compress` | Compress the input file |
| `-d`, `--decompress` | Decompress the input file |
| `-i`, `--input <path>` | Input file path |
| `-o`, `--output <path>` | Output file path |
| `-h`, `--help` | Print command-line usage |

Choose one operation (`--compress` or `--decompress`) per run.

## Compression result

In a run on `mobby_dick.txt`, the input was 1,253,984 bytes and the compressed output was 780,788 bytes, a reduction of about 38%.

## How it works

1. The LZ77 encoder searches up to 32 KiB behind the current position and emits either a literal byte or a `(length, distance)` match. Matches are limited to 258 bytes and are emitted when at least 3 bytes long.
2. The token stream is mapped to `u16` symbols: literals use their byte value; matches use one symbol for the length and one for the distance.
3. The Huffman encoder writes the symbol stream and its frequency table to the output file. The frequency table lets the decoder rebuild the Huffman tree.
4. Decompression reverses those steps, rebuilding the tokens and expanding them into the original bytes. The current CLI converts the result to UTF-8 text, so decompression expects valid UTF-8 input data.

## Project layout

```text
src/
├── main.rs             # CLI compression and decompression flow
├── commandline.rs      # Command-line options
├── utils.rs            # Shared I/O helpers
├── lz77/               # LZ77 token encoding and decoding library
└── huffman/            # Huffman coding and compressed-file I/O library
```

The `lz77` and `huffman` directories are local Cargo crates used by the command-line application.

## Dependencies

- `anyhow` for error handling
- `bitvec` for Huffman bit streams
- `getopts` for command-line parsing


## TODOS
- [ ] This program is relatively slow, need to find a way to make it faster
- [ ] Using buckets we can improve the effectiveness of the algorithm
