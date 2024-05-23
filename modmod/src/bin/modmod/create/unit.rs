use modmod::load::{ModuleDef, UnitDef};

use super::imports::*;

#[derive(Debug, Clone, clap::Args)]
pub struct CreateUnit {
    module: PathBuf,
    name: String,
    #[arg(short, long)]
    index: Option<usize>,
}

impl CreateUnit {
    pub fn create(self, _common_args: &CommonArgs) -> Result<(), ModModError> {
        let mut module = ModuleDef::load(&self.module, None)
            .change_context(ModModError::default())?
            .data;

        let index = self
            .index
            .unwrap_or(module.units.len())
            .min(module.units.len());

        module.units.insert(
            index,
            UnitDef {
                name: self.name,
                template: None,
                topics: vec![],
            },
        );

        self.module
            .create_file()?
            .write_all(toml::to_string_pretty(&module).unwrap().as_bytes())?;
        Ok(())
    }
}
