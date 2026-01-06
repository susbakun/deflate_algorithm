mod utils;
pub use utils::is_literal;
mod lz77;

// (offset, length, next_char) - the LZ77 output format
pub type Triple = (u16, u16, Option<char>);

pub const MAX_SIZE: usize = 4096;

pub fn compress(input: &String) -> Vec<Triple>{    
    let compressed = lz77::compress(input);

    compressed
}

pub fn decompress(compressed_input: &Vec<Triple>) -> String {
    lz77::decompress(compressed_input)
}