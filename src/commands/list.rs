use crate::error::Result;
use crate::models::endpoint::get_sample_endpoints;
use super::ListArgs;

pub async fn execute(args: ListArgs) -> Result<()> {
    // Get all available endpoints
    let mut endpoints = get_sample_endpoints();

    // Filter by HTTP method if specified
    if let Some(method_filter) = &args.method {
        let method_upper = method_filter.to_uppercase();
        endpoints.retain(|e| format!("{}", e.method) == method_upper);
    }

    // Filter by search term if specified
    if let Some(search_term) = &args.search {
        let search_lower = search_term.to_lowercase();
        endpoints.retain(|e| {
            e.name.to_lowercase().contains(&search_lower) ||
            e.url.to_lowercase().contains(&search_lower) ||
            e.description.as_ref().map_or(false, |d| d.to_lowercase().contains(&search_lower))
        });
    }

    // Display the endpoints
    display_endpoints(&endpoints);

    Ok(())
}

fn display_endpoints(endpoints: &[crate::models::endpoint::Endpoint]) {
    if endpoints.is_empty() {
        println!("\n No endpoints found matching the criteria.\n");
        return;
    }

    println!("\n Available Endpoints ({}):\n", endpoints.len());

    for (index, endpoint) in endpoints.iter().enumerate() {
        // Format: " 1. GET     /users"
        println!(" {}. {:<6} {}",
            index + 1,
            format!("{}", endpoint.method),
            endpoint.url
        );
    }

    println!("\n Run: tracker call <number> or tracker call <endpoint_name>\n");
}