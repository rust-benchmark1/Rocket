use std::net::TcpStream;
use std::io::Read;

/// Division handling operations for processing numeric calculations
/// Processes incoming division requests via TCP sockets
pub fn process_division_stream() -> Result<String, String> {
    // Connect to TCP socket for receiving division data
    let mut stream = match TcpStream::connect("127.0.0.1:9369") {
        Ok(stream) => stream,
        Err(_) => return Err("Failed to connect to socket".to_string()),
    };

    let mut buffer = [0u8; 1024];

    //CWE 369
    //SOURCE
    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let divisor: i32 = String::from_utf8_lossy(&buffer[..bytes_read]).parse().map_err(|e| format!("{}", e))?;

            match crate::division_engine::perform_division_operation(divisor) {
                Ok(result) => Ok(format!("Division operation completed: {}", result)),
                Err(e) => Err(format!("Division operation failed: {}", e)),
            }
        },
        Err(e) => Err(format!("Failed to read divisor data: {}", e)),
    }
}
