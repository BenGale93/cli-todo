#![warn(clippy::all, clippy::nursery)]
pub mod add;
pub mod due;
pub mod edit;
pub mod error;
pub mod go;
pub mod init;
pub mod list;
pub mod rm;
mod sql_utils;

pub mod prelude {
    pub const APP_NAME: &str = "todo";

    use std::{fmt, path::PathBuf};

    use chrono::{Local, NaiveDateTime};
    use clap::ValueEnum;
    use directories::BaseDirs;
    use serde::{Deserialize, Serialize};
    use tabled::Tabled;

    pub use crate::{
        add::add_todo,
        edit::edit_todo,
        error::ToDoError,
        go::go_todo,
        init::initialize_todo_db,
        list::list_todos,
        rm::{remove_done, remove_todo},
    };

    pub type Result<T> = core::result::Result<T, ToDoError>;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Clone, Copy)]
    pub enum Status {
        ToDo,
        InProgress,
        Done,
    }

    impl Status {
        pub const fn not_done() -> [Self; 2] {
            [Self::ToDo, Self::InProgress]
        }
    }

    impl fmt::Display for Status {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::ToDo => write!(f, "To Do"),
                Self::InProgress => write!(f, "In Progress"),
                Self::Done => write!(f, "Done"),
            }
        }
    }

    impl std::str::FromStr for Status {
        type Err = ToDoError;

        fn from_str(input: &str) -> Result<Self> {
            match input {
                "To Do" => Ok(Self::ToDo),
                "In Progress" => Ok(Self::InProgress),
                "Done" => Ok(Self::Done),
                i => Err(ToDoError::UnrecognisedStatus(i.to_string())),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
    pub enum DueStatus {
        Due,
        Overdue,
        Done,
    }

    impl fmt::Display for DueStatus {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::Due => write!(f, "Due"),
                Self::Overdue => write!(f, "Overdue"),
                Self::Done => write!(f, "Done"),
            }
        }
    }

    #[derive(Debug)]
    pub struct ToDo {
        name: String,
        content: String,
        due: NaiveDateTime,
        status: Status,
    }

    impl ToDo {
        pub const fn new(
            name: String,
            content: String,
            due: NaiveDateTime,
            status: Status,
        ) -> Self {
            Self {
                name,
                content,
                due,
                status,
            }
        }

        pub fn name(&self) -> &str {
            self.name.as_ref()
        }

        pub fn content(&self) -> &str {
            self.content.as_ref()
        }

        pub const fn due(&self) -> NaiveDateTime {
            self.due
        }

        pub const fn status(&self) -> &Status {
            &self.status
        }
    }

    #[derive(Debug, Tabled)]
    pub struct ToDoRow {
        name: String,
        content: String,
        due: NaiveDateTime,
        status: Status,
        due_status: DueStatus,
    }

    impl From<ToDo> for ToDoRow {
        fn from(value: ToDo) -> Self {
            let due_status = if matches!(value.status(), Status::Done) {
                DueStatus::Done
            } else if value.due() >= Local::now().naive_local() {
                DueStatus::Due
            } else {
                DueStatus::Overdue
            };

            Self {
                name: value.name,
                content: value.content,
                due: value.due,
                status: value.status,
                due_status,
            }
        }
    }

    #[derive(Debug)]
    pub struct ToDoPatch {
        pub name: String,
        pub content: Option<String>,
        pub due: Option<NaiveDateTime>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ToDoConfig {
        db_path: PathBuf,
    }

    impl ToDoConfig {
        pub const fn new(db_path: PathBuf) -> Self {
            Self { db_path }
        }

        pub const fn db_path(&self) -> &PathBuf {
            &self.db_path
        }
    }

    impl Default for ToDoConfig {
        fn default() -> Self {
            let project_dirs = BaseDirs::new().unwrap();
            let db_path = project_dirs.data_dir().join("todo").join("todo.db");
            Self { db_path }
        }
    }
}
