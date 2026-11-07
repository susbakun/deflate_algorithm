#![allow(dead_code)]
use std::path::PathBuf;

mod huffman;
pub use huffman::Huffman;
mod io;
pub use io::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sym {
    pub symbol: u16,
    pub extra_value: u32,
    pub extra_bits: u32,
}

pub fn encode(input: &[Sym], output_path: &String) {
    let (mut huffman, _) = Huffman::from_decoded(input);
    let encoded = huffman.encode(input);

    write_bits_to_file(PathBuf::from(&output_path), &encoded, &huffman.counts)
        .expect("Couldn't write to the file");

    println!("Successfully compressed and written to {output_path}");
}

pub fn decode(input_path: &String, extra_bits_for: impl Fn(u16) -> u32) -> Option<Vec<Sym>> {
    let (counts, payload) =
        read_huffman_file(PathBuf::from(input_path)).expect("Failed to read the file");

    let (mut huffman, root) = Huffman::from_encoded(counts);
    huffman.decode(&payload, &root, extra_bits_for)
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

    // fake rule for the tests: symbols >= 286 are followed by 3 raw bits
    fn extra_bits_for(symbol: u16) -> u32 {
        if symbol >= 286 { 3 } else { 0 }
    }

    // a symbol with no extra bits
    fn plain(symbol: u16) -> Sym {
        Sym {
            symbol,
            extra_value: 0,
            extra_bits: 0,
        }
    }

    // a symbol with 3 extra bits holding `value` (0..=7)
    fn with_extra(symbol: u16, value: u32) -> Sym {
        Sym {
            symbol,
            extra_value: value,
            extra_bits: 3,
        }
    }

    #[test]
    fn huffman_tree_round_trips_repeated_symbols() {
        let input: Vec<Sym> = [7, 3, 7, 7, 12, 3, 9, 7, 12]
            .into_iter()
            .map(plain)
            .collect();
        let (mut huffman, root) = Huffman::from_decoded(&input);
        let encoded = huffman.encode(&input);
        let decoded = huffman.decode(&encoded, &root, extra_bits_for).unwrap();

        assert_eq!(decoded, input);
    }

    #[test]
    fn huffman_tree_round_trips_a_single_symbol() {
        let input: Vec<Sym> = vec![plain(42); 25];
        let (mut huffman, root) = Huffman::from_decoded(&input);
        let encoded = huffman.encode(&input);

        let decoded = huffman.decode(&encoded, &root, extra_bits_for).unwrap();

        assert_eq!(decoded, input);
    }

    #[test]
    fn extra_bits_survive_the_round_trip() {
        // same symbol (300) with different extra values, mixed with plain symbols
        let input = vec![
            plain(5),
            with_extra(300, 0),
            with_extra(300, 5),
            plain(5),
            with_extra(301, 7),
            with_extra(300, 3),
            plain(9),
        ];
        let (mut huffman, root) = Huffman::from_decoded(&input);
        let encoded = huffman.encode(&input);
        let decoded = huffman.decode(&encoded, &root, extra_bits_for).unwrap();

        assert_eq!(decoded, input);
    }

    #[test]
    fn extra_bits_do_not_change_the_counts() {
        // only the symbol is counted, so 300 appears 3 times here
        let input = vec![
            with_extra(300, 1),
            with_extra(300, 2),
            with_extra(300, 6),
            plain(1),
        ];
        let counts = Huffman::count_symbols(&input);

        assert_eq!(counts.get(&300), Some(&3));
        assert_eq!(counts.get(&1), Some(&1));
        assert_eq!(counts.len(), 2);
    }

    #[test]
    fn file_format_round_trips_symbols() {
        let input = vec![
            plain(0),
            plain(1),
            plain(1),
            plain(2),
            plain(255),
            with_extra(290, 4),
            with_extra(315, 7),
            plain(2),
            plain(2),
            plain(2),
        ];
        let path = temporary_file_path();
        let path_string = path.to_string_lossy().into_owned();

        encode(&input, &path_string);
        let decoded = decode(&path_string, extra_bits_for).unwrap();
        assert_eq!(decoded, input);

        std::fs::remove_file(path).unwrap();
    }
}
