use serde::{Deserialize, Serialize};

use super::repositories;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub id: String,
    pub status: TodoStatus,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TodoStatus {
    Done,
    Pending,
    InProgress { from: String },
}

impl From<repositories::todo::TodoStatus> for TodoStatus {
    fn from(status: repositories::todo::TodoStatus) -> Self {
        use repositories::todo::TodoStatus;
        match status {
            TodoStatus::Done => Self::Done,
            TodoStatus::Pending => Self::Pending,
        }
    }
}
