use crate::pass_entry::{prompt, read_passwords_from_file, ServiceInfo};

mod pass_entry;

fn clr() {
    print!("{}[2J", 27 as char);
}

fn main() {
    clr();

    let ascii = r#"
    ___                                    _                    _ _   
    / _ \__ _ ___ _____      _____  _ __ __| | /\   /\__ _ _   _| | |_ 
   / /_)/ _` / __/ __\ \ /\ / / _ \| '__/ _` | \ \ / / _` | | | | | __|
  / ___/ (_| \__ \__ \\ V  V / (_) | | | (_| |  \ V / (_| | |_| | | |_ 
  \/    \__,_|___/___/ \_/\_/ \___/|_|  \__,_|   \_/ \__,_|\__,_|_|\__|
                                                                       
    "#;

    println!("{ascii}");

    loop {
        println!("\n\nPassword Manager Menu:");
        println!("1. Add Password");
        println!("2. List Passwords");
        println!("3. Search Password");
        println!("4. Exit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                clr();

                let entry = ServiceInfo::new(
                    prompt("Service: "),
                    prompt("Username: "),
                    prompt("Password: "),
                );

                println!("Entry added succesfully.");

                entry.write_to_file();
            }
            "2" => {
                clr();

                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprint!("Error reading passwords: {}", err);
                    Vec::new()
                });

                for item in &services {
                    println!(
                        "
- Service: {}
- Username: {} 
- Password: {}",
                        item.service, item.username, item.password
                    );
                }
            }
            "3" => {
                clr();

                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords: {}", err);
                    Vec::new()
                });

                let search = prompt("Search: ");

                let mut count = 0;

                for item in &services {
                    if item.service.as_str() == search.as_str() {
                        println!(
                            "
- Service: {}
- Username: {} 
- Password: {}",
                            item.service, item.username, item.password
                        );

                        count += 1;
                    }
                }

                if count == 0 {
                    println!("Found {} matches.", count);
                }
            }
            "4" => {
                clr();
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid option! Please try again.")
            }
        };
    }
}
