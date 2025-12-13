use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: i32,
    text: String,
    completed: bool,
}

fn save_todo(tasks: &Vec<Todo>) -> io::Result<()> {
    let serialized = serde_json::to_string(tasks)?;

    fs::write("todo.json", serialized)?;

    Ok(())
}

fn main() -> io::Result<()> {
    let task: Vec<Todo> = vec![Todo {
        id: 1,
        text: "test 1".to_string(),
        completed: false,
    }];
    save_todo(&task)?;

    Ok(())
}
