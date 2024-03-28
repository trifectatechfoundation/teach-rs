use clap::{Parser, Subcommand};
use std::{error::Error, process::exit};

#[derive(Parser)]
struct App {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Clone)]
enum Command {
    Generate(gen::GenArgs),
    AddTopic(add_topic::AddTopicArgs),
}

fn main() {
    let app = App::parse();

    fn run(app: App) -> std::result::Result<(), Box<dyn Error>> {
        match app.command {
            Command::AddTopic(args) => {
                add_topic::add_topic(args).map_err(|e| Box::new(e.into_error()).into())
            }
            Command::Generate(args) => {
                gen::generate(args).map_err(|e| Box::new(e.into_error()).into())
            }
        }
    }

    if let Err(e) = run(app) {
        eprintln!("Error: {e:?}");
        exit(1);
    }

    println!("Done!");
}

mod gen {
    use error_stack::Result;
    use std::path::PathBuf;

    use clap::Args;
    use modmod::LoadTrackError;

    #[derive(Args, Clone)]
    pub struct GenArgs {
        #[arg(
            short = 'o',
            long = "output",
            help = "The folder the output will be written to"
        )]
        output_dir: PathBuf,
        #[arg(short = 'c', long = "clear", help = "Clear the output folder")]
        clear_output_dir: bool,
        #[arg(
            short = 'b',
            long = "base",
            help = "The base path for the slide assets"
        )]
        base_path: String,
        track_toml_path: PathBuf,
    }

    pub fn generate(args: GenArgs) -> Result<(), LoadTrackError> {
        let GenArgs {
            output_dir,
            clear_output_dir,
            track_toml_path,
            base_path,
        } = args;
        let track = modmod::Track::load_toml_def(track_toml_path)?;
        track.render(output_dir, clear_output_dir)?;
        Ok(())
    }
}

mod add_topic {
    use error_stack::{IntoReport, Result, ResultExt};
    use modmod::{
        io::PathExt,
        load::{Load, ModuleDef, PathTo, TopicDef},
    };
    use std::{fmt, path::PathBuf};

    use clap::Args;

    #[derive(Args, Debug, Clone)]
    pub struct AddTopicArgs {
        mod_path: PathBuf,
        name: String,
        slug: String,
        unit: usize,
    }

    #[derive(Debug, Default)]
    pub struct AddTopicError;

    impl fmt::Display for AddTopicError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("Unable generate topic")
        }
    }

    impl error_stack::Context for AddTopicError {}

    pub fn add_topic(args: AddTopicArgs) -> Result<(), AddTopicError> {
        let AddTopicArgs {
            mod_path,
            name,
            slug,
            unit,
        } = args;

        let PathTo {
            mut data,
            path: mod_path,
        } = ModuleDef::load(&mod_path, None).change_context(AddTopicError)?;

        let mod_dir = mod_path.parent().unwrap();
        let topic_dir = mod_dir.join("topics").join(slug);
        topic_dir.create_dir_all()?;

        let topic_path = topic_dir.join("topic.toml");

        let topic = TopicDef {
            name,
            exercises: vec![],
            summary: vec![],
            objectives: vec![],
            content: PathBuf::from("slides.md"),
            further_reading: vec![],
        };

        let unit = &mut data.units[args.unit - 1];
        unit.topics
            .push(topic_path.strip_prefix(&mod_dir).unwrap().to_path_buf());

        // dbg!(toml::to_string(topic));
        dbg!(unit);

        Ok(())
    }
}
