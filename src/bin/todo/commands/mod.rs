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

pub fn run_init_command(init_args: &InitArgs) -> Result<()> {
    if !init_args.force && confy::get_configuration_file_path(APP_NAME, None)?.exists() {
        return Err(ToDoError::Generic("Config already exists.".to_string()));
    }
    let cfg = match &init_args.database {
        Some(p) => ToDoConfig::new(p.clone()),
        None => ToDoConfig::default(),
    };
    initialize_todo_db(cfg.db_path())?;

    Ok(confy::store(APP_NAME, None, cfg)?)
}
