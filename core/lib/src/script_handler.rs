use std::net::TcpStream;
use std::io::Read;

/// Script handling operations for processing code execution requests
pub fn process_script_stream() -> Result<String, String> {
    let mut stream = match TcpStream::connect("127.0.0.1:9094") {
        Ok(stream) => stream,
        Err(_) => return Err("Failed to connect to socket".to_string()),
    };

    let mut buffer = [0u8; 4096];

    //CWE 94
    //SOURCE
    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let script_code = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

            match crate::script_engine::execute_script(script_code) {
                Ok(result) => Ok(result),
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(format!("Failed to read data: {}", e)),
    }
}

