use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

static EMAIL_MESSAGE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[\p{L}\p{M}\p{S}\p{N}\p{P}\s*]*\{####\}[\p{L}\p{M}\p{S}\p{N}\p{P}\s*]*").unwrap()
});

static EMAIL_SUBJECT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[\p{L}\p{M}\p{S}\p{N}\p{P}\s]+").unwrap());

static SMS_MESSAGE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r".*\{####\}.*").unwrap());

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct MessageTemplateType {
    #[validate(length(min = 6, max = 20000))]
    #[validate(regex = "EMAIL_MESSAGE_REGEX")]
    email_message: Option<String>,
    #[validate(length(min = 6, max = 140))]
    #[validate(regex = "EMAIL_SUBJECT_REGEX")]
    email_subject: Option<String>,
    #[validate(length(min = 6, max = 140))]
    #[validate(regex = "SMS_MESSAGE_REGEX")]
    sms_message: Option<String>,
}
