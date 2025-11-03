fn main() {
    let mut tries = 0;
    loop {
        println!("Enter your username:");
        let username = authentication::read_line();
        println!("Enter your password:");
        let password = authentication::read_line();

        match authentication::login(&username, &password) {
            Some(authentication::LoginAction::Granted(login_role)) => {
                match login_role {
                    authentication::LoginRole::Admin => println!("Admin"),
                    authentication::LoginRole::User => println!("User"),
                }
                break;
            }
            Some(authentication::LoginAction::Denied) => {
                // Do nothing
            }
            None => {
                println!("New user system");
            }
        }

        println!("Incorrect username or password");
        tries += 1;
        if tries >= 3 {
            println!("Too many failed logins");
            break;
        }
    }
}
