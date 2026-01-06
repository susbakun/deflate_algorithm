#![allow(dead_code)]
use std::collections::HashMap;
use std::path::PathBuf;

mod huffman;
pub use huffman::Huffman;
mod io;
pub use io::*;


pub fn encode(input: &Vec<u8>, output_path: &String) {
    let mut huffman = Huffman::from_decoded(input);
    let encoded = huffman.encode(input);

    // compute symbol frequencies to store in the header
    let mut counts: HashMap<u8, u16> = HashMap::new();
    for b in input {
        *counts.entry(*b).or_insert(0) += 1;
    }

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