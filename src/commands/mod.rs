use clap::Parser;

pub mod list;
pub mod call;

#[derive(Parser, Debug, Clone)]
pub struct ListArgs {
    #[arg(long, help = "Filter endpoints by HTTP method (GET, POST, PUT, DELETE, PATCH)")]
    pub method: Option<String>,

    #[arg(long, help = "Search endpoints by name or path")]
    pub search: Option<String>,
}

#[derive(Parser, Debug, Clone)]
pub struct CallArgs {
    #[arg(help = "Endpoint identifier (number or name)")]
    pub endpoint: String,

    #[arg(short, long, help = "HTTP method (GET, POST, PUT, DELETE, PATCH)")]
    pub method: Option<String>,

    #[arg(short = 'H', long, help = "Custom header (repeatable): Key: Value")]
    pub header: Vec<String>,

    #[arg(short, long, help = "Request body as JSON string")]
    pub body: Option<String>,

    #[arg(long, help = "Load request body from file")]
    pub body_file: Option<String>,

    #[arg(long, help = "API key for authentication")]
    pub api_key: Option<String>,

    #[arg(long, help = "Bearer token for authentication")]
    pub bearer_token: Option<String>,

    #[arg(long, help = "Output format (json, table)", default_value = "json")]
    pub format: String,

    #[arg(short, long, help = "Pretty-print JSON output")]
    pub pretty: bool,

    #[arg(long, help = "Show response headers")]
    pub headers: bool,

    #[arg(long, help = "Comma-separated fields to display")]
    pub fields: Option<String>,
}