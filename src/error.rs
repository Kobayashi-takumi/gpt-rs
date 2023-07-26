use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorContent,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorContent {
    pub code: String,
    pub message: String,
    #[serde(rename = "type")]
    pub type_: String,
}
