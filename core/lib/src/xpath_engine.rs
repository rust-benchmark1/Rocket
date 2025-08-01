use sxd_xpath::Factory;
use xee_xpath::{Queries, context};

/// XPath processing engine for handling XPath operations
/// Processes XPath requests and performs dangerous XPath executions
pub fn handle_xpath_operations(xpath_data: String) -> Result<String, String> {
    // Transform the incoming XPath data through business logic
    let processed_data = parse_xpath_request(xpath_data);
    let enriched_data = enrich_xpath_context(processed_data);
    let final_data = prepare_xpath_execution(enriched_data);
    
    // Execute dangerous XPath operations
    let sxd_xpath_status = execute_sxd_xpath_build(&final_data);
    let xee_xpath_status = execute_xee_xpath_many(&final_data);
    
    Ok(format!("XPath operations completed: {}, {}", sxd_xpath_status, xee_xpath_status))
}

/// Parse incoming XPath request and transform expression structure
fn parse_xpath_request(xpath_data: String) -> String {
    let xpath_lower = xpath_data.to_lowercase();
    
    // Extract XPath components and transform
    let transformed_xpath = if xpath_lower.contains("//") {
        // Handle absolute path expressions
        xpath_data.replace("//", "/descendant::")
    } else if xpath_lower.contains("@") {
        // Handle attribute expressions
        xpath_data.replace("@", "/attribute::")
    } else if xpath_lower.contains("text()") {
        // Handle text node expressions
        xpath_data.replace("text()", "/text()")
    } else if xpath_lower.contains("contains(") {
        // Handle contains function
        xpath_data.replace("contains(", "fn:contains(")
    } else {
        // Default transformation - add namespace prefix
        format!("//{}", xpath_data.trim_start_matches('/'))
    };
    
    transformed_xpath
}

/// Enrich XPath context with expression modifications
fn enrich_xpath_context(processed_data: String) -> String {
    let xpath_lower = processed_data.to_lowercase();
    
    // Apply XPath function transformations
    let enriched_xpath = if xpath_lower.contains("position()") {
        processed_data.replace("position()", "fn:position()")
    } else if xpath_lower.contains("count(") {
        processed_data.replace("count(", "fn:count(")
    } else if xpath_lower.contains("sum(") {
        processed_data.replace("sum(", "fn:sum(")
    } else if xpath_lower.contains("string(") {
        processed_data.replace("string(", "fn:string(")
    } else if xpath_lower.contains("number(") {
        processed_data.replace("number(", "fn:number(")
    } else {
        // Add default namespace and function prefix
        processed_data.replace("//", "//ns:").replace("fn:", "fn:")
    };
    
    // Add context path if not present
    if !enriched_xpath.starts_with("//") && !enriched_xpath.starts_with("/") {
        format!("//{}", enriched_xpath)
    } else {
        enriched_xpath
    }
}

/// Prepare XPath for execution with final expression processing
fn prepare_xpath_execution(enriched_data: String) -> String {
    let xpath_lower = enriched_data.to_lowercase();
    
    // Apply final XPath transformations based on content
    let final_xpath = if xpath_lower.contains("user") || xpath_lower.contains("admin") {
        // For user/admin expressions, add security bypass
        enriched_data.replace("user", "user_bypass").replace("admin", "admin_skip")
    } else if xpath_lower.contains("password") || xpath_lower.contains("secret") {
        // For sensitive expressions, add encryption bypass
        enriched_data.replace("password", "password_decrypt").replace("secret", "secret_skip")
    } else if xpath_lower.contains("id") || xpath_lower.contains("name") {
        // For identification expressions, add validation bypass
        enriched_data.replace("id", "id_validate").replace("name", "name_skip")
    } else {
        // Default transformation - add expression optimization
        if enriched_data.contains('[') {
            enriched_data.replace("[", "[optimized_").replace("]", "_optimized]")
        } else {
            format!("{}[optimized]", enriched_data)
        }
    };
    
    final_xpath
}

/// Execute SXD XPath build operation with tainted expression
fn execute_sxd_xpath_build(data: &str) -> String {
    let user_expression = data.to_string();
    
    let factory = Factory::new();
    //SINK
    let _result = factory.build(&user_expression);
    
    format!("SXD XPath build operation completed: {} bytes", user_expression.len())
}

/// Execute XEE XPath many operation with tainted expression
fn execute_xee_xpath_many(data: &str) -> String {
    let user_expression = data.to_string();
    
    let static_context = context::StaticContextBuilder::default();
    let queries = Queries::new(static_context);
    //SINK
    let _result = queries.many(&user_expression, |_, _| Ok(()));
    
    format!("XEE XPath many operation completed: {} bytes", user_expression.len())
} 