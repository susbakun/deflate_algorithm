#![allow(dead_code)]
mod commandline;
mod utils;

use commandline::CommandLine;
use std::fs;

use deflate::bucket::{extra_bits_for, syms_to_tokens, tokens_to_syms};

fn main() {
    let commandline = CommandLine::new();

    let input_path = commandline.str_arg("i", "resources/L.txt");
    let output_path = commandline.str_arg("o", "output.dfl");

    let should_compress = commandline.is_present("c");
    let should_decompress = commandline.is_present("d");

    if should_compress {
        let input = fs::read(input_path).expect("Failed to read the decompressed file");
        let tokens = lz77::compress(&input);
        let syms = tokens_to_syms(&tokens);

        huffman::encode(&syms, &output_path);
    } else if should_decompress {
        let syms = huffman::decode(&input_path, extra_bits_for)
            .expect("No matching huffman decoding for the input");
        let tokens = syms_to_tokens(&syms).expect("No matching tokens for the provided syms");
        let decompressed = lz77::decompress(&tokens);

        fs::write(&output_path, decompressed).expect("Failed to write the decompressed file");
        println!("Successfully decompressed and written to {output_path}");
    }
}
