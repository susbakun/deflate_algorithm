use super::Token;
use super::utils::*;

pub fn compress(input: &[u8]) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut search_buffer: Vec<u8> = Vec::new();
    // Convert input to Vec<char> for efficient indexing
    let mut i = 0;

    while i < input.len() {
        let matched = find_longest_match(&search_buffer, &input, i);

        if let Some((offset, length)) = matched {
            // Add the match to output
            tokens.push(Token::Match {
                len: length as u16,
                dist: offset as u16,
            });

            // Add matched characters to search buffer
            for j in 0..length {
                push_to_buffer(&mut search_buffer, input[i + j]);
            }

            // Also add the next character if it exists
            i += length;
        } else {
            // no match found, output literal
            tokens.push(Token::Lit(input[i]));
            push_to_buffer(&mut search_buffer, input[i]);
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
