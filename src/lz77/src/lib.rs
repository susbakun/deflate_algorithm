mod lz77;
mod utils;

// the LZ77 output format
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Lit(u8),
    Match { len: u16, dist: u16 },
}

pub const MAX_SIZE: usize = 32768; // 32KB

pub fn compress(input: &[u8]) -> Vec<Token> {
    let compressed = lz77::compress(input);

    compressed
}

pub fn decompress(compressed_input: &Vec<Token>) -> Vec<u8> {
    let bytes = lz77::decompress(compressed_input);
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compress_and_decompress_text_with_repeated_sequences() {
        let input = b"ABABABCABCABC";
        let tokens = compress(input);

        assert!(
            tokens
                .iter()
                .any(|token| matches!(token, Token::Match { .. }))
        );
        assert_eq!(decompress(&tokens), input);
    }

    #[test]
    fn empty_input_round_trips() {
        let tokens = compress(b"");

        assert!(tokens.is_empty());
        assert!(decompress(&tokens).is_empty());
    }

    #[test]
    fn decompression_accepts_arbitrary_bytes() {
        let tokens = vec![Token::Lit(0xff), Token::Lit(0x00)];

        assert_eq!(decompress(&tokens), vec![0xff, 0x00]);
    }

    #[test]
    fn overlapping_match_encodes_a_run() {
        let tokens = compress(b"aaaaaaaaaa");

        assert_eq!(
            tokens,
            vec![Token::Lit(b'a'), Token::Match { len: 9, dist: 1 }]
        );
        assert_eq!(decompress(&tokens), b"aaaaaaaaaa");
    }

    #[test]
    fn long_run_is_split_at_max_match_length() {
        let input = vec![b'x'; 1000];
        let tokens = compress(&input);

        for token in &tokens {
            if let Token::Match { len, dist } = token {
                assert!((3..=258).contains(len));
                assert!((1..=32768).contains(dist));
            }
        }
        assert_eq!(decompress(&tokens), input);
    }

    #[test]
    fn binary_data_round_trips() {
        let mut input: Vec<u8> = (0..=255u8).cycle().take(2000).collect();
        let mut x: u32 = 12345;
        for _ in 0..1000 {
            x = x.wrapping_mul(1103515245).wrapping_add(12345);
            input.push((x >> 16) as u8);
        }

        assert_eq!(decompress(&compress(&input)), input);
    }
}
