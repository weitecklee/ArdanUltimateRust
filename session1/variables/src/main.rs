// fn greet(s: String) -> String {
//     println!("Hello {s}");
//     s
// }

// fn greet_borrow(s: &String) {
//     println!("{s}");
// }

// fn greet_borrow_mut(s: &mut String) {
//     *s = format!("Hello {s}");
// }

// fn main() {
//     let mut name = "Hello".to_string();
//     greet_borrow_mut(&mut name);
//     println!("{name}");
// }

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Stdin not working");
    input.trim().to_string()
}

fn main() {
    let input = read_line();
    println!("You typed: [{input}]")
}
