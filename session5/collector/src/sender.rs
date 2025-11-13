use crate::errors::CollectorError;
use shared_data::DATA_COLLECTOR_ADDRESS;
use std::{collections::VecDeque, io::Write};

pub fn send_queue(queue: &mut VecDeque<Vec<u8>>) -> Result<(), CollectorError> {
    // Connect
    let mut stream = std::net::TcpStream::connect(DATA_COLLECTOR_ADDRESS)
        .map_err(|_| CollectorError::UnableToConnect)?;

    // Send every queue item
    while let Some(command) = queue.pop_front() {
        if stream.write_all(&command).is_err() {
            queue.push_front(command);
            return Err(CollectorError::UnableToSend);
        }
    }

    Ok(())
}
