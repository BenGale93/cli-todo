pub mod error;
pub mod init;

pub mod prelude {
    pub const APP_NAME: &str = "todo";

    use std::path::PathBuf;

    use chrono::{DateTime, Utc};
    use directories::BaseDirs;
    use serde::{Deserialize, Serialize};

    pub use crate::error::ToDoError;

    pub type Result<T> = core::result::Result<T, ToDoError>;

    pub use crate::init::initialize_todo_db;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ToDo {
        id: i32,
        content: String,
        due: DateTime<Utc>,
        status: String,
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
