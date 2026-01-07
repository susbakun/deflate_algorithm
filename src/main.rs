#![allow(dead_code)]
mod serializer;
mod commandline;
mod utils;

use std::fs;
use serializer::{serialize, deserialize};
use commandline::CommandLine;


fn main(){
    let commandline = CommandLine::new();

    let input_path = commandline.str_arg(
        "i",
        "resources/L.txt"
        );

    let output_path = commandline.str_arg(
        "o",
        "output.dfl"
    );

    let should_compress = commandline.is_present("c");
    let should_decompress = commandline.is_present("d");

    if should_compress {
        let input = fs::read_to_string(input_path)
            .expect("Failed to read the input file");

        let lz_encoded = lz77::compress(&input);
        let serialized = serialize(&lz_encoded);
        huffman::encode(&serialized, &output_path);
    } else if should_decompress {
        let huffman_decoded = huffman::decode(&input_path);        
        let deserialized = deserialize(&huffman_decoded)
            .expect("Deserialization failed");


        let decompressed = lz77::decompress(&deserialized);

        fs::write(&output_path, decompressed)
            .expect("Failed to write the decompressed file");

        println!("Successfully decompressed and written to {output_path}");
    }

}