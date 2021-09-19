use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct HttpHeader {
    pub header_name: Option<String>,
    pub header_value: Option<String>,
}
