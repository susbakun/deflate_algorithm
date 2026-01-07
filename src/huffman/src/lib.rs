#![allow(dead_code)]
use std::path::PathBuf;

mod huffman;
pub use huffman::Huffman;
mod io;
pub use io::*;


pub fn encode(input: &Vec<u8>, output_path: &String) {
    let mut huffman = Huffman::from_decoded(input);
    let encoded = huffman.encode(input);

    // compute symbol frequencies to store in the header
    let counts = huffman.counts;

    write_bits_to_file(PathBuf::from(&output_path), &encoded, &counts)
        .expect("Couldn't write to the file");

    println!("Successfully compressed and written to {output_path}");

}

pub fn decode(input_path: &String) -> Vec<u8> {
    let (counts, payload) = read_huffman_file(PathBuf::from(input_path))
        .expect("Failed to read the file");

    let mut huffman = Huffman::from_encoded(counts);
    huffman.decode(&payload)
}