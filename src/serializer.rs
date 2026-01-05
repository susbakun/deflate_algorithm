use lz77::{Triple, is_literal};

pub fn serialize(lz_output: &Vec<Triple>) -> Vec<u8>{
    let mut serialized = Vec::new();
    for triple in lz_output.iter(){
        if is_literal(*triple){
            serialized.push(0);
            let byte = triple.2
                .expect("There must be a char") as u8;
            serialized.push(byte);
        } else {
            if let Some(_) = triple.2 {
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
        }
    }

    serialized
}

pub fn deserialize(){
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_literal() {
        // (0, 0, Some('a')) is a literal
        let input = vec![(0, 0, Some('a'))];
        let result = serialize(&input);
        
        assert_eq!(result[0], 0x00); // literal flag
        assert_eq!(result[1], b'a'); // the byte
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_serialize_back_reference_with_char() {
        // (5, 3, Some('x')) is a back-ref followed by literal 'x'
        let input = vec![(5, 3, Some('x'))];
        let result = serialize(&input);
        
        assert_eq!(result[0], 0x01); // back-ref flag
        
        let offset_bytes = 5u16.to_le_bytes();
        let length_bytes = 3u16.to_le_bytes();
        
        assert_eq!(&result[1..3], &offset_bytes);
        assert_eq!(&result[3..5], &length_bytes);
        assert_eq!(result.len(), 5); // 1 flag + 2 offset + 2 length
    }

    #[test]
    fn test_serialize_back_reference_none() {
        // (20, 5, None) is a back-ref with no trailing char
        let input = vec![(20, 5, None)];
        let result = serialize(&input);
        
        assert_eq!(result[0], 0x02); // back-ref with None flag
        
        let offset_bytes = 20u16.to_le_bytes();
        let length_bytes = 5u16.to_le_bytes();
        
        assert_eq!(&result[1..3], &offset_bytes);
        assert_eq!(&result[3..5], &length_bytes);
        assert_eq!(result.len(), 5); // 1 flag + 2 offset + 2 length
    }

    #[test]
    fn test_serialize_multiple_triples() {
        let input = vec![
            (0, 0, Some('h')),  // literal
            (0, 0, Some('e')),  // literal
            (2, 1, Some('o')),  // back-ref with char
        ];
        let result = serialize(&input);
        
        // First literal: 0x00 + 'h'
        assert_eq!(result[0], 0x00);
        assert_eq!(result[1], b'h');
        
        // Second literal: 0x00 + 'e'
        assert_eq!(result[2], 0x00);
        assert_eq!(result[3], b'e');
        
        // Back-ref: 0x01 + offset + length
        assert_eq!(result[4], 0x01);
    }

    #[test]
    fn test_serialize_empty() {
        let input: Vec<Triple> = vec![];
        let result = serialize(&input);
        
        assert!(result.is_empty());
    }
}