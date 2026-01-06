use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::fs::{self, File};

use anyhow::bail;
use bitvec::vec::BitVec;

const DEFLATE_FORMAT: &[u8] = "DFLT".as_bytes();
type ReadHuffmanOutput = (HashMap<u8, u16>, BitVec<u8>);


pub fn read_string_file(file_path: PathBuf) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}

pub fn write_string_to_file(file_path: PathBuf, contents: &String) -> Result<(), std::io::Error> {
    fs::write(file_path, contents)
}

pub fn read_huffman_file(file_path: PathBuf) -> Result<ReadHuffmanOutput, anyhow::Error> {
    let mut file = File::open(file_path)?;

    // header
    let mut format = [0u8; 4];
    file.read_exact(&mut format)?;
    if format != DEFLATE_FORMAT {
        bail!("File format is not accepted")
    }

    let mut num_symbols = [0u8; 2];
    file.read_exact(&mut num_symbols)?;
    let num_symbols = u16::from_le_bytes(num_symbols);

    let mut counts = HashMap::new();
    for _ in 0..num_symbols {
        let mut byte = [0u8;1];
        file.read_exact(&mut byte)?;
        let byte = u8::from_le_bytes(byte);

        let mut freq_bytes = [0u8;2];
        file.read_exact(&mut freq_bytes)?;
        let freq = u16::from_le_bytes(freq_bytes);

        counts.insert(byte, freq);
    }


    // payload
    let mut count_bytes = [0u8; 8];
    file.read_exact(&mut count_bytes)?;
    let bit_count = u64::from_le_bytes(count_bytes) as usize;

    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    let mut bitvec = BitVec::from_vec(bytes);
    bitvec.truncate(bit_count);

    Ok((counts, bitvec))
}

pub fn write_bits_to_file(
    file_path: PathBuf, 
    payload: &BitVec<u8>, 
    counts: &HashMap<u8, u16>
) -> Result<(), std::io::Error> {
    let mut bytes = Vec::new();
    let mut file = File::create(file_path)?;

    // header: magic + number of symbols + (byte, frequency) pairs
    let mut header = Vec::new();
    header.extend_from_slice(DEFLATE_FORMAT);

    let num_symbols = counts.len() as u16;
    let num_symbols = num_symbols.to_le_bytes();
    header.extend_from_slice(&num_symbols);

    for (byte, freq) in counts {
        header.push(*byte);

        let freq_bytes = freq.to_le_bytes();
        header.extend_from_slice(&freq_bytes);
    }

    file.write_all(&header)?;

    // payload bits
    let bit_count = payload.len();
    file.write_all(&bit_count.to_le_bytes())?;
    
    for chunk in payload.chunks(8) {
        let mut byte = 0u8;
        for (i, bit) in chunk.iter().enumerate() {
            if *bit {
                byte |= 1 << i;
            }
        }
        bytes.push(byte);
    }

    file.write_all(&bytes)
}