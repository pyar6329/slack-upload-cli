use serde::Deserialize;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct ErrorInfo {
    #[serde(rename = "error")]
    pub error_type: String,
    #[serde(rename = "response_metadata")]
    pub metadata: ErrorDetail,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
pub struct ErrorDetail {
    #[serde(rename = "messages")]
    pub reasons: Vec<String>,
}
