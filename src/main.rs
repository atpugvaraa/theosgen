use std::io::{self, Write};
use std::fs;
use std::process;

fn main() {
    println!("--- Theos Project Generator ---");

    // Get Project Name & Create Directory
    // In Rust, variables are immutable by default. use 'mut' if they change.
    let project_name = read_input();

    let path = std::path::Path::new(&project_name);
    if path.exists() {
        eprintln!("Error: That folder already exists!");
        process::exit(1);
    }

    // Success/Error switch statement type shit
    match fs::create_dir(&project_name) {
        Ok(_) => println!("Created directory: {}", project_name),
        Err(e) => {
            eprintln!("Failed to create directory: {}", e);
            process::exit(1);
        }
    }

    // Get tweak metadata

    // Write files




    // similar to \(projectName) in Swift
    // {} is used to interpolate strings and use variables in rust.
    println!("Successfully created project: {}", project_name);
}

fn read_input() -> String {
    print!("Enter the project name: ");

    // flush() ensures the prompt appears before the user types
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // trim() removes the newline character (\n) at the end
    input.trim().to_string()
}