use clap::Parser;

#[derive(Parser)]
#[command(name = "pngmsg")]
// #[command(name = "encode")]
#[command(about = "Hide and exract hidden messages in PNG files", long_about=None)]
pub enum PngCli {
    /// Encode a secret message into a PNG file
    Encode(EncodeArgs),
    /// Decode / reveal a hidden message from a PNG file
    Decode(DecodeArgs),
    /// Remove a hidden message from a PNG file
    Remove(DecodeArgs),
    /// Print the internal structure of a PNG file (lists Chunks)
    Print(PrintArg),
}

#[derive(clap::Args)]
#[command(version, about = "Hide a message inside a PNG file", long_about = None)]
pub struct EncodeArgs {
    pub filepath: String,
    #[arg(help = "4-letter chunk type (e.g. RuSt)")]
    pub chunk_type: String,
    #[arg(help = "The secret message to hide")]
    pub message: String,
    #[arg(
        short = 'o',
        long = "output",
        value_name = "FILE",
        help = "Optional output file path (defaults to overwriting input file)"
    )]
    pub out_filepath: Option<String>,
}

#[derive(clap::Args)]
#[command(version, about = "Remove a hidden message from a PNG file", long_about = None)]
pub struct DecodeArgs {
    pub filepath: String,
    #[arg(help = "4-letter chunk type to remove")]
    pub chunk_type: String,
}

#[derive(clap::Args)]
#[command(version, about = "Display PNG file structure and chunks", long_about = None)]
pub struct PrintArg {
    pub filepath: String,
}
