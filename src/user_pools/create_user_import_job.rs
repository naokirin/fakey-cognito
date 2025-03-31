use crate::common::{ARN_REGEX, USER_POOL_ID_REGEX};
use crate::http;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use validator::Validate;

pub const CREATE_USER_IMPORT_JOB_NAME: &str = "CreateUserImportJob";
pub const CREATE_USER_IMPORT_JOB_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.CreateUserImportJob";
static JOB_NAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[\w\s+=.@-]+").unwrap());

super::gen_response_err!(
    CreateUserImportJobError,
    InvalidParameterException
    | LimitExceededException
    | NotAuthorizedException
    | PreconditionNotMetException
    | ResourceNotFoundException
    | TooManyRequestsException => http::status_code(400),
    InternalErrorException => http::status_code(500)
);

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct CreateUserImportJobRequest {
    #[validate(required)]
    #[validate(length(min = 20, max = 2048))]
    #[validate(regex(path = *ARN_REGEX))]
    pub cloud_watch_logs_role_arn: Option<String>,
    #[validate(required)]
    #[validate(length(min = 1, max = 128))]
    #[validate(regex(path = *JOB_NAME_REGEX))]
    pub job_name: Option<String>,
    #[validate(required)]
    #[validate(length(min = 1, max = 55))]
    #[validate(regex(path = *USER_POOL_ID_REGEX))]
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for CreateUserImportJobRequest {
    fn to_action_name() -> &'static str {
        CREATE_USER_IMPORT_JOB_NAME
    }
}

impl super::ToResponse for CreateUserImportJobRequest {
    type E = CreateUserImportJobError;
    fn to_response(&self) -> super::Response {
        super::to_json_response(self, CREATE_USER_IMPORT_JOB_NAME)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = CreateUserImportJobRequest {
            cloud_watch_logs_role_arn: Some(
                "arn:aws:iam::123456789012:user/Development/product_1234".to_string(),
            ),
            job_name: Some("job-name".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn failure_to_valid_request() {
        let request = CreateUserImportJobRequest {
            cloud_watch_logs_role_arn: Some(
                "arn:aws:iam::123456789012:user/Development/product_1234".to_string(),
            ),
            job_name: Some("job_name".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = CreateUserImportJobError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = CreateUserImportJobError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
