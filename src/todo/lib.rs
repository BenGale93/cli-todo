pub mod add;
pub mod error;
pub mod init;
pub mod list;

pub mod prelude {
    pub const APP_NAME: &str = "todo";

    use std::{fmt, path::PathBuf};

    use chrono::{DateTime, Utc};
    use clap::ValueEnum;
    use directories::BaseDirs;
    use serde::{Deserialize, Serialize};
    use tabled::Tabled;

    pub use crate::{add::add_todo, error::ToDoError, init::initialize_todo_db, list::list_todos};

    pub type Result<T> = core::result::Result<T, ToDoError>;

    #[derive(Debug, PartialEq, ValueEnum, Clone, Copy)]
    pub enum Status {
        ToDo,
        InProgress,
        Done,
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
                i => Err(ToDoError::Generic(format!("Unrecognized status: {}", i))),
            }
        }
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum DueStatus {
        Due,
        Overdue,
    }

    impl fmt::Display for DueStatus {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::Due => write!(f, "Due"),
                Self::Overdue => write!(f, "Overdue"),
            }
        }
    }

    #[derive(Debug)]
    pub struct ToDo {
        name: String,
        content: String,
        due: DateTime<Utc>,
        status: Status,
    }

    impl ToDo {
        pub fn new(name: String, content: String, due: DateTime<Utc>, status: Status) -> Self {
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

        pub fn due(&self) -> DateTime<Utc> {
            self.due
        }

        pub fn status(&self) -> &Status {
            &self.status
        }
    }

    #[derive(Debug, Tabled)]
    pub struct ToDoRow {
        name: String,
        content: String,
        due: DateTime<Utc>,
        status: Status,
        due_status: DueStatus,
    }

    impl From<ToDo> for ToDoRow {
        fn from(value: ToDo) -> Self {
            let due_status = if value.due() >= Utc::now() {
                DueStatus::Due
            } else {
                DueStatus::Overdue
            };

            ToDoRow {
                name: value.name,
                content: value.content,
                due: value.due,
                status: value.status,
                due_status,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ToDoConfig {
        db_path: PathBuf,
    }

    impl ToDoConfig {
        pub fn new(db_path: PathBuf) -> Self {
            Self { db_path }
        }

        pub fn db_path(&self) -> &PathBuf {
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
