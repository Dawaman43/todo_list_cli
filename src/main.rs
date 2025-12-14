mod todo;
use colored::*;
use figlet_rs::FIGfont;
use std::io;

use todo::TodoOptions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let font = FIGfont::standard().unwrap();
    let figure = font.convert("Todo List Cli").unwrap();
    println!("{}", figure.to_string().cyan().bold());

    loop {
        println!("{}", "Choose your option".green().bold());
        println!("1. Add todo \n2. Show Todo lists \n3. Mark completed \n4. Delete todo \n5. Quit");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("can't read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(d) => d,
            Err(_) => {
                println!("Please enter a number");
                return Ok(());
            }
        };

        let options = match choice {
            1 => TodoOptions::Add,
            2 => TodoOptions::Show,
            3 => TodoOptions::Complete,
            4 => TodoOptions::Delete,
            5 => {
                println!("{}", "Thank you".cyan().bold());
                break;
            }
            _ => {
                println!("Invalid Option");
                return Ok(());
            }
        };

        options.run()?;
    }

    Ok(())
}
