use crate::todo::Todo;
use std::fs;

pub fn load_todo() -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
    let data = match fs::read_to_string("todo.json") {
        Ok(data) => data,
        Err(_) => return Ok(Vec::new()),
    };

    let tasks: Vec<Todo> = serde_json::from_str(&data).unwrap_or_default();

    Ok(tasks)
}
