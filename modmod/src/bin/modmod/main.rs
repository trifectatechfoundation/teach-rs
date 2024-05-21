use clap::{Parser, Subcommand};
use std::{fmt, process::exit};

mod gen;

#[non_exhaustive]
#[derive(Debug, Default)]
struct ModModError {}

impl error_stack::Context for ModModError {}

impl fmt::Display for ModModError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Error running ModMod")
    }
}

#[derive(Parser)]
struct App {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Generate(gen::Args),
}

fn main() {
    let app = App::parse();

    match app.command {
        Command::Generate(args) => {
            if let Err(e) = gen::run(args) {
                eprintln!("Error rendering track: {e:?}");
                exit(1);
            }
        }
    }

    println!("Done!");
}
