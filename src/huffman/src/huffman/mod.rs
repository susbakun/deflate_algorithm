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
    pub fn from_decoded(input: &Vec<u16>) -> Self {
        let counts = Self::count_symbols(&input);
        let root = HuffmanNode::new(&counts);

        let mut huffman = Huffman {
            table: HashMap::new(),
            counts,
        };

        huffman.make_table(Box::new(root), &mut BitVec::<u8>::new());

        huffman
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

    pub fn from_encoded(counts: HashMap<u16, u32>) -> Self {
        let root = HuffmanNode::new(&counts);
        let mut huffman = Huffman {
            table: HashMap::new(),
            counts,
        };

        huffman.make_table(Box::new(root), &mut BitVec::<u8>::new());

        huffman
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

    pub fn decode(&mut self, encoded: &BitVec<u8>) -> Vec<u16> {
        let mut current = bitvec![];
        let mut output = Vec::new();

        for i in 0..encoded.len() {
            let bit = encoded[i];
            current.push(bit);

            self.table
                .iter()
                .find(|entry| entry.1.eq(&current))
                .and_then(|entry| {
                    current.clear();
                    let byte = *entry.0;
                    output.push(byte);
                    Some(())
                });
        }

        output
    }
}
