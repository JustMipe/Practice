use std::process::Command;
use std::io;

/// Clear terminal like 'cls'
fn clear() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

/// Show menu and return input
fn show_menu() -> String {
    let mut choice = String::new();
    println!("1. Registrovat se");
    println!("2. Zobrazit seznam");
    println!("3. Ukončit");
    io::stdin().read_line(&mut choice).unwrap();
    clear();
    return choice;
}

fn registrace() -> String {
    let mut prezdivka = String::new();
    println!("Zadej přezdívku:");
    io::stdin().read_line(&mut prezdivka).unwrap();
    clear();
    return prezdivka;
}

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
                println!("ÚSPĚŠNĚ REGISTROVÁNO!");
               
                break;
            }
            "2" => { loop {
                    for x in &user_list {
                        print!("{}", x);
                    }
                    println!("Přejete se vrátit zpět do menu? (ano/ne)");
                    let mut vratit_zpet = String::new();
                    io::stdin().read_line(&mut vratit_zpet).unwrap();
                    match vratit_zpet.trim() {
                        "ano" => break,
                        _ => continue,
                    }
                }
            }
            "3" => {
                clear();
                println!("PROGRAM UKONČEN!");
                break;
            }
            _ => println!("ERROR"),
        }


    }
    
    


}