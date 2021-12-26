use crate::common::APPLICATION_ID_REGEX;
use crate::common::ARN_REGEX;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct AnalyticsConfigurationType {
    #[validate(length(min = 20, max = 2048))]
    #[validate(regex = "ARN_REGEX")]
    application_arn: Option<String>,
    #[validate(regex = "APPLICATION_ID_REGEX")]
    application_id: Option<String>,
    external_id: Option<String>,
    #[validate(length(min = 20, max = 2048))]
    #[validate(regex = "ARN_REGEX")]
    role_arn: Option<String>,
    user_data_shared: Option<bool>,
}
