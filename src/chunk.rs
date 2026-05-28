use std::fmt::Display;
use std::io::{BufReader, Read};

use anyhow::anyhow;

use crate::chunk_type::ChunkType;

#[derive(Debug)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

#[allow(dead_code)]
impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        // Chaining chunk_type and data for CRC
        let chunk_type_and_data: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .cloned()
            .chain(data.iter().cloned())
            .collect();

        // Calculate CRC
        const X25: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let calculated_crc = X25.checksum(&chunk_type_and_data);

        Chunk {
            length: data.len() as u32,
            chunk_type,
            data,
            crc: calculated_crc,
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> crate::Result<String> {
        let result = String::from_utf8(self.data.clone())?;
        Ok(result)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        // Bytes of length
        let length_bytes: Vec<u8> = self.length.to_be_bytes().into();
        // Bytes of Chunk Type
        let chunk_type_bytes: Vec<u8> = self.chunk_type.bytes().to_vec();
        // Data bytes
        let data = &self.data;
        // Crc bytes
        let crc: Vec<u8> = self.crc.to_be_bytes().to_vec();

        let result: Vec<u8> = length_bytes
            .iter()
            .cloned()
            .chain(chunk_type_bytes.iter().cloned())
            .chain(data.iter().cloned())
            .chain(crc.iter().cloned())
            .collect();
        result
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Chunk Type: {}", self.chunk_type)?;
        writeln!(f, "Length: {}", self.length)?;
        if let Ok(data_value) = self.data_as_string() {
            writeln!(f, "Data: {}", data_value)?;
        } else {
            writeln!(f, "[Binary data - {} bytes]", self.data.len())?;
        }
        writeln!(f, "CRC : {}", self.crc)?;

        Ok(())
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = crate::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut reader = BufReader::new(value);
        let mut length_and_data_buffer: [u8; 4] = [0u8; 4];

        // Read length
        reader.read_exact(&mut length_and_data_buffer)?;
        let length = <u32>::from_be_bytes(length_and_data_buffer);

        // Read Chunk Type
        reader.read_exact(&mut length_and_data_buffer)?;
        let chunk_type = ChunkType::try_from(length_and_data_buffer)?;

        // Read Data
        let mut data_buffer: Vec<u8> = vec![0; length.try_into().unwrap()];
        reader.read_exact(&mut data_buffer)?;

        // Read CRC
        let mut crc_buffer: [u8; 4] = [0u8; 4];
        reader.read_exact(&mut crc_buffer)?;
        let crc = <u32>::from_be_bytes(crc_buffer);

        // Chaining chunk type and data buffers to calculate and compare CRC
        let chunk_type_and_data: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .cloned()
            .chain(data_buffer.iter().cloned())
            .collect();

        // Get crc and compare with what we got above
        const X25: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let calculated_crc = X25.checksum(&chunk_type_and_data);

        // Do the comparison
        if calculated_crc != crc {
            return Err(anyhow!("Crc mismatch!"));
        }

        Ok(Chunk {
            length: length,
            chunk_type: chunk_type,
            data: data_buffer,
            crc: crc,
        })
    }
}

#[allow(unused_variables)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
