use std::net::TcpStream;
use std::io::Read;

/// Serializer handling operations for processing binary module data
/// Processes incoming deserialization requests via TCP sockets
pub fn deserialize_data() -> Result<String, String> {
    // Connect to TCP socket for receiving serialized module data
    let mut stream = match TcpStream::connect("127.0.0.1:9502") {
        Ok(stream) => stream,
        Err(_) => return Err("Failed to connect to socket".to_string()),
    };

    let mut buffer = Vec::new();

    //CWE 502
    //SOURCE
    match stream.read_to_end(&mut buffer) {
        Ok(_) => {
            let wasm_bytes = buffer;

            match crate::serializer_engine::process_module_deserialization(wasm_bytes) {
                Ok(result) => Ok(format!("Module deserialization completed: {}", result)),
                Err(e) => Err(format!("Module deserialization failed: {}", e)),
            }
        },
        Err(e) => Err(format!("Failed to read module data: {}", e)),
    }
}
