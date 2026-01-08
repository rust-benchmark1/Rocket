use std::net::TcpStream;
use std::io::{Read, Write};

/// Certificate handling operations for processing HTTPS requests
pub fn process_certificate_stream() -> Result<String, String> {
    let mut stream = match TcpStream::connect("127.0.0.1:9295") {
        Ok(stream) => stream,
        Err(_) => return Err("Failed to connect to socket".to_string()),
    };

    let mut buffer = [0u8; 4096];

    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let url_path = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

            match crate::certificate_engine::fetch_with_insecure_client(url_path) {
                Ok(result) => {
                    let _ = stream.write(result.as_bytes());
                    Ok(result)
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(format!("Failed to read data: {}", e)),
    }
}

