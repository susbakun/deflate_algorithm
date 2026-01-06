use std::io::Read;
use anyhow::{bail, Result};

pub fn read_to_buffer<R: Read>(reader: &mut R, buffer: &mut [u8], error_msg: &str) -> Result<()> {
    let expected_len = buffer.len();
    let bytes_read = reader.read(buffer)?;
    
    if bytes_read != expected_len {
        bail!("{}: expected {} bytes, got {}", error_msg, expected_len, bytes_read);
    }
    
    Ok(())
}