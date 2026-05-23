use std::str::FromStr;

use anyhow::anyhow;

#[derive(Debug, PartialEq)]
pub struct ChunkType {
    chunk_type: [u8; 4],
}

#[allow(dead_code)]
impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.chunk_type
    }

    pub fn is_valid(&self) -> bool {
        if self.chunk_type.len() != 4 {
            return false;
        }
        for c in self.chunk_type {
            if !c.is_ascii_alphabetic() {
                return false;
            }
        }
        if !self.is_reserved_bit_valid() {
            return false;
        }
        true
    }

    pub fn is_critical(&self) -> bool {
        let bit5 = (self.chunk_type[0] & (1 << 5)) != 0;
        if bit5 {
            return false;
        }
        true
    }

    pub fn is_public(&self) -> bool {
        let bit5 = (self.chunk_type[1] & (1 << 5)) != 0;
        if bit5 {
            return false;
        }
        true
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        let bit5 = (self.chunk_type[2] & (1 << 5)) != 0;
        if bit5 {
            return false;
        }
        true
    }

    pub fn is_safe_to_copy(&self) -> bool {
        let bit5 = (self.chunk_type[3] & (1 << 5)) != 0;
        if bit5 {
            return true;
        }
        false
    }
}

impl FromStr for ChunkType {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        if s.len() != 4 {
            return Err(anyhow!(
                "Incorrect lengh of string {} for Chunk Type. Has to be of length 4.",
                s
            ));
        }

        let mut code_vec: Vec<u8> = vec![];

        for i in s.chars() {
            if !i.is_ascii_alphabetic() {
                return Err(anyhow!("Incorrect char for Chunk Type: {}", i));
            } else {
                code_vec.push(i as u8);
            }
        }

        let code = match <[u8; 4]>::try_from(code_vec) {
            Ok(val) => val,
            Err(_err) => {
                return Err(anyhow!(
                    "Error: . Could not convert s ({}) into array of bytes",
                    s
                ));
            }
        };
        Ok(ChunkType { chunk_type: code })
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = crate::Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(ChunkType { chunk_type: value })
    }
}

impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.chunk_type {
            write!(f, "{}", c as char)?;
        }

        Ok(())
    }
}

#[allow(unused_variables)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
