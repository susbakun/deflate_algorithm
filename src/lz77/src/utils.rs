use super::*;

pub fn find_longest_match(input: &[u8], current_idx: usize) -> Option<(u16, u16)> {
    if current_idx == 0 {
        return None;
    }

    let window_start = current_idx.saturating_sub(MAX_SIZE);
    let mut best_match_len = 0;
    let mut best_match_dist = 0;

    // Try to find the longest match starting at each position in the search buffer
    for start_pos in window_start..current_idx {
        let mut len = 0;

        // Continue if in-progress match is not good enough
        if current_idx + best_match_len < input.len()
            && input[start_pos + best_match_len] != input[current_idx + best_match_len]
        {
            continue;
        }

        while current_idx + len < input.len()
            && len < 258
            && input[start_pos + len] == input[current_idx + len]
        {
            len += 1;
        }

        if len > best_match_len {
            best_match_len = len;
            best_match_dist = current_idx - start_pos;
        }

        // No other matches can beat this one
        if len == 258 {
            break;
        }
    }

    // Only return a match if it's at least 3 characters (to make it worthwhile)
    (best_match_len >= 3).then(|| (best_match_dist as u16, best_match_len as u16))
}
