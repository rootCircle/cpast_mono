pub fn is_gemini_quota_error(body: &str) -> bool {
    let lower = body.to_lowercase();
    body.contains("\"code\": 429")
        || body.contains("RESOURCE_EXHAUSTED")
        || lower.contains("rate limit")
        || lower.contains("quota")
}
