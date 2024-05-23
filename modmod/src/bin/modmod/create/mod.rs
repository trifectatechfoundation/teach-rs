use clap::{Parser, Subcommand};
use error_stack::Result;

use crate::ModModError;

mod exercise;
mod module;
mod topic;
mod unit;

#[derive(Debug, Clone, Parser)]
pub struct Args {
    #[command(subcommand)]
    what: What,
    #[clap(flatten)]
    common: CommonArgs,
}

#[derive(Debug, Clone, clap::Args)]
pub struct CommonArgs {
    #[arg(short = 'f', long = "force")]
    force: bool,
}

#[derive(Debug, Clone, Subcommand)]
enum What {
    Module(module::CreateModule),
    Unit(unit::CreateUnit),
    Topic(topic::CreateTopic),
    Exercise(exercise::CreateExercise),
}

impl What {
    fn create(self, common: &CommonArgs) -> Result<(), ModModError> {
        match self {
            What::Module(m) => m.create(common),
            What::Unit(u) => u.create(common),
            What::Topic(t) => t.create(common),
            What::Exercise(e) => e.create(common),
        }
    }
}

pub fn run(args: Args) -> Result<(), ModModError> {
    args.what.create(&args.common)
}

mod imports {
    pub(crate) use crate::create::CommonArgs;
    pub(crate) use crate::ModModError;
    pub use error_stack::{Result, ResultExt};
    pub use modmod::{
        io::{PathExt, WriteExt},
        load::Load,
    };
    pub use std::path::PathBuf;
}
