use clap::Parser;

// #[derive(Parser)]
// #[command(version, about, long_about = None)]
// pub struct Cli {
//     /// Command to do
//     pub command: String,
//
//     /// PNG file to work on
//     pub filepath: String,
//
//     /// Chunk Type "name" to work on
//     pub chunk_type: String,
//
//     /// Message
//     pub message: String,
//
//     /// Optional output filename
//     pub output_filename: Option<String>,
// }

#[derive(Parser)]
#[command(name = "encode")]
pub enum PngCli {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
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
