use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct HttpHeader {
    pub header_name: Option<String>,
    pub header_value: Option<String>,
}
