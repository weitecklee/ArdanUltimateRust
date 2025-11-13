use std::collections::VecDeque;

use shared_data::{CollectorCommandV1, encode_v1};

use crate::sender::send_queue;
mod data_collector;
mod errors;
mod sender;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel::<CollectorCommandV1>();

    // Start the collector thread
    let _collector_thread = std::thread::spawn(move || {
        data_collector::collect_data(tx);
    });

    // Listen for commands to send
    let mut command_queue = VecDeque::with_capacity(120);
    while let Ok(command) = rx.recv() {
        let encoded = encode_v1(&command);
        command_queue.push_back(encoded);
        let _ = send_queue(&mut command_queue);

        // Send all queued commands
        // while let Some(command) = command_queue.pop_front() {
        //     if sender::send_command(&command).is_err() {
        //         println!("Error sending command");
        //         command_queue.push_front(command);
        //         break;
        //     }
        // }
        // let _ = sender::send_command(command);
    }
}
