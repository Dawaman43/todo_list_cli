use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Todo {
    id: i32,
    text: String,
    completed: bool,
}
