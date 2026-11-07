use bitvec::prelude::*;
use std::collections::HashMap;

mod huffman_node;
use huffman_node::HuffmanNode;

use crate::Sym;

#[derive(Debug)]
pub struct Huffman {
    pub table: HashMap<u16, BitVec<u8>>,
    pub counts: HashMap<u16, u32>,
}

impl Huffman {
    pub fn from_decoded(input: &[Sym]) -> (Self, HuffmanNode) {
        let counts = Self::count_symbols(&input);
        let root = HuffmanNode::new(&counts);

        let mut huffman = Huffman {
            table: HashMap::new(),
            counts,
        };

        huffman.make_table(Box::new(root.clone()), &mut BitVec::<u8>::new());

        (huffman, root)
    }

    pub fn count_symbols(input: &[Sym]) -> HashMap<u16, u32> {
        let mut counts: HashMap<u16, u32> = HashMap::new();
        for sym in input {
            if counts.contains_key(&sym.symbol) {
                let char_freq = counts.get_mut(&sym.symbol).unwrap();
                *char_freq += 1;
            } else {
                counts.insert(sym.symbol, 1);
            }
        }

        counts
    }

    // We need root for decode
    pub fn from_encoded(counts: HashMap<u16, u32>) -> (Self, HuffmanNode) {
        let root = HuffmanNode::new(&counts);
        let mut huffman = Huffman {
            table: HashMap::new(),
            counts,
        };

        huffman.make_table(Box::new(root.clone()), &mut BitVec::<u8>::new());

        (huffman, root)
    }

    pub fn make_table(&mut self, curr_node: Box<HuffmanNode>, code: &mut BitVec<u8>) {
        if let Some(byte) = curr_node.symbol {
            // Single character case: assign a code of [false] if code is empty
            if code.is_empty() {
                let mut zero_bit = BitVec::<u8>::new();
                zero_bit.push(false);

                self.table.insert(byte, zero_bit);
            } else {
                self.table.insert(byte, code.clone());
            }

            return;
        }

        if let Some(left) = curr_node.left {
            code.push(false);
            self.make_table(left, code);
            code.pop();
        }

        if let Some(right) = curr_node.right {
            code.push(true);
            self.make_table(right, code);
            code.pop();
        }
    }

    pub fn encode(&mut self, input: &[Sym]) -> BitVec<u8> {
        let mut bits = BitVec::<u8>::new();

        for s in input {
            bits.extend(self.table[&s.symbol].iter());

            for i in 0..s.extra_bits {
                bits.push((s.extra_value >> i) & 1 == 1);
            }
        }

        bits
    }

    pub fn decode(
        &mut self,
        encoded: &BitVec<u8>,
        huffman_root: &HuffmanNode,
        extra_bits_for: impl Fn(u16) -> u32,
    ) -> Option<Vec<Sym>> {
        let mut output = Vec::new();
        let single_symbol = huffman_root.left.is_none() && huffman_root.right.is_none();
        let mut current_node = huffman_root;

        let mut i = 0;
        while i < encoded.len() {
            let bit = encoded[i];
            i += 1;

            if !single_symbol {
                current_node = if bit {
                    current_node.right.as_deref()?
                } else {
                    current_node.left.as_deref()?
                };
            }

            // We're on leaf restart from root
            if current_node.left.is_none() && current_node.right.is_none() {
                let sym = calc_sym(&current_node, &encoded, &mut i, &extra_bits_for)?;
                output.push(sym);
                current_node = huffman_root;
            }
        }
        // Making sure that we're on the root (no incompatible symbols)
        if current_node != huffman_root {
            return None;
        }

        Some(output)
    }
}

fn calc_sym(
    current_node: &HuffmanNode,
    encoded: &BitVec<u8>,
    current_idx: &mut usize,
    extra_bits_for: impl Fn(u16) -> u32,
) -> Option<Sym> {
    let symbol = current_node.symbol?;
    let extra_bits = extra_bits_for(symbol);
    let mut extra_value = 0u32;

    for b in 0..extra_bits {
        if *current_idx >= encoded.len() {
            return None;
        }

        if encoded[*current_idx] {
            extra_value |= 1 << b;
        }

        *current_idx += 1;
    }

    Some(Sym {
        symbol,
        extra_value,
        extra_bits,
    })
}
