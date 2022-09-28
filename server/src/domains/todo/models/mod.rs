use serde::{Deserialize, Serialize};

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

impl From<todo::TodoStatus> for TodoStatus {
    fn from(status: todo::TodoStatus) -> Self {
        match status {
            todo::TodoStatus::Done => Self::Done,
            todo::TodoStatus::Pending => Self::Pending,
        }
    }
}
