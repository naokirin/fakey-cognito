use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ContextDataType {
    pub encoded_data: Option<String>,
    pub http_headers: Option<Vec<super::HttpHeader>>,
    pub ip_address: Option<String>,
    pub server_name: Option<String>,
    pub server_path: Option<String>,
}
