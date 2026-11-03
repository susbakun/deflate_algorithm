mod lz77;
mod utils;

// the LZ77 output format
#[derive(Debug)]
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

#[test]
fn dist_roundtrip() {
    for d in 1..=32768u32 {
        let (code, extra, _) = dist_to_code(d as u16);
        assert_eq!(code_to_dist(code, extra) as u32, d);
    }
}
