use std::net::TcpStream;
use std::io::Read;

/// Allocation handling operations for processing memory reservation requests
pub fn process_allocation_stream() -> Result<String, String> {
    let mut stream = match TcpStream::connect("127.0.0.1:9789") {
        Ok(stream) => stream,
        Err(_) => return Err("Failed to connect to socket".to_string()),
    };

    let mut buffer = [0u8; 1024];

    //CWE 789
    //SOURCE
    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let capacity: usize = String::from_utf8_lossy(&buffer[..bytes_read]).parse().unwrap_or(0);

            match crate::allocation_engine::reserve_memory_capacity(capacity) {
                Ok(result) => Ok(result),
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(format!("Failed to read data: {}", e)),
    }
}
