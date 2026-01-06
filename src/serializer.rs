use std::io::{BufReader, Read};

use lz77::{Triple, is_literal};
use anyhow::{bail, Result};

use super::utils;

pub fn serialize(lz_encoded: &Vec<Triple>) -> Vec<u8>{
    let mut serialized = Vec::new();
    for triple in lz_encoded.iter(){
        if is_literal(*triple){
            serialized.push(0);
            let byte = triple.2
                .expect("There must be a char") as u8;
            serialized.push(byte);
        } else {
            let next_char = if let Some(ch) = triple.2 {
                Some(ch)
            } else {
                None
            };

            if let Some(_) = next_char {
                serialized.push(1);
            } else {
                serialized.push(2);
            }

            let offset = triple.0
                    .to_le_bytes();

            let length = triple.1
                .to_le_bytes();

            serialized.extend(offset.iter());
            serialized.extend(length.iter());

            if let Some(next_char) = next_char {
                let byte= next_char as u8;
                serialized.push(byte);
            }
        }
    }

    serialized
}

pub fn deserialize(huffman_decoded: &Vec<u8>) -> Result<Vec<Triple>, anyhow::Error>{
    let mut result = Vec::new();
    if huffman_decoded.len() == 0 {
        bail!("deserialization failed");
    }

    let huffman_decoded = huffman_decoded.as_slice();

    let mut buf_reader = BufReader::new(huffman_decoded);

    loop {
        let mut flag = [0u8;1];
        if buf_reader.read(&mut flag)? == 0 {
            break; // EOF
        }
        let flag = u8::from_le_bytes(flag);

        match flag{
            0 => {
                let mut byte = [0u8;1];
                utils::read_to_buffer(
                    &mut buf_reader, 
                    &mut byte, 
                    "Unexpected EOF while reading literal byte"
                )?;
                let char = u8::from_le_bytes(byte) as char;

                let triple = (0, 0, Some(char));
                result.push(triple);
            }
            1 => {
                let mut offset = [0u8;2];
                utils::read_to_buffer(
                    &mut buf_reader, 
                    &mut offset, 
                    "Unexpected EOF while reading offset"
                )?;
                let offset = u16::from_le_bytes(offset);

                let mut length = [0u8;2];
                utils::read_to_buffer(
                    &mut buf_reader, 
                    &mut length, 
                    "Unexpected EOF while reading length"
                )?;
                let length = u16::from_le_bytes(length);

                let mut byte = [0u8;1];
                utils::read_to_buffer(
                    &mut buf_reader, 
                    &mut byte, 
                    "Unexpected EOF while reading next char"
                )?;
                let char = u8::from_le_bytes(byte) as char;
                let triple = (offset, length, Some(char));

                result.push(triple);
            }
            2 => {
                let mut offset = [0u8;2];
                utils::read_to_buffer(
                    &mut buf_reader, 
                    &mut offset, 
                    "Unexpected EOF while reading offset"
                )?;
                let offset = u16::from_le_bytes(offset);

                let mut length = [0u8;2];
                utils::read_to_buffer(
                    &mut buf_reader, 
                    &mut length, 
                    "Unexpected EOF while reading length"
                )?;
                let length = u16::from_le_bytes(length);

                let triple = (offset, length, None);
                result.push(triple);

                break;
            }
            flag => bail!("Unknown flag {flag}")
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_literal_triple() {
        let triples = vec![(0u16, 0u16, Some('a'))];
        let serialized = serialize(&triples);
        
        // flag 0 + byte 'a'
        assert_eq!(serialized, vec![0, b'a']);
    }

    #[test]
    fn serialize_back_reference_with_char() {
        let triples = vec![(5u16, 3u16, Some('x'))];
        let serialized = serialize(&triples);
        
        // flag 1 + offset (2 bytes LE) + length (2 bytes LE) + char
        assert_eq!(serialized, vec![1, 5, 0, 3, 0, b'x']);
    }

    #[test]
    fn serialize_back_reference_no_char() {
        let triples = vec![(10u16, 7u16, None)];
        let serialized = serialize(&triples);
        
        // flag 2 + offset (2 bytes LE) + length (2 bytes LE)
        assert_eq!(serialized, vec![2, 10, 0, 7, 0]);
    }

    #[test]
    fn deserialize_literal_triple() {
        let data = vec![0, b'a'];
        let result = deserialize(&data).unwrap();
        
        assert_eq!(result, vec![(0, 0, Some('a'))]);
    }

    #[test]
    fn deserialize_back_reference_with_char() {
        let data = vec![1, 5, 0, 3, 0, b'x'];
        let result = deserialize(&data).unwrap();
        
        assert_eq!(result, vec![(5, 3, Some('x'))]);
    }

    #[test]
    fn deserialize_back_reference_no_char() {
        let data = vec![2, 10, 0, 7, 0];
        let result = deserialize(&data).unwrap();
        
        assert_eq!(result, vec![(10, 7, None)]);
    }

    #[test]
    fn roundtrip_mixed_triples() {
        let triples = vec![
            (0, 0, Some('h')),
            (0, 0, Some('e')),
            (0, 0, Some('l')),
            (0, 0, Some('l')),
            (0, 0, Some('o')),
            (5, 3, Some('!')),
            (2, 4, None),
        ];
        
        let serialized = serialize(&triples);
        let deserialized = deserialize(&serialized).unwrap();
        
        assert_eq!(deserialized, triples);
    }

    #[test]
    fn deserialize_empty_input_fails() {
        let data: Vec<u8> = vec![];
        let result = deserialize(&data);
        
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_unknown_flag_fails() {
        let data = vec![99, 0, 0]; // flag 99 is invalid
        let result = deserialize(&data);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unknown flag 99"));
    }

    #[test]
    fn roundtrip_large_offset_and_length() {
        let triples = vec![(4000u16, 255u16, Some('z'))];
        
        let serialized = serialize(&triples);
        let deserialized = deserialize(&serialized).unwrap();
        
        assert_eq!(deserialized, triples);
    }
}
