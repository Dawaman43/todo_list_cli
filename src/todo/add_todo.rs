use crate::todo::Todo;
use std::fs;

pub fn add_todo(tasks: &mut Vec<Todo>) -> Result<(), Box<dyn std::error::Error>> {
    let task = serde_json::to_string_pretty(tasks)?;
    fs::write("todo.json", task)?;
    Ok(())
}
