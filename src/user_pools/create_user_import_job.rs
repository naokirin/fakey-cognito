use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const CREATE_USER_IMPORT_JOB_NAME: &str = "CreateUserImportJob";
pub const CREATE_USER_IMPORT_JOB_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.CreateUserImportJob";

/// CreateUserImportJob response errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_CreateUserImportJob.html#API_CreateUserImportJob_Errors
#[allow(clippy::enum_variant_names)]
#[derive(Display, EnumString)]
pub enum CreateUserImportJobError {
    InternalErrorException,
    InvalidParameterException,
    LimitExceededException,
    NotAuthorizedException,
    PreconditionNotMetException,
    ResourceNotFoundException,
    TooManyRequestsException,
}

impl super::ToStatusCode for CreateUserImportJobError {
    fn to_status_code(&self) -> hyper::StatusCode {
        match self {
            CreateUserImportJobError::InvalidParameterException
            | CreateUserImportJobError::LimitExceededException
            | CreateUserImportJobError::NotAuthorizedException
            | CreateUserImportJobError::PreconditionNotMetException
            | CreateUserImportJobError::ResourceNotFoundException
            | CreateUserImportJobError::TooManyRequestsException => http::status_code(400),
            CreateUserImportJobError::InternalErrorException => http::status_code(500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct CreateUserImportJobRequest {
    pub cloud_watch_logs_role_arn: Option<String>,
    pub job_name: Option<String>,
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
        super::to_json_response(self, CREATE_USER_IMPORT_JOB_NAME, valid_request)
    }
}

/// Validates request.
fn valid_request(request: &CreateUserImportJobRequest) -> bool {
    !common::is_blank(&request.cloud_watch_logs_role_arn)
        && !common::is_blank(&request.job_name)
        && !common::is_blank(&request.user_pool_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = CreateUserImportJobRequest {
            cloud_watch_logs_role_arn: Some("cloud_watch_logs_role_arn".to_string()),
            job_name: Some("job_name".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = CreateUserImportJobRequest {
            cloud_watch_logs_role_arn: Some("cloud_watch_logs_role_arn".to_string()),
            job_name: Some("job_name".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(!valid_request(&request));
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
