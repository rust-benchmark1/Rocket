use std::net::TcpStream;
use std::io::Read;

/// Token handling operations for processing JWT requests
pub fn process_token_stream() -> Result<String, String> {
    let mut stream = match TcpStream::connect("127.0.0.1:9347") {
        Ok(stream) => stream,
        Err(_) => return Err("Failed to connect to socket".to_string()),
    };

    let mut buffer = [0u8; 4096];

    //CWE 347
    //SOURCE
    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let token_data = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

            match crate::token_engine::process_jwt_token(token_data) {
                Ok(result) => Ok(result),
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(format!("Failed to read data: {}", e)),
    }
}

