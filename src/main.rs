#![allow(dead_code)]
mod commandline;
mod utils;

use commandline::CommandLine;
use lz77::{symbols_to_tokens, tokens_to_symbols};
use std::fs;

fn main() {
    let commandline = CommandLine::new();

    let input_path = commandline.str_arg("i", "resources/L.txt");

    let output_path = commandline.str_arg("o", "output.dfl");

    let should_compress = commandline.is_present("c");
    let should_decompress = commandline.is_present("d");

    if should_compress {
        let input = fs::read(input_path).expect("Failed to read the decompressed file");

        let lz_encoded = lz77::compress(&input);
        let symbols = tokens_to_symbols(&lz_encoded);
        huffman::encode(&symbols, &output_path);
    } else if should_decompress {
        let huffman_decoded = huffman::decode(&input_path);
        let tokens = symbols_to_tokens(&huffman_decoded);
        let decompressed = lz77::decompress(&tokens).expect("Failed to read the compressed file");

        fs::write(&output_path, decompressed).expect("Failed to write the decompressed file");

        println!("Successfully decompressed and written to {output_path}");
    }
}
