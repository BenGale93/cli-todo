#![warn(clippy::all, clippy::nursery)]
use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
struct ToDoCli {
    #[command(subcommand)]
    command: ToDoCommands,
}

#[derive(Subcommand)]
enum ToDoCommands {
    Init(commands::InitArgs),
    Add(commands::AddArgs),
    Edit(commands::EditArgs),
    Go(commands::GoArgs),
    List(commands::ListArgs),
    Rm(commands::RemoveArgs),
}

fn main() {
    pretty_env_logger::init();
    let cli = ToDoCli::parse();

    let result = match cli.command {
        ToDoCommands::Init(init_args) => init_args.run(),
        ToDoCommands::Add(add_args) => add_args.run(),
        ToDoCommands::Edit(edit_args) => edit_args.run(),
        ToDoCommands::Go(go_args) => go_args.run(),
        ToDoCommands::List(list_args) => list_args.run(),
        ToDoCommands::Rm(rm_args) => rm_args.run(),
    };

    if let Err(e) = result {
        log::error!("{}", e);
    }
}
