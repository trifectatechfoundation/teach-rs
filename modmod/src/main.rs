use clap::Parser;
use error_stack::Result;
use modmod::{patch::GenPatchOptions, LoadTrackError, SlidesRenderOptions, TrackRenderOptions};
use std::{fs, path::PathBuf, process::exit};

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
        help = "Generate patch file to update output dir at given path"
    )]
    patch_file: Option<PathBuf>,
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
            patch_file,
        } = args;

        let track = modmod::Track::load_toml_def(track_toml_path)?;

        let slide_opts = SlidesRenderOptions {
            theme: &slide_theme,
            package_json,
            url_base: slide_url_base.as_str(),
        };

        let (out_dir, patch_opts) = if let Some(patch_file) = patch_file {
            let tmp_dir = std::env::temp_dir().join("modmod_tmp");
            let patch_opts = GenPatchOptions {
                new_dir: tmp_dir.clone(),
                old_dir: out_dir,
                patch_file,
            };
            (tmp_dir, Some(patch_opts))
        } else {
            (out_dir, None)
        };

        let track_opts = TrackRenderOptions {
            out_dir,
            slide_opts,
            clear_output_dir,
        };
        track.render(track_opts)?;

        if let Some(patch_opts) = patch_opts {
            let tmp_dir = patch_opts.new_dir.clone();
            modmod::patch::Patch::render(patch_opts).unwrap();
            fs::remove_dir_all(tmp_dir).unwrap();
        }

        Ok(())
    }

    if let Err(e) = run(args) {
        eprintln!("Error rendering track: {e:?}");
        exit(1);
    }

    println!("Done!");
}
