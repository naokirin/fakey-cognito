use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct HttpHeader {
    header_name: Option<String>,
    header_value: Option<String>,
}
