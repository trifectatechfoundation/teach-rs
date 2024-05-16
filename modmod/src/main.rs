use clap::Parser;
use error_stack::Result;
use modmod::{LoadTrackError, SlidesRenderOptions, TrackRenderOptions};
use std::{path::PathBuf, process::exit};

#[derive(Parser)]
struct Args {
    #[arg(
        short = 'o',
        long = "output",
        help = "The folder the output will be written to"
    )]
    out_dir: PathBuf,
    #[arg(short = 'c', long = "clear", help = "Clear the output folder")]
    clear_output_dir: bool,
    #[arg(
        short = 'p',
        long = "patch",
        help = "Generate patch file to update output dir"
    )]
    gen_patch: bool,
    track_toml_path: PathBuf,
    #[arg(
        long,
        help = "Use this as a base when deploying the slides to a web server",
        default_value = "/"
    )]
    slide_url_base: String,
    #[arg(
        long = "theme",
        help = "The name of the Slidev theme to use in generated slide decks",
        default_value = "teach-rs"
    )]
    slide_theme: String,
    #[arg(
        long = "json-stub",
        help = "The path of the package.json stub to use when generating the slide package"
    )]
    package_json: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    fn run(args: Args) -> Result<(), LoadTrackError> {
        let Args {
            out_dir,
            clear_output_dir,
            track_toml_path,
            slide_url_base,
            slide_theme,
            package_json,
            gen_patch,
        } = args;

        let track = modmod::Track::load_toml_def(track_toml_path)?;

        let slide_opts = SlidesRenderOptions {
            theme: &slide_theme,
            package_json,
            url_base: slide_url_base.as_str(),
        };
        let track_opts = TrackRenderOptions {
            out_dir,
            slide_opts,
            clear_output_dir,
            gen_patch,
        };
        track.render(track_opts)?;
        Ok(())
    }

    if let Err(e) = run(args) {
        eprintln!("Error rendering track: {e:?}");
        exit(1);
    }

    println!("Done!");
}
