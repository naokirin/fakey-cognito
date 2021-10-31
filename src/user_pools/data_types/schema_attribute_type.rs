use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct SchemaAttributeType {
    pub attribute_data_type: Option<String>,
    pub developer_only_attribute: Option<bool>,
    pub mutable: Option<bool>,
    pub name: Option<String>,
    pub number_attribute_constraints: Option<super::NumberAttributeConstraintsType>,
    pub required: Option<bool>,
    pub string_attribute_constraints: Option<super::StringAttributeConstraintsType>,
}
