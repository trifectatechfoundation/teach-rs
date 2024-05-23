use modmod::load::ModuleDef;

use super::imports::*;

#[derive(Debug, Clone, clap::Args)]
pub struct CreateModule {
    path: PathBuf,
    name: String,
    description: String,
}

impl CreateModule {
    pub fn create(self, common_args: &CommonArgs) -> Result<(), ModModError> {
        let mod_toml_path = self.path.join("mod.toml");

        self.path.create_dir_all()?;
        let mut mod_toml = mod_toml_path.try_create_file(common_args.force)?;

        let module = ModuleDef {
            name: self.name,
            description: self.description,
            units: vec![],
        };

        mod_toml.write_all(toml::to_string_pretty(&module).unwrap().as_bytes())?;

        Ok(())
    }
}
