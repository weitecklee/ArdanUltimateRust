use std::fmt::Debug;

fn just_print_it<T, U>(x: T, y: U)
where
    T: ToString + Debug,
    U: ToString,
{
    println!("{} and {}", x.to_string(), y.to_string());
}

fn main() {
    just_print_it("Hello!", "Goodbye!");
    just_print_it(5, 8);
}
