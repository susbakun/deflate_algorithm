
use super::*;

pub fn find_longest_match(
    search_buffer: &Vec<char>,
    input_chars: &[char],
    current_idx: usize,
) -> Option<(usize, usize)> {
    if search_buffer.is_empty() || current_idx >= input_chars.len() {
        return None;
    }

    let mut best_match_len = 0;
    let mut best_match_offset = 0;
    
    // Try to find the longest match starting at each position in the search buffer
    for start_pos in 0..search_buffer.len() {
        let mut match_len = 0;
        let mut search_idx = start_pos;
        let mut input_idx = current_idx;
        
        // Try to extend the match as long as possible
        while search_idx < search_buffer.len() 
              && input_idx < input_chars.len()
              && search_buffer[search_idx] == input_chars[input_idx] {
            match_len += 1;
            search_idx += 1;
            input_idx += 1;
        }
        
        // Update best match if this one is longer
        if match_len > best_match_len {
            best_match_len = match_len;
            // Offset is distance from end of search buffer
            best_match_offset = search_buffer.len() - start_pos;
        }
    }
    
    // Only return a match if it's at least 2 characters (to make it worthwhile)
    if best_match_len >= 4 {
        Some((best_match_offset, best_match_len))
    } else {
        None
    }
}

pub fn push_to_buffer(search_buffer: &mut Vec<char>, ch: char) {
    if search_buffer.len() >= MAX_SIZE {
        search_buffer.remove(0);
    }

    search_buffer.push(ch);
}

pub fn is_literal(triple: Triple) -> bool {
    if triple.0 == 0 {
        return true
    }
    false
}