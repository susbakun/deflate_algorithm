#![allow(dead_code)]
use std::path::PathBuf;

mod huffman;
pub use huffman::Huffman;
mod io;
pub use io::*;

pub fn encode(input: &Vec<u16>, output_path: &String) {
    let (mut huffman, _) = Huffman::from_decoded(input);
    let encoded = huffman.encode(input);

    // compute symbol frequencies to store in the header
    let counts = huffman.counts;

    write_bits_to_file(PathBuf::from(&output_path), &encoded, &counts)
        .expect("Couldn't write to the file");

    println!("Successfully compressed and written to {output_path}");
}

pub fn decode(input_path: &String) -> Option<Vec<u16>> {
    let (counts, payload) =
        read_huffman_file(PathBuf::from(input_path)).expect("Failed to read the file");

    let (mut huffman, root) = Huffman::from_encoded(counts);
    huffman.decode(&payload, &root)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static NEXT_FILE_ID: AtomicUsize = AtomicUsize::new(0);

    fn temporary_file_path() -> PathBuf {
        let id = NEXT_FILE_ID.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "huffman-roundtrip-{}-{id}.dflt",
            std::process::id()
        ))
    }

    #[test]
    fn huffman_tree_round_trips_repeated_symbols() {
        let input = vec![7, 3, 7, 7, 12, 3, 9, 7, 12];
        let (mut huffman, root) = Huffman::from_decoded(&input);
        let encoded = huffman.encode(&input);
        let decoded = huffman.decode(&encoded, &root).unwrap();

        assert_eq!(decoded, input);
    }

    #[test]
    fn huffman_tree_round_trips_a_single_symbol() {
        let input = vec![42; 25];
        let (mut huffman, root) = Huffman::from_decoded(&input);
        let encoded = huffman.encode(&input);

        println!("{root:?}");

        let decoded = huffman.decode(&encoded, &root).unwrap();

        assert_eq!(decoded, input);
    }

    #[test]
    fn file_format_round_trips_symbols() {
        let input = vec![0, 1, 1, 2, 255, 1024, 4096, 2, 2, 2];
        let path = temporary_file_path();
        let path_string = path.to_string_lossy().into_owned();

        encode(&input, &path_string);
        let decoded = decode(&path_string).unwrap();
        assert_eq!(decoded, input);

        std::fs::remove_file(path).unwrap();
    }
}
