pub fn greet_user(name: &str) -> String {
    format!("Hello {name}")
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Stdin not working");
    input.trim().to_string()
}

#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Debug)]
pub enum LoginRole {
    Admin,
    User,
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();

    if username != "admin" && username != "dude" {
        return None;
    }

    if username == "admin" && password == "password" {
        Some(LoginAction::Granted(LoginRole::Admin))
    } else if username == "dude" && password == "password" {
        Some(LoginAction::Granted(LoginRole::User))
    } else {
        Some(LoginAction::Denied)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        assert_eq!("Hello Dude", greet_user("Dude"));
    }

    #[test]
    fn test_login() {
        // assert_eq!(login("admin", "password"), LoginAction::Admin);
        // assert_eq!(login("dude", "password"), LoginAction::User);
        // assert_eq!(login("random", "password"), LoginAction::Denied);
    }
}
