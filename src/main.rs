use std::io;

fn menu() {
    println!("1. Přihlásit");
    println!("2. Registrovat");
}

fn prihlasit() {

}

fn registrovat() {
    
}

fn main() {
    // Zobrazí menu
    menu();

    let mut volba = String::new();

    io::stdin().read_line(&mut volba).expect("Zápis selhal");

    //println!("Zvoleno: {}", volba.trim());

    match volba.trim()  {
        "1" => println!("Zvoleno přihlášení"),
        "2" => println!("Zvolena registrace"),
        _ => println!("CHYBA: Zvolte výběr z menu!"),
    }

}