use clap::{Parser, Subcommand};
use todo::prelude::*;

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
    Edit,
    Go(commands::GoArgs),
    List(commands::ListArgs),
}

fn main() -> Result<()> {
    let cli = ToDoCli::parse();

    match cli.command {
        ToDoCommands::Init(init_args) => init_args.run(),
        ToDoCommands::Add(add_args) => add_args.run(),
        ToDoCommands::Edit => todo!(),
        ToDoCommands::Go(go_args) => go_args.run(),
        ToDoCommands::List(list_args) => list_args.run(),
    }
}
