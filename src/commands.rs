use std::{fs, path::PathBuf, str::FromStr};

use anyhow::anyhow;

use crate::{
    args::DecodeArgs, args::EncodeArgs, args::PrintArg, chunk::Chunk, chunk_type::ChunkType,
    png::Png,
};

pub fn encode_message(parameters: &EncodeArgs) -> crate::Result<()> {
    let path: PathBuf = PathBuf::from(&parameters.filepath);

    // Sanity checks
    if !path.exists() {
        return Err(anyhow!("File {} doesn't exist.", &path.display()));
    }
    if path.extension().and_then(|s| s.to_str()) != Some("png") {
        return Err(anyhow!("File must have .png extension"));
    }

    // Use new filepath if provided
    let new_filepath = match &parameters.out_filepath {
        Some(name) => PathBuf::from(name),
        None => path.clone(),
    };

    // Reading file and forming PNG
    let file_data = fs::read(&path)?;
    let mut png = Png::try_from(file_data.as_slice())?;

    // Convert everything to bytes and form Chunk Type and Chunk
    let message_data: Vec<u8> = parameters.message.as_bytes().to_vec();
    // Chunk Type
    let new_chunk_type: ChunkType = ChunkType::from_str(&parameters.chunk_type)?;
    // Chunk
    let hidden_msg_chunk: Chunk = Chunk::new(new_chunk_type, message_data);

    // Check if safe to copy
    if !hidden_msg_chunk.chunk_type().is_safe_to_copy() {
        return Err(anyhow!("Could not encode message. Chunk not safe to copy."));
    }

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
    if path.extension().and_then(|s| s.to_str()) != Some("png") {
        return Err(anyhow!("File must have .png extension"));
    }

    // Reading file and interpreting as PNG data
    let file_data = fs::read(&path)?;
    let png = Png::try_from(file_data.as_slice())?;

    // loop through Chunks until we find Chunk Type
    for chunk in png.chunks() {
        if chunk.chunk_type().bytes() == parameters.chunk_type.as_bytes() {
            let message = String::from_utf8(chunk.data().to_vec())?;
            println!("Message in Chunk: {}", message);
            return Ok(());
        }
    }
    return Err(anyhow!("No matching Chunk found."));
}

pub fn remove_message(parameters: &DecodeArgs) -> crate::Result<()> {
    let path: PathBuf = PathBuf::from(&parameters.filepath);
    if !path.exists() {
        return Err(anyhow!("File {} doesn't exist.", &path.display()));
    }
    if path.extension().and_then(|s| s.to_str()) != Some("png") {
        return Err(anyhow!("File must have .png extension"));
    }

    // Reading file and interpreting as PNG data
    let file_data = fs::read(&path)?;
    let mut png = Png::try_from(file_data.as_slice())?;

    // Use png type's method to remove first Chunk with given Chunk Type
    png.remove_first_chunk(&parameters.chunk_type)?;

    // Write output to disk
    fs::write(path, png.as_bytes())?;

    Ok(())
}

pub fn print_file(parameters: &PrintArg) -> crate::Result<()> {
    let path = PathBuf::from(&parameters.filepath);
    if !path.exists() {
        return Err(anyhow!("File {} doesn't exist.", &path.display()));
    }
    if path.extension().and_then(|s| s.to_str()) != Some("png") {
        return Err(anyhow!("File must have .png extension"));
    }

    // Reading file and interpreting as PNG data
    let file_data = fs::read(&path)?;
    let png = Png::try_from(file_data.as_slice())?;

    // Printing the information
    println!("PNG File: {}", path.display());
    println!("Number of Chunks: {}", png.chunks().len());
    println!("Chunks:");
    for chunk in png.chunks() {
        println!("{}", chunk);
    }

    Ok(())
}
