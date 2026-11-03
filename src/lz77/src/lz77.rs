use super::Token;
use super::utils::*;

pub fn compress(input: &[u8]) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    // Convert input to Vec<char> for efficient indexing
    let mut i = 0;

    while i < input.len() {
        let matched = find_longest_match(&input, i);

        if let Some((offset, length)) = matched {
            // Add the match to output
            tokens.push(Token::Match {
                len: length as u16,
                dist: offset as u16,
            });

            i += length as usize;
        } else {
            // no match found, output literal
            tokens.push(Token::Lit(input[i]));
            i += 1;
        }
    }

    tokens
}

pub fn decompress(compressed_input: &[Token]) -> Vec<u8> {
    let mut output = Vec::new();

    for token in compressed_input {
        match token {
            Token::Lit(b) => output.push(*b),
            Token::Match { len, dist } => {
                for _ in 0..*len {
                    output.push(output[output.len() - *dist as usize]);
                }
            }
        }
    }

    output
}
