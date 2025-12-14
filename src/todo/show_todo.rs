use crate::todo::load_todo::load_todo;
use colored::*;
use tabled::Table;

pub fn show_todo() -> Result<(), Box<dyn std::error::Error>> {
    let todos = load_todo()?;

    let display: Vec<_> = todos
        .iter()
        .map(|t| {
            let text = if t.completed {
                t.text.trim().green().bold().to_string()
            } else {
                t.text.trim().yellow().to_string()
            };

            (t.id, text, t.completed)
        })
        .collect();

    let table = Table::new(display);
    println!("{table}");
    Ok(())
}
