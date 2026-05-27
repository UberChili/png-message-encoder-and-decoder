use anyhow::Ok;
use clap::Parser;

use crate::args::PngCli;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = anyhow::Error;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = PngCli::parse();

    match cli {
        PngCli::Decode(decode_args) => {
            println!("filepath to decode: {}", decode_args.filepath);
            println!("Chunk Type to decode {}", decode_args.chunk_type);
        }
        _ => (),
    }

    Ok(())
}
