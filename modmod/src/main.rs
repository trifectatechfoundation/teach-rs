use clap::Parser;
use error_stack::Result;
use modmod::LoadTrackError;
use std::{path::PathBuf, process::exit};

#[derive(Parser)]
struct Args {
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
    let args = Args::parse();

    fn run(args: Args) -> Result<(), LoadTrackError> {
        let Args {
            output_dir,
            clear_output_dir,
            track_toml_path,
        } = args;
        let track = modmod::Track::load_toml_def(track_toml_path)?;
        track.render(output_dir, clear_output_dir)?;
        Ok(())
    }

    if let Err(e) = run(args) {
        eprintln!("Error rendering track: {e:?}");
        exit(1);
    }

    println!("Done!");
}
