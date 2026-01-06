use super::Triple;
use super::utils::*;

pub fn compress(input: &str) -> Vec<Triple> {
    let mut encoded: Vec<Triple> = Vec::new();
    let mut search_buffer: Vec<char> = Vec::new();
    // Convert input to Vec<char> for efficient indexing
    let input_chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < input_chars.len() {
        let matched = find_longest_match(
            &search_buffer, 
            &input_chars, 
            i);

        if let Some((offset, length)) = matched {
            let next_char = if i + length < input_chars.len() {
                Some(input_chars[i + length])
            } else {
                None
            };
            
            // Add the match to output
            encoded.push((offset as u16, length as u16, next_char));
            
            // Add matched characters to search buffer
            for j in 0..length {
                push_to_buffer(&mut search_buffer, input_chars[i + j]);
            }
            
            // Also add the next character if it exists
            if let Some(ch) = next_char {
                push_to_buffer(&mut search_buffer, ch);
                i += length + 1;
            } else {
                i += length;
            }
        } else {
            // no match found, output literal
            let ch = input_chars[i];
            encoded.push((0, 0, Some(ch)));
            push_to_buffer(&mut search_buffer, ch);
            i += 1;
        }
    }

    encoded
}

pub fn decompress(compressed_input: &Vec<Triple>) -> String {
    let mut output: Vec<char> = Vec::new();

    for triple in compressed_input {
        let (offset, length, next_char) = triple;
        if is_literal(*triple){
            // literal character
            if let Some(ch) = next_char {
                output.push(*ch);
            }
        } else {
            let start = output.len().saturating_sub(*offset as usize);
            for i in start..(start + (*length as usize)) {
                if i < output.len() {
                    output.push(output[i]);
                }
            }
            
            // Add the next character if it exists
            if let Some(ch) = next_char {
                output.push(*ch);
            }
        }
    }

    output.into_iter().collect()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip_simple() {
        let input = "ABABABCABC";
        let compressed = compress(input);
        let decompressed = decompress(&compressed);
        assert_eq!(input, decompressed);
    }

    #[test]
    fn test_roundtrip_repeated_pattern() {
        let input = "the fox and the dog and the fox";
        let compressed = compress(input);
        let decompressed = decompress(&compressed);
        assert_eq!(input, decompressed);
    }

    #[test]
    fn test_roundtrip_no_repeats() {
        let input = "abcdefghij";
        let compressed = compress(input);
        let decompressed = decompress(&compressed);
        assert_eq!(input, decompressed);
    }

    #[test]
    fn test_roundtrip_single_char_repeat() {
        let input = "aaaaaaaaaa";
        let compressed = compress(input);
        let decompressed = decompress(&compressed);
        assert_eq!(input, decompressed);
    }

    #[test]
    fn test_roundtrip_empty() {
        let input = "";
        let compressed = compress(input);
        let decompressed = decompress(&compressed);
        assert_eq!(input, decompressed);
    }

    #[test]
    fn test_compression_reduces_size() {
        let input = "ABABABABABABABAB";
        let compressed = compress(input);
        // Should have fewer tokens than input length due to back-references
        assert!(compressed.len() < input.len());
    }
}
