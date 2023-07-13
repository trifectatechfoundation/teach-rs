use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    #[arg(
        short = 'o',
        long = "output",
        help = "The folder the output will be written to"
    )]
    output_dir: PathBuf,
    #[arg(short = 'c', long = "clear", help = "Clear the output folder")]
    clear_output_dir: bool,
    track_toml_path: PathBuf,
}

fn main() {
    let Args {
        output_dir,
        clear_output_dir,
        track_toml_path,
    } = Args::parse();
    modmod::render(track_toml_path, output_dir, clear_output_dir).unwrap();
}
