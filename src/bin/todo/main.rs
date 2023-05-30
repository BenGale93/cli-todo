use clap::{Parser, Subcommand};
use commands::run_init_command;
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
    Add,
    Edit,
    Done,
    List,
}

fn main() -> Result<()> {
    let cli = ToDoCli::parse();

    match &cli.command {
        ToDoCommands::Init(init_args) => run_init_command(init_args),
        ToDoCommands::Add => todo!(),
        ToDoCommands::Edit => todo!(),
        ToDoCommands::Done => todo!(),
        ToDoCommands::List => todo!(),
    }
}
