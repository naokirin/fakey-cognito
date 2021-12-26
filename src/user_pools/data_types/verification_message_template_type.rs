use crate::validator::includes;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

static EMAIL_MESSAGE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[\p{L}\p{M}\p{S}\p{N}\p{P}\s*]*\{####\}[\p{L}\p{M}\p{S}\p{N}\p{P}\s*]*").unwrap()
});

static EMAIL_MESSAGE_BY_LINK_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[\p{L}\p{M}\p{S}\p{N}\p{P}\s*]*\{##[\p{L}\p{M}\p{S}\p{N}\p{P}\s*]*##\}[\p{L}\p{M}\p{S}\p{N}\p{P}\s*]*").unwrap()
});

static EMAIL_SUBJECT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[\p{L}\p{M}\p{S}\p{N}\p{P}\s]+").unwrap());

static SMS_MESSAGE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r".*\{####\}.*").unwrap());

fn includes_valid_email_option(value: &str) -> Result<(), ValidationError> {
    includes(value, vec!["CONFIRM_WITH_LINK", "CONFIRM_WITH_CODE"])
}

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct VerificationMessageTemplateType {
    #[validate(custom(function = "includes_valid_email_option"))]
    default_email_option: Option<String>,
    #[validate(length(min = 6, max = 20000))]
    #[validate(regex = "EMAIL_MESSAGE_REGEX")]
    email_message: Option<String>,
    #[validate(length(min = 6, max = 20000))]
    #[validate(regex = "EMAIL_MESSAGE_BY_LINK_REGEX")]
    email_message_by_link: Option<String>,
    #[validate(length(min = 1, max = 140))]
    #[validate(regex = "EMAIL_SUBJECT_REGEX")]
    email_subject: Option<String>,
    #[validate(length(min = 1, max = 140))]
    #[validate(regex = "EMAIL_SUBJECT_REGEX")]
    email_subject_by_link: Option<String>,
    #[validate(length(min = 6, max = 140))]
    #[validate(regex = "SMS_MESSAGE_REGEX")]
    sms_message: Option<String>,
}
