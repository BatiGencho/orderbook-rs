use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub message: String,
    pub status: String,
    pub errors: Option<Vec<FieldError>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldError {
    pub field: String,
    pub field_errors: Vec<String>,
}
