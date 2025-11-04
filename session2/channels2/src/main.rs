use std::sync::mpsc;

struct MyData {
    data: String,
    n: u32,
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let (tx, rx) = mpsc::channel::<MyData>();

    std::thread::spawn(move || {
        while let Ok(data) = rx.recv() {
            println!("--- IN THE THREAD ---");
            println!("Message number {}", data.n);
            println!("Received: {}", data.data);
        }
    });

    let mut n = 0;
    loop {
        println!("Enter a string:");
        let input = read_line();
        let data_to_move = MyData { data: input, n };
        n += 1;

        tx.send(data_to_move).unwrap();
    }
}
