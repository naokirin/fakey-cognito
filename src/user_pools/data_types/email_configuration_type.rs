use crate::common::{ARN_REGEX, EMAIL_REGEX};
use crate::validator::includes;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

static CONFIGURATION_SET_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap());

fn includes_email_sending_account(value: &str) -> Result<(), ValidationError> {
    includes(value, vec!["COGNITO_DEFAULT", "DEVELOPER"])
}

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct EmailConfigurationType {
    #[validate(length(min = 1, max = 64))]
    #[validate(regex = "CONFIGURATION_SET_REGEX")]
    configuration_set: Option<String>,
    #[validate(custom(function = "includes_email_sending_account"))]
    email_sending_account: Option<String>,
    from: Option<String>,
    #[validate(regex = "EMAIL_REGEX")]
    reply_to_email_address: Option<String>,
    #[validate(length(min = 20, max = 2048))]
    #[validate(regex = "ARN_REGEX")]
    source_arn: Option<String>,
}
