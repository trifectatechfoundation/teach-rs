use modmod::load::{ModuleDef, TopicDef};

use super::imports::*;

#[derive(Debug, Clone, clap::Args)]
pub struct CreateTopic {
    module: PathBuf,
    #[arg(
        short = 'i',
        long = "index",
        help = "The unit to which to add the topic to. Defaults to the last unit"
    )]
    unit_index: Option<usize>,
    dir: PathBuf,
    name: String,
    description: Option<String>,
}

impl CreateTopic {
    pub fn create(self, common_args: &CommonArgs) -> Result<(), ModModError> {
        let mut module = ModuleDef::load(&self.module, None)
            .change_context(ModModError::default())?
            .data;

        if module.units.is_empty() {
            return Err(ModModError::report().attach_printable(
                "There are no units to attach the topic to. Create units first.",
            ));
        }

        let max_unit_index = module.units.len() - 1;
        let unit_index = self.unit_index.unwrap_or(max_unit_index);
        let Some(unit) = module.units.get_mut(unit_index) else {
            return Err(ModModError::report().attach_printable(format!(
                "No unit at that index yet. Pick a number between 0 and {max_unit_index} or create more units first"
            )));
        };

        let topic_toml_path = self.module.parent().unwrap().join("topics").join(&self.dir);
        topic_toml_path.create_dir_all()?;
        let mut topic_toml = topic_toml_path
            .join("topic.toml")
            .try_create_file(common_args.force)?;

        let topic = TopicDef {
            name: self.name,
            ..Default::default()
        };
        unit.topics.push(self.dir.join("mod.toml"));
        unit.topics.dedup();

        topic_toml.write_all(toml::to_string_pretty(&topic).unwrap().as_bytes())?;
        topic_toml_path
            .join("slides.md")
            .try_create_file(common_args.force)?;

        self.module
            .create_file()?
            .write_all(toml::to_string_pretty(&module).unwrap())?;

        Ok(())
    }
}
