use anyhow::Ok;
use clap::Parser;

use crate::{
    args::PngCli,
    commands::{decode_message, encode_message},
};

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
            encode_message(&encode_args)?;
        }
        PngCli::Decode(decode_args) => {
            decode_message(&decode_args)?;
        }
    }

    Ok(())
}
