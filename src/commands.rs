use std::{fs, path::PathBuf, str::FromStr};

use anyhow::anyhow;

use crate::{args::EncodeArgs, chunk::Chunk, chunk_type::ChunkType, png::Png};

pub fn encode_message(parameters: EncodeArgs) -> crate::Result<()> {
    let path: PathBuf = PathBuf::from(parameters.filepath);

    // Sanity
    if !path.exists() {
        return Err(anyhow!("File {} doesn't exist.", &path.display()));
    }
    // Not sure what I'm doing

    // Reading file and forming PNG
    let file_data: Vec<u8> = fs::read(path)?;
    let mut png = Png::try_from(file_data.as_slice())?;

    // Convert everything to bytes and form Chunk Type and Chunk
    let message_data: Vec<u8> = message.as_bytes().to_vec();
    // Chunk Type
    let new_chunk_type: ChunkType = ChunkType::from_str(&chunk_type)?;
    // Chunk
    let hidden_msg_chunk: Chunk = Chunk::new(new_chunk_type, message_data);

    // Push new Chunk to png file
    png.append_chunk(hidden_msg_chunk);

    Ok(())
}
