use std::path::PathBuf;

use clap::Args;
use todo::prelude::*;

#[derive(Args)]
pub struct InitArgs {
    #[arg(short, long, value_name = "FILE")]
    database: Option<PathBuf>,
    #[arg(short, long)]
    force: bool,
}

impl InitArgs {
    pub fn run(&self) -> Result<()> {
        if !self.force && confy::get_configuration_file_path(APP_NAME, None)?.exists() {
            return Err(ToDoError::Generic("Config already exists.".to_string()));
        }
        let cfg = self
            .database
            .as_ref()
            .map_or_else(ToDoConfig::default, |p| ToDoConfig::new(p.clone()));

        initialize_todo_db(cfg.db_path())?;

        Ok(confy::store(APP_NAME, None, cfg)?)
    }
}
