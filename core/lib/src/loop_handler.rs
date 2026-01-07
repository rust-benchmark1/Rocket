use std::net::TcpStream;
use std::io::Read;

/// Loop handling operations for processing iteration requests
pub fn process_loop_stream() -> Result<String, String> {
    let mut stream = match TcpStream::connect("127.0.0.1:9606") {
        Ok(stream) => stream,
        Err(_) => return Err("Failed to connect to socket".to_string()),
    };

    let mut buffer = [0u8; 1024];

    //CWE 606
    //SOURCE
    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let iterations: usize = String::from_utf8_lossy(&buffer[..bytes_read]).parse().unwrap_or(0);

            match crate::loop_engine::execute_loop_operation(iterations) {
                Ok(result) => Ok(result),
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(format!("Failed to read data: {}", e)),
    }
}
