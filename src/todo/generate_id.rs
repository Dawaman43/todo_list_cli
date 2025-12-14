use crate::todo::Todo;
use std::fs;

pub fn generate_id() -> Result<i32, Box<dyn std::error::Error>> {
    let data = fs::read_to_string("todo.json")?;

    let file: Vec<Todo> = serde_json::from_str(&data)?;

    let next_id = file.iter().map(|t| t.id).max().unwrap_or(0) + 1;

    Ok(next_id)
}
