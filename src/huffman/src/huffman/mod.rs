use bitvec::prelude::*;
use std::collections::HashMap;

mod huffman_node;
use huffman_node::HuffmanNode;


#[derive(Debug)]
pub struct Huffman {
    pub table: HashMap<u8, BitVec<u8>>,
    pub counts: HashMap<u8, u16>
}

impl Huffman{
    pub fn from_decoded(input: &Vec<u8>) -> Self {
        let counts = Self::count_chars(&input);
        let root = HuffmanNode::new(&counts);
        
        let mut huffman = Huffman {
            table: HashMap::new(),
            counts
        };

        huffman.make_table(Box::new(root),
        &mut BitVec::<u8>::new());

        huffman
    }

    pub fn count_chars(input: &Vec<u8>) -> HashMap<u8, u16> {
        let mut counts = HashMap::new();
        input.iter().for_each(|char| {
            if counts.contains_key(char) {
                let char_freq = counts.get_mut(char).unwrap();
                *char_freq += 1;
            }else {
                counts.insert(char.clone(), 1);
            }
        });

        counts
    }

    pub fn from_encoded(counts: HashMap<u8, u16>) -> Self {
        let root = HuffmanNode::new(&counts);
        let mut huffman = Huffman {
            table: HashMap::new(),
            counts
        };

        huffman.make_table(Box::new(root),
        &mut BitVec::<u8>::new());

        huffman
    }

    pub fn make_table(&mut self, curr_node: Box<HuffmanNode>, 
        code: &mut BitVec<u8>) {
        if let Some(byte) = curr_node.byte {
            // Single character case: assign a code of [false] if code is empty
            if code.is_empty() {
                let mut zero_bit = BitVec::<u8>::new();
                zero_bit.push(false);
                
                self.table.insert(byte, zero_bit);
            } else {
                self.table.insert(byte, code.clone());
            }

            return
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
    
    pub fn encode(&mut self, input: &Vec<u8>) -> BitVec<u8> {
        let mut bits = BitVec::<u8>::new();

        input.iter().for_each(|i_byte| {
            let code = self.table
                .get(&i_byte)
                .unwrap();
            
            bits.extend(code.iter());
        });
    
        bits
    }
    
    pub fn decode(&mut self, encoded: &BitVec<u8>) -> Vec<u8> {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_roundtrip() {
        let input = b"hello world".to_vec();
        let mut huffman = Huffman::from_decoded(&input);

        let encoded = huffman.encode(&input);
        assert!(!encoded.is_empty(), "encoded output should not be empty");

        let decoded = huffman.decode(&encoded);
        assert_eq!(decoded, input);
    }

    #[test]
    fn single_character_input() {
        let input = b"aaaaaa".to_vec();
        let mut huffman = Huffman::from_decoded(&input);

        let encoded = huffman.encode(&input);
        let decoded = huffman.decode(&encoded);

        assert_eq!(decoded, input);
    }

    #[test]
    fn empty_input() {
        let input: Vec<u8> = vec![];
        let mut huffman = Huffman::from_decoded(&input);

        let encoded = huffman.encode(&input);
        let decoded = huffman.decode(&encoded);

        assert!(encoded.is_empty());
        assert!(decoded.is_empty());
    }

    #[test]
    fn all_unique_characters() {
        let input = b"abcdefg".to_vec();
        let mut huffman = Huffman::from_decoded(&input);

        let encoded = huffman.encode(&input);
        let decoded = huffman.decode(&encoded);

        assert_eq!(decoded, input);
    }

    #[test]
    fn table_contains_all_input_bytes() {
        let input = b"mississippi".to_vec();
        let huffman = Huffman::from_decoded(&input);

        for byte in &input {
            assert!(
                huffman.table.contains_key(byte),
                "missing Huffman code for byte {}",
                byte
            );
        }
    }

    #[test]
    fn no_empty_codes_in_table() {
        let input = b"hello".to_vec();
        let huffman = Huffman::from_decoded(&input);

        for (byte, code) in &huffman.table {
            assert!(
                !code.is_empty(),
                "byte {} has an empty Huffman code",
                byte
            );
        }
    }

    #[test]
    fn encoding_is_deterministic() {
        let input = b"banana".to_vec();

        let mut h1 = Huffman::from_decoded(&input);
        let mut h2 = Huffman::from_decoded(&input);

        let b1 = h1.encode(&input);
        let b2 = h2.encode(&input);

        assert_eq!(b1, b2);
    }

}
