use huffman::Sym;
use lz77::Token;

const LEN_BASE: [u16; 29] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258,
];
const LEN_EXTRA: [u32; 29] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
];
const DIST_BASE: [u16; 30] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537,
    2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577,
];
const DIST_EXTRA: [u32; 30] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13,
    13,
];

const LEN_START: u16 = 257;
const DIST_START: u16 = 286;

// which bucket a value falls in: the last one whose base <= value
fn bucket(base: &[u16], value: u16) -> usize {
    let index = base.partition_point(|b| *b <= value);
    assert!(index > 0, "No bucket for value: {value}");
    index - 1
}

// how many raw bits follow a symbol (the decoder needs this)
pub fn extra_bits_for(symbol: u16) -> u32 {
    match symbol {
        257..=285 => LEN_EXTRA[(symbol - LEN_START) as usize],
        286..=315 => DIST_EXTRA[(symbol - DIST_START) as usize],
        _ => 0,
    }
}

pub fn tokens_to_syms(tokens: &[Token]) -> Vec<Sym> {
    let mut syms = Vec::with_capacity(tokens.len() * 2);
    for token in tokens {
        match token {
            Token::Lit(b) => syms.push(Sym {
                symbol: *b as u16,
                extra_value: 0,
                extra_bits: 0,
            }),
            Token::Match { len, dist } => {
                let lc = bucket(&LEN_BASE, *len);
                syms.push(Sym {
                    symbol: LEN_START + lc as u16,
                    extra_value: (*len - LEN_BASE[lc]) as u32,
                    extra_bits: LEN_EXTRA[lc],
                });
                let dc = bucket(&DIST_BASE, *dist);
                syms.push(Sym {
                    symbol: DIST_START + dc as u16,
                    extra_value: (*dist - DIST_BASE[dc]) as u32,
                    extra_bits: DIST_EXTRA[dc],
                })
            }
        }
    }

    syms
}

pub fn syms_to_tokens(syms: &[Sym]) -> Option<Vec<Token>> {
    let mut tokens = Vec::with_capacity(syms.len());
    let mut i = 0;
    while i < syms.len() {
        let sym = &syms[i];
        match sym.symbol {
            0..=255 => {
                tokens.push(Token::Lit(sym.symbol as u8));
                i += 1;
            }
            257..=285 => {
                let len = sym.extra_value as u16 + LEN_BASE[(sym.symbol - LEN_START) as usize];
                let d = syms.get(i + 1)?; // a distance must follow length
                if !(286..=315).contains(&d.symbol) {
                    return None;
                }
                let dist = d.extra_value as u16 + DIST_BASE[(d.symbol - DIST_START) as usize];
                tokens.push(Token::Match { len, dist });
                i += 2;
            }
            _ => return None,
        }
    }

    Some(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_length_and_distance_roundtrips() {
        for len in 3..=258u16 {
            for dist in [1u16, 2, 100, 5000, 32768] {
                let t = vec![Token::Match { len, dist }];
                assert_eq!(syms_to_tokens(&tokens_to_syms(&t)).unwrap(), t);
            }
        }
        for dist in 1..=32768u16 {
            let t = vec![Token::Match { len: 3, dist }];
            assert_eq!(syms_to_tokens(&tokens_to_syms(&t)).unwrap(), t);
        }
    }
}
