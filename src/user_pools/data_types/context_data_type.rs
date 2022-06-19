use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct ContextDataType {
    pub encoded_data: Option<String>,
    #[validate]
    #[validate(required)]
    pub http_headers: Option<Vec<super::HttpHeader>>,
    #[validate(required)]
    #[validate(length(min = 1))]
    pub ip_address: Option<String>,
    #[validate(required)]
    #[validate(length(min = 1))]
    pub server_name: Option<String>,
    #[validate(required)]
    #[validate(length(min = 1))]
    pub server_path: Option<String>,
}
