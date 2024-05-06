use std::{process, thread};

use chrono::Utc;
use clap::Args;
use humantime::Duration;
use signal_hook::{
    consts::{SIGINT, SIGTERM},
    iterator::Signals,
};
use todo::prelude::*;

use crate::commands::get_connection;

#[derive(Args)]
pub struct WatchArgs {
    // Length of time to wait before checking if any items are overdue.
    #[arg(short, long, default_value = "5 minutes")]
    duration: Duration,
    // Length of time to wait before checking for signal interrupts.
    #[arg(short, long, default_value = "500 ms")]
    poll: Duration,
}

impl WatchArgs {
    pub fn run(self) -> Result<()> {
        let conn = get_connection()?;

        let mut signals = Signals::new([SIGTERM, SIGINT])?;

        let sig_thread = thread::spawn(move || {
            #[allow(clippy::never_loop)]
            for signal in signals.forever() {
                println!("\nShutting down todo watch.");
                match signal {
                    SIGTERM | SIGINT => return,
                    _ => unreachable!(),
                }
            }
        });

        println!("My pid is {}", process::id());
        let mut work_thread = thread::spawn(move || thread::sleep(*self.duration));

        loop {
            if work_thread.is_finished() {
                let time = Utc::now().naive_utc();

                let todos = list_todos(Some(&Status::not_done()), Some(time), &conn)?;
                let todo_rows: Vec<ToDoRow> = todos.into_iter().map(|t| t.into()).collect();
                if !todo_rows.is_empty() {
                    let table = tabled::Table::new(todo_rows);
                    println!("{}", table);
                }
                work_thread = thread::spawn(move || thread::sleep(*self.duration));
            }
            if sig_thread.is_finished() {
                drop(work_thread);
                break;
            } else {
                thread::sleep(*self.poll);
            }
        }

        Ok(())
    }
}
