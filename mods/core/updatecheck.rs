use std::process::Command;
use std::io::{self, Write};

fn main() {
    println!("Checking for updates using 'hyman update'...");

    // Run "hyman update" and capture its output
    let update_output = Command::new("hyman")
        .arg("update")
        .output()
        .expect("Failed to execute 'hyman update' command");

    if !update_output.status.success() {
        eprintln!("Error: Failed to check for updates.");
        return;
    }

    // Parse and display the output of "hyman update"
    let output = String::from_utf8_lossy(&update_output.stdout);
    if output.contains("No updates available") {
        println!("All packages are up to date!");
        return;
    }

    println!("Updates found:\n{}", output);

    // Ask the user for confirmation
    println!("Do you want to run the server with outdated packages? (y/n)");
    print!("> ");
    io::stdout().flush().unwrap();

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");
    let user_input = user_input.trim().to_lowercase();

    if user_input == "y" {
        println!("Warning: Running the server with outdated packages...");
        // Continue server execution logic here
    } else if user_input == "n" {
        println!("Running 'hyman update --now' to apply updates...");
        let now_update_output = Command::new("hyman")
            .arg("update")
            .arg("--now")
            .output()
            .expect("Failed to execute 'hyman update --now' command");

        if now_update_output.status.success() {
            println!("All packages updated successfully. Proceeding...");
            // Proceed to server start logic here
        } else {
            eprintln!(
                "Error: Failed to apply updates.\n{}",
                String::from_utf8_lossy(&now_update_output.stderr)
            );
        }
    } else {
        println!("Invalid input. Aborting.");
    }
}
