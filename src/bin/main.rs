use projekt::functions::*;
use std::io;



fn main() {
    clear(); // Clear terminal for first use
    let mut user_list: Vec<String> = vec![];

    loop {
        clear();
        let volba = show_menu();  // Zavolá menu a input
        match volba.trim() {
            "1" => loop {
                let udaje = registrace();
                user_list.push(udaje);
                break;
            }
            "2" => { loop {
                    for x in &user_list {
                        print!("{}", x);
                    }
                    println!("Would you like back to menu? (yes/no)");
                    let mut vratit_zpet = String::new();
                    io::stdin().read_line(&mut vratit_zpet).unwrap();
                    match vratit_zpet.trim() {
                        "yes" => break,
                        _ => continue,
                    }
                }
            }
            "3" => {
                clear();
                println!("PROGRAM CLOSED!");
                break;
            }
            _ => println!("ERROR"),
        }
    }
}