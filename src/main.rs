mod password_manager;

use std::io::{self, Write};
use password_manager::PasswordManager;

fn main() {
    let mut manager = PasswordManager::new();

    loop {
        println!("\nPassword Manager:");
        println!("1. Add a password");
        println!("2. Retrieve a password");
        println!("3. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => manager.add_password(),
            "2" => manager.get_password(),
            "3" => break,
            _ => println!("Invalid choice, please try again."),
        }
    }
}
