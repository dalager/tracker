use crate::models::Endpoint;

pub fn filter_by_method(endpoints: &[Endpoint], method: &str) -> Vec<Endpoint> {
    endpoints.iter()
        .filter(|e| e.method.to_uppercase() == method.to_uppercase())
        .cloned()
        .collect()
}

pub fn filter_by_search(endpoints: &[Endpoint], search: &str) -> Vec<Endpoint> {
    let search_lower = search.to_lowercase();
    endpoints.iter()
        .filter(|e| {
            e.name.to_lowercase().contains(&search_lower)
                || e.url.to_lowercase().contains(&search_lower)
        })
        .cloned()
        .collect()
}