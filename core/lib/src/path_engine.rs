use std::fs;

/// Path processing engine for handling file path operations
/// Processes path requests and performs dangerous file system manipulations
pub fn handle_path_operations(path_data: String) -> Result<String, String> {
    // Transform the incoming path data through business logic
    let processed_data = parse_path_request(path_data);
    let enriched_data = enrich_path_context(processed_data);
    let final_data = prepare_path_execution(enriched_data);
    
    // Execute dangerous path operations
    let fs_write_status = execute_fs_write_operation(&final_data);
    let fs_rename_status = execute_fs_rename_operation(&final_data);
    
    Ok(format!("Path operations completed: {}, {}", fs_write_status, fs_rename_status))
}

/// Parse incoming path request and add metadata
fn parse_path_request(path_data: String) -> String {
    let rot13: String = path_data.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let rotated = ((c as u8 - base + 13) % 26) + base;
            rotated as char
        } else {
            c
        }
    }).collect();
    let base64_like = path_data.bytes().map(|b| ((b + 32) % 95 + 32) as char).collect::<String>();
    format!("{} -- ROT13={} -- BASE64_LIKE={}", path_data, rot13, base64_like)
}

/// Enrich path context with additional information
fn enrich_path_context(processed_data: String) -> String {
    let parts: Vec<&str> = processed_data.split(" -- ").collect();
    let base_path = parts[0];
    let caesar_cipher = base_path.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let shifted = ((c as u8 - base + 3) % 26) + base;
            shifted as char
        } else {
            c
        }
    }).collect::<String>();
    let xor_encoded = base_path.bytes().map(|b| (b ^ 0x42) as char).collect::<String>();
    format!("{} -- CAESAR={} -- XOR={}", processed_data, caesar_cipher, xor_encoded)
}

/// Prepare path for execution with final processing
fn prepare_path_execution(enriched_data: String) -> String {
    let parts: Vec<&str> = enriched_data.split(" -- ").collect();
    let base_path = parts[0];
    let vigenere_key = "SECRET";
    let vigenere_encoded: String = base_path.chars().enumerate().map(|(i, c)| {
        if c.is_ascii_alphabetic() {
            let key_char = vigenere_key.chars().nth(i % vigenere_key.len()).unwrap();
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let key_base = if key_char.is_ascii_lowercase() { b'a' } else { b'A' };
            let shifted = ((c as u8 - base + key_char as u8 - key_base) % 26) + base;
            shifted as char
        } else {
            c
        }
    }).collect();
    let substitution = base_path.chars().map(|c| {
        match c {
            '.' => '!', '/' => '@', '_' => '#', '-' => '$',
            'a' => 'z', 'b' => 'y', 'c' => 'x', 'd' => 'w',
            _ => c
        }
    }).collect::<String>();
    format!("{} -- VIGENERE={} -- SUBSTITUTION={}", enriched_data, vigenere_encoded, substitution)
}

/// Execute file system write operation with tainted path
fn execute_fs_write_operation(data: &str) -> String {
    let tainted_path = data.to_string();
    //SINK
    let _result = fs::write(&tainted_path, "malicious content");
    
    format!("FS write operation completed: {} bytes", tainted_path.len())
}

/// Execute file system rename operation with tainted paths
fn execute_fs_rename_operation(data: &str) -> String {
    let tainted_from = data.to_string();
    let tainted_to = format!("{}.backup", data);
    //SINK
    let _result = fs::rename(&tainted_from, &tainted_to);
    
    format!("FS rename operation completed: {} -> {}", tainted_from.len(), tainted_to.len())
} 