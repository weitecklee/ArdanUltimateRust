// fn greet(s: String) -> String {
//     println!("Hello {s}");
//     s
// }

// fn greet_borrow(s: &String) {
//     println!("{s}");
// }

fn greet_borrow_mut(s: &mut String) {
    *s = format!("Hello {s}");
}

fn main() {
    let mut name = "Hello".to_string();
    greet_borrow_mut(&mut name);
    println!("{name}");
}
