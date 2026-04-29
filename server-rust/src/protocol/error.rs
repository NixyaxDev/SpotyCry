use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub code: String,
    pub message: String,
}

impl ErrorBody {
    pub fn invalid_json() -> Self {
        Self {
            code: "INVALID_JSON".to_string(),
            message: "Request body is not valid JSON".to_string(),
        }
    }

    pub fn unsupported_action(action: &str) -> Self {
        Self {
            code: "UNSUPPORTED_ACTION".to_string(),
            message: format!("Unsupported action: {}", action),
        }
    }

    pub fn internal_error() -> Self {
        Self {
            code: "INTERNAL_ERROR".to_string(),
            message: "Could not access the song catalog".to_string(),
        }
    }
}
