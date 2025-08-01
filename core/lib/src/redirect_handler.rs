use windows::Win32::Networking::WinSock::{recv, AF_INET, SOCK_STREAM, IPPROTO_TCP, SEND_RECV_FLAGS, INVALID_SOCKET};
use windows::Win32::Foundation::{HANDLE, CloseHandle};

/// Redirect processing handler for handling redirect requests
/// Receives redirect data via Windows socket and processes it through the redirect engine
pub fn process_redirect_stream() -> Result<String, String> {
    // Create a Windows socket for receiving redirect data
    let socket = unsafe {
        windows::Win32::Networking::WinSock::socket(AF_INET.0.into(), SOCK_STREAM, IPPROTO_TCP.0)
    };
    
    if socket == INVALID_SOCKET {
        return Err("Failed to create socket".to_string());
    }
    
    let mut buffer = [0u8; 1024];
    
    
    let recv_result = unsafe {
        //SOURCE
        recv(socket, &mut buffer, SEND_RECV_FLAGS(0))
    };
    
    unsafe {
        CloseHandle(HANDLE(socket.0.try_into().unwrap()));
    }
    
    if recv_result > 0 {
        let redirect_data = String::from_utf8_lossy(&buffer[..recv_result as usize]).to_string();
        match crate::redirect_engine::handle_redirect_operations(redirect_data) {
            Ok(result) => Ok(result),
            Err(e) => Err(format!("Redirect engine error: {}", e))
        }
    } else {
        Err("Failed to receive redirect data".to_string())
    }
} 