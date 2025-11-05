// use tokio::runtime;

async fn hello() {
    println!("Hello Tokio!");
}

// #[tokio::main(flavor = "current_thread")]
// #[tokio::main(flavor = "multi_thread")]
#[tokio::main]
async fn main() {
    hello().await;
}

// fn main() {
// let rt = runtime::Builder::new_current_thread()
//     .enable_all()
//     .build()
//     .unwrap();

// let rt = runtime::Builder::new_multi_thread()
//     .enable_all()
//     .worker_threads(4)
//     .build()
//     .unwrap();

// rt.block_on(hello());

// }
