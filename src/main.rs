use anyhow::Ok;
use clap::Parser;

use crate::{args::PngCli, commands::encode_message};

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
        PngCli::Encode(encode_args) => {
            // FIX THIS BULLSHIT
            // let in_fp = encode_args.filepath;
            // let chunk_type = encode_args.chunk_type;
            // let message = encode_args.message;
            // let out_fp: &str = match encode_args.out_filepath {
            //     Some(val) => &val.to_string(),
            //     None => "",
            // };
            // encode_message(&in_fp, chunk_type, message, &out_fp)?;
        }
        PngCli::Decode(decode_args) => {
            println!("filepath to decode: {}", decode_args.filepath);
            println!("Chunk Type to decode {}", decode_args.chunk_type);
        }
    }

    Ok(())
}
