use bitvec::prelude::*;
use std::collections::HashMap;

mod huffman_node;
use huffman_node::HuffmanNode;

#[derive(Debug)]
pub struct Huffman {
    pub table: HashMap<u16, BitVec<u8>>,
    pub counts: HashMap<u16, u32>,
}

impl Huffman {
    pub fn from_decoded(input: &Vec<u16>) -> (Self, HuffmanNode) {
        let counts = Self::count_symbols(&input);
        let root = HuffmanNode::new(&counts);

        let mut huffman = Huffman {
            table: HashMap::new(),
            counts,
        };

        huffman.make_table(Box::new(root.clone()), &mut BitVec::<u8>::new());

        (huffman, root)
    }

    pub fn count_symbols(input: &Vec<u16>) -> HashMap<u16, u32> {
        let mut counts: HashMap<u16, u32> = HashMap::new();
        input.iter().for_each(|symbol| {
            if counts.contains_key(&symbol) {
                let char_freq = counts.get_mut(&symbol).unwrap();
                *char_freq += 1;
            } else {
                counts.insert(*symbol, 1);
            }
        });

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

    pub fn encode(&mut self, input: &Vec<u16>) -> BitVec<u8> {
        let mut bits = BitVec::<u8>::new();

        input.iter().for_each(|symbol| {
            let code = self.table.get(&symbol).unwrap();

            bits.extend(code.iter());
        });

        bits
    }

    pub fn decode(&mut self, encoded: &BitVec<u8>, huffman_root: &HuffmanNode) -> Option<Vec<u16>> {
        let mut output = Vec::new();
        let mut current_node = huffman_root;

        for bit in encoded {
            // We only have root and can't go left or right
            if huffman_root.left.is_none() && huffman_root.right.is_none() {
                output.push(current_node.symbol?);
                continue;
            }

            current_node = if *bit {
                current_node.right.as_deref()?
            } else {
                current_node.left.as_deref()?
            };

            // We're on leaf restart from root
            if current_node.left.is_none() && current_node.right.is_none() {
                output.push(current_node.symbol?);
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
