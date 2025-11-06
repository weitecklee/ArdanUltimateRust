#![allow(dead_code)]
use serde::Deserialize;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
enum UsersError {
    #[error("No users found")]
    NoUsers,
    #[error("Too many users found")]
    TooManyUsers,
}

fn maybe_read_a_file() -> Result<String, std::io::Error> {
    let my_file = Path::new("myfile.txt");
    std::fs::read_to_string(my_file)
}

fn file_to_uppercase() -> Result<String, std::io::Error> {
    let contents = maybe_read_a_file()?;
    Ok(contents.to_uppercase())
}

#[derive(Deserialize)]
struct User {
    user: String,
}

type GenericResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// fn load_users() -> anyhow::Result<Vec<User>> {
// fn load_users() -> GenericResult<Vec<User>> {
fn load_users() -> Result<Vec<User>, UsersError> {
    let my_path = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_path).map_err(|_| UsersError::NoUsers)?;
    let users: Vec<User> = serde_json::from_str(&raw_text).map_err(|_| UsersError::TooManyUsers)?;
    Ok(users)
}

fn main() {
    if let Ok(content) = file_to_uppercase() {
        println!("Contents: {content}");
    }

    let my_file = Path::new("myfile.txt");
    let content = std::fs::read_to_string(my_file);
    match content {
        Ok(contents) => println!("{contents}"),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => println!("File not found: myfile.txt"),
            _ => println!("Error: {e:#?}"),
        },
    }
}
