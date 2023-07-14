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

    match modmod::render(&track_toml_path, &output_dir, clear_output_dir) {
        Ok(()) => println!(
            "Done writing your track content to directory '{}'",
            output_dir.to_string_lossy()
        ),
        Err(e) => eprintln!("{e:?}"),
    }
}
