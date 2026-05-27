use std::env;

use anyhow::Ok;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = anyhow::Error;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    Ok(())
}
