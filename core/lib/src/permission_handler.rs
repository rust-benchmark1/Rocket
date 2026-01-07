use std::net::TcpStream;
use std::io::Read;

/// Permission handling operations for processing file permission changes
/// Processes incoming permission modification requests via TCP sockets
pub fn process_permission_stream() -> Result<String, String> {
    let mut stream = match TcpStream::connect("127.0.0.1:9732") {
        Ok(stream) => stream,
        Err(_) => return Err("Failed to connect to socket".to_string()),
    };

    let mut buffer = [0u8; 1024];

    //CWE 732
    //SOURCE
    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let file_path = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

            match crate::permission_engine::modify_file_permissions(file_path) {
                Ok(result) => Ok(result),
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(format!("Failed to read data: {}", e)),
    }
}
