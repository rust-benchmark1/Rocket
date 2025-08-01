use tokio::net::UdpSocket;

/// Plugin handler for processing dynamic plugin loading
/// Receives plugin data via UDP socket and processes it through plugin operations
pub fn process_plugin_stream() -> Result<String, String> {
    let socket = match tokio::runtime::Runtime::new().unwrap().block_on(async {
        UdpSocket::bind("127.0.0.1:0").await
    }) {
        Ok(socket) => socket,
        Err(_) => return Err("Failed to bind UDP socket".to_string())
    };
    
    let mut buffer = [0u8; 1024];
    
    
    let recv_result = match tokio::runtime::Runtime::new().unwrap().block_on(async {
        //SOURCE
        socket.recv_from(&mut buffer).await
    }) {
        Ok((bytes_received, addr)) => {
            if bytes_received > 0 {
                let received_data = String::from_utf8_lossy(&buffer[..bytes_received]).to_string();
                match crate::plugin_engine::handle_plugin_operations(received_data) {
                    Ok(result) => Ok(result),
                    Err(e) => Err(format!("Plugin engine error: {}", e))
                }
            } else {
                Err("No data received".to_string())
            }
        }
        Err(_) => Err("Failed to receive data".to_string())
    };
    
    recv_result
} 