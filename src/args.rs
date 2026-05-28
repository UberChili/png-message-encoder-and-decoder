use clap::Parser;

#[derive(Parser)]
#[command(name = "encode")]
pub enum PngCli {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(DecodeArgs),
    Print(PrintArg),
}

#[derive(clap::Args)]
#[command(version, about, long_about = None)]
pub struct EncodeArgs {
    pub filepath: String,
    pub chunk_type: String,
    pub message: String,
    pub out_filepath: Option<String>,
}

#[derive(clap::Args)]
#[command(version, about, long_about = None)]
pub struct DecodeArgs {
    pub filepath: String,
    pub chunk_type: String,
}

#[derive(clap::Args)]
#[command(version, about, long_about = None)]
pub struct PrintArg {
    pub filepath: String,
}
