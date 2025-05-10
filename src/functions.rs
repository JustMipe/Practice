use std::process::Command;
use std::io;

/// Clear terminal like 'cls'
pub fn clear() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

/// Show menu and return input
pub fn show_menu() -> String {
    let mut choice = String::new();
    println!("1. Register");
    println!("2. Show list of users");
    println!("3. Quit");
    io::stdin().read_line(&mut choice).unwrap();
    clear();
    return choice;
}

pub fn registrace() -> String {
    let mut prezdivka = String::new();
    println!("Set nickname:");
    io::stdin().read_line(&mut prezdivka).unwrap();
    clear();
    return prezdivka;
}