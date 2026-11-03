mod lz77;
mod utils;

// the LZ77 output format
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Lit(u8),
    Match { len: u16, dist: u16 },
}

pub fn symbols_to_tokens(symbols: &Vec<u16>) -> Vec<Token> {
    let mut tokens = vec![];
    let mut i = 0;

    while i < symbols.len() {
        let symbol = symbols[i];

        if symbol < 256 {
            tokens.push(Token::Lit(symbol as u8));
        } else {
            let len = symbols[i] - 256;
            let dist = symbols[i + 1] - 1000;

            tokens.push(Token::Match { len, dist });

            // skip dist
            i += 1;
        }

        i += 1;
    }

    tokens
}

pub fn tokens_to_symbols(tokens: &Vec<Token>) -> Vec<u16> {
    let mut output = Vec::new();

    for token in tokens {
        match token {
            Token::Lit(b) => output.push(*b as u16),
            Token::Match { len, dist } => {
                output.push(256 + len);
                output.push(1000 + dist);
            }
        }
    }

    output
}

pub const MAX_SIZE: usize = 32768; // 32KB

pub fn compress(input: &[u8]) -> Vec<Token> {
    let compressed = lz77::compress(input);

    compressed
}

pub fn decompress(compressed_input: &Vec<Token>) -> Result<String, String> {
    let bytes = lz77::decompress(compressed_input);
    String::from_utf8(bytes).map_err(|_| format!("Couldn't create string from bytes"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compress_and_decompress_text_with_repeated_sequences() {
        let input = b"ABABABCABCABC";
        let tokens = compress(input);

        assert!(tokens.iter().any(|token| matches!(token, Token::Match { .. })));
        assert_eq!(decompress(&tokens).unwrap().as_bytes(), input);
    }

    #[test]
    fn empty_input_round_trips() {
        let tokens = compress(b"");

        assert!(tokens.is_empty());
        assert_eq!(decompress(&tokens).unwrap(), "");
    }

    #[test]
    fn decompression_rejects_invalid_utf8() {
        let tokens = vec![Token::Lit(0xff)];

        assert!(decompress(&tokens).is_err());
    }

    #[test]
    fn tokens_round_trip_through_huffman_symbols() {
        let tokens = vec![
            Token::Lit(b'A'),
            Token::Match { len: 3, dist: 1 },
            Token::Lit(b'!'),
        ];

        assert_eq!(symbols_to_tokens(&tokens_to_symbols(&tokens)), tokens);
    }
}
