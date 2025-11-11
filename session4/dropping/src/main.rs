#![allow(dead_code, unused_variables)]

struct MyStruct {
    n: i32,
}

impl MyStruct {
    fn new(n: i32) -> Self {
        println!("Constructing {n}");
        Self { n }
    }
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping {}", self.n);
    }
}

struct HasDroppables {
    x: MyStruct,
}

fn move_me(x: MyStruct) {
    println!("End of function");
}

fn main() {
    let x = MyStruct::new(1);
    {
        let y = MyStruct::new(2);
    }
    move_me(x);
    println!("Back to main function");
    let has_drop = HasDroppables {
        x: MyStruct::new(3),
    };
    println!("Ending main function");
}
