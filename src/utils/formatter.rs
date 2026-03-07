use crate::models::Endpoint;

pub fn format_endpoints(endpoints: &[Endpoint]) -> String {
    endpoints.iter()
        .enumerate()
        .map(|(i, e)| format!("{}. {} {}", i + 1, e.method, e.url))
        .collect::<Vec<_>>()
        .join("\n")
}