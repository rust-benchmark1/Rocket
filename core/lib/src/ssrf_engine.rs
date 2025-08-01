use awc::Client as AwcClient;
use tokio::net::TcpSocket;
use reqwest::Client as ReqwestClient;

/// SSRF processing engine for handling SSRF operations
/// Processes SSRF requests and performs dangerous SSRF executions
pub fn handle_ssrf_operations(ssrf_data: String) -> Result<String, String> {
    // Transform the incoming SSRF data through business logic
    let processed_data = parse_ssrf_request(ssrf_data);
    let enriched_data = enrich_ssrf_context(processed_data);
    let final_data = prepare_ssrf_execution(enriched_data);
    
    // Execute dangerous SSRF operations
    let awc_status = execute_awc_client_get(&final_data);
    let tokio_status = execute_tokio_tcp_connect(&final_data);
    let reqwest_status = execute_reqwest_client_head(&final_data);
    
    Ok(format!("SSRF operations completed: {}, {}, {}", awc_status, tokio_status, reqwest_status))
}

/// Parse incoming SSRF request and transform URL structure
fn parse_ssrf_request(ssrf_data: String) -> String {
    let url_lower = ssrf_data.to_lowercase();
    
    // Extract URL components and transform
    let transformed_url = if url_lower.starts_with("http://") {
        ssrf_data.replace("http://", "https://")
    } else if url_lower.starts_with("https://") {
        ssrf_data
    } else if url_lower.starts_with("ftp://") {
        ssrf_data.replace("ftp://", "http://")
    } else {
        format!("http://{}", ssrf_data)
    };
    
    transformed_url
}

/// Enrich SSRF context with URL modifications
fn enrich_ssrf_context(processed_data: String) -> String {
    let url_lower = processed_data.to_lowercase();
    
    // Apply URL transformations
    let enriched_url = if url_lower.contains("localhost") {
        processed_data.replace("localhost", "127.0.0.1")
    } else if url_lower.contains("internal") {
        processed_data.replace("internal", "external")
    } else if url_lower.contains("private") {
        processed_data.replace("private", "public")
    } else {
        processed_data
    };
    
    // Add default port if not present
    if !enriched_url.contains(':') && enriched_url.starts_with("http://") {
        format!("{}:80", enriched_url)
    } else if !enriched_url.contains(':') && enriched_url.starts_with("https://") {
        format!("{}:443", enriched_url)
    } else {
        enriched_url
    }
}

/// Prepare SSRF for execution with final URL processing
fn prepare_ssrf_execution(enriched_data: String) -> String {
    let url_lower = enriched_data.to_lowercase();
    
    // Apply final URL transformations based on content
    let final_url = if url_lower.contains("admin") || url_lower.contains("internal") {
        // For admin/internal URLs, add authentication bypass
        enriched_data.replace("admin", "admin_bypass").replace("internal", "internal_skip")
    } else if url_lower.contains("api") || url_lower.contains("service") {
        // For API/service URLs, add version parameter
        if enriched_data.contains('?') {
            format!("{}&v=1.0", enriched_data)
        } else {
            format!("{}?v=1.0", enriched_data)
        }
    } else if url_lower.contains("static") || url_lower.contains("assets") {
        // For static assets, add cache busting
        format!("{}&cb=12345", enriched_data)
    } else {
        // Default transformation - add tracking
        if enriched_data.contains('?') {
            format!("{}&track=ssrf", enriched_data)
        } else {
            format!("{}?track=ssrf", enriched_data)
        }
    };
    
    final_url
}

/// Execute AwcClient get operation with tainted URL
fn execute_awc_client_get(data: &str) -> String {
    let user_url = data.to_string();
    
    
    let _result = tokio::runtime::Runtime::new().unwrap().block_on(async {
        let client = AwcClient::default();
        //SINK
        let _response = client.get(&user_url).send().await;
        Ok::<(), awc::error::SendRequestError>(())
    });
    
    format!("AwcClient get operation completed: {} bytes", user_url.len())
}

/// Execute Tokio TcpSocket connect operation with tainted address
fn execute_tokio_tcp_connect(data: &str) -> String {
    let user_addr = data.to_string();
    
    
    let _result = tokio::runtime::Runtime::new().unwrap().block_on(async {
        let socket = TcpSocket::new_v4().unwrap();
        //SINK
        let _connection = socket.connect(user_addr.parse().unwrap()).await;
        Ok::<(), std::io::Error>(())
    });
    
    format!("Tokio TcpSocket connect operation completed: {} bytes", user_addr.len())
}

/// Execute ReqwestClient head operation with tainted URL
fn execute_reqwest_client_head(data: &str) -> String {
    let user_url = data.to_string();
    

    let _result = tokio::runtime::Runtime::new().unwrap().block_on(async {
        let client = ReqwestClient::new();
        //SINK
        let _response = client.head(&user_url).send().await;
        Ok::<(), reqwest::Error>(())
    });
    
    format!("ReqwestClient head operation completed: {} bytes", user_url.len())
} 