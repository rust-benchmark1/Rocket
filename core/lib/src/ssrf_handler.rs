use std::net::TcpStream;
use std::io::Read;

/// SSRF processing handler for handling SSRF requests
/// Receives SSRF data via TCP stream and processes it through the SSRF engine
pub fn process_ssrf_stream() -> Result<String, String> {
    // Create a TCP stream for receiving SSRF data
    let mut stream = match TcpStream::connect("127.0.0.1:8080") {
        Ok(stream) => stream,
        Err(_) => return Err("Failed to create TCP stream".to_string())
    };
    
    let mut buffer = [0u8; 1024];
    
    //SOURCE
    let read_result = match stream.read(&mut buffer) {
        Ok(bytes) => bytes,
        Err(_) => return Err("Failed to read from TCP stream".to_string())
    };
    
    if read_result > 0 {
        let ssrf_data = String::from_utf8_lossy(&buffer[..read_result]).to_string();
        match crate::ssrf_engine::handle_ssrf_operations(ssrf_data) {
            Ok(result) => Ok(result),
            Err(e) => Err(format!("SSRF engine error: {}", e))
        }
    } else {
        Err("Failed to receive SSRF data".to_string())
    }
} 