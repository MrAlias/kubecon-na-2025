// Placeholder for tracing utilities
// This can be extended with tracing header extraction/injection

pub fn extract_tracing_headers(req: &actix_web::HttpRequest) -> std::collections::HashMap<String, String> {
    let mut headers = std::collections::HashMap::new();
    
    // B3 headers
    if let Some(val) = req.headers().get("x-b3-traceid") {
        if let Ok(s) = val.to_str() {
            headers.insert("x-b3-traceid".to_string(), s.to_string());
        }
    }
    if let Some(val) = req.headers().get("x-b3-spanid") {
        if let Ok(s) = val.to_str() {
            headers.insert("x-b3-spanid".to_string(), s.to_string());
        }
    }
    
    // W3C Trace Context
    if let Some(val) = req.headers().get("traceparent") {
        if let Ok(s) = val.to_str() {
            headers.insert("traceparent".to_string(), s.to_string());
        }
    }
    
    headers
}
