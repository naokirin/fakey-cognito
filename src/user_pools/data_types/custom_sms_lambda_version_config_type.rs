use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct CustomSMSLambdaVersionConfigType {
    lambda_arn: Option<String>,
    lambda_version: Option<String>,
}
