use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct ClientRequest {
    pub request_id: String,
    pub action: String,
    #[serde(default)]
    pub payload: Value,
}
