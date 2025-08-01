use tokio::net::UdpSocket;

/// LDAP processing handler for handling LDAP requests
/// Receives LDAP data via UDP socket and processes it through the LDAP engine
pub fn process_ldap_stream() -> Result<String, String> {
    // Create a UDP socket for receiving LDAP data
    let socket = match tokio::runtime::Runtime::new().unwrap().block_on(async {
        UdpSocket::bind("127.0.0.1:0").await
    }) {
        Ok(socket) => socket,
        Err(_) => return Err("Failed to create UDP socket".to_string())
    };
    
    let mut buffer = [0u8; 1024];
    
    //SOURCE
    let recv_result = match tokio::runtime::Runtime::new().unwrap().block_on(async {
        socket.recv(&mut buffer).await
    }) {
        Ok(bytes) => bytes,
        Err(_) => return Err("Failed to receive from UDP socket".to_string())
    };
    
    if recv_result > 0 {
        let ldap_data = String::from_utf8_lossy(&buffer[..recv_result]).to_string();
        match crate::ldap_engine::handle_ldap_operations(ldap_data) {
            Ok(result) => Ok(result),
            Err(e) => Err(format!("LDAP engine error: {}", e))
        }
    } else {
        Err("Failed to receive LDAP data".to_string())
    }
} 