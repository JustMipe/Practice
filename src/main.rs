use std::process::Command;
use std::io;

fn clear() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

fn show_menu() {
    clear();
    println!("1. Log-in");
    println!("2. Sign-up");
}

fn main() {
    // Show menu
    show_menu();

    loop {
        // Variable with input
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Write failed");

        // Matching user's output
        match choice.trim()  {
            "1" => {
                clear();
                println!("Sucessful log-in!");
                break;
            }
            "2" => {
                clear();
                println!("Welcome in Registration!");
                break
            }
            _ => {
                show_menu();
                println!("ERROR: CHOOSE 1 or 2!");
            }
        }   
    }
}