use std::{fs, path::PathBuf, str::FromStr};

use anyhow::anyhow;

use crate::{args::DecodeArgs, args::EncodeArgs, chunk::Chunk, chunk_type::ChunkType, png::Png};

pub fn encode_message(parameters: &EncodeArgs) -> crate::Result<()> {
    let path: PathBuf = PathBuf::from(&parameters.filepath);

    // Sanity
    if !path.exists() {
        return Err(anyhow!("File {} doesn't exist.", &path.display()));
    }
    let new_filepath = match &parameters.out_filepath {
        Some(name) => PathBuf::from(name),
        None => path,
    };

    // Reading file and forming PNG
    let file_data = fs::read(&new_filepath)?;
    let mut png = Png::try_from(file_data.as_slice())?;

    // Convert everything to bytes and form Chunk Type and Chunk
    let message_data: Vec<u8> = parameters.message.as_bytes().to_vec();
    // Chunk Type
    let new_chunk_type: ChunkType = ChunkType::from_str(&parameters.chunk_type)?;
    // Chunk
    let hidden_msg_chunk: Chunk = Chunk::new(new_chunk_type, message_data);

    // Push new Chunk to png file
    png.append_chunk(hidden_msg_chunk);

    // Write output to disk
    fs::write(new_filepath, png.as_bytes())?;

    Ok(())
}

pub fn decode_message(parameters: &DecodeArgs) -> crate::Result<()> {
    let path: PathBuf = PathBuf::from(&parameters.filepath);

    if !path.exists() {
        return Err(anyhow!("File {} doesn't exist.", &path.display()));
    }

    // Reading file and interpreting as PNG data
    let file_data = fs::read(&path)?;
    let png = Png::try_from(file_data.as_slice())?;

    // loop through Chunks until we find Chunk Type
    for chunk in png.chunks() {
        if chunk.chunk_type().bytes() == parameters.chunk_type.as_bytes() {
            let message = String::from_utf8(chunk.data().to_vec())?;
            println!("{:?}", message);
        }
    }

    Ok(())
}
