#![warn(clippy::all, clippy::nursery)]
use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(about = "Manage todos from the CLI.")]
struct ToDoCli {
    #[command(subcommand)]
    command: ToDoCommands,
}

#[derive(Subcommand)]
enum ToDoCommands {
    /// Initialise a new todo database.
    Init(commands::InitArgs),
    /// Add a new todo to your list.
    Add(commands::AddArgs),
    /// Edit an existing todo.
    Edit(commands::EditArgs),
    /// Progress a todo to the next status.
    Go(commands::GoArgs),
    /// List your todos.
    List(commands::ListArgs),
    /// Delete a todo.
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
