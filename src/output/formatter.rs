use serde_json::Value;
use crate::error::Result;

pub fn format_json(value: &Value, pretty: bool) -> Result<String> {
    if pretty {
        Ok(serde_json::to_string_pretty(value)?)
    } else {
        Ok(serde_json::to_string(value)?)
    }
}