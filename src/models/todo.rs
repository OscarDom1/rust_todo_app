use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TodoModel {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub completed: bool,
}
