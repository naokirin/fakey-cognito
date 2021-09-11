use crate::common;
use crate::http;
use crate::templates;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const ADMIN_CREATE_USER_NAME: &str = "AdminCreateUser";
pub const ADMIN_CREATE_USER_ACTION_NAME: &str = "AWSCognitoIdentityProviderService.AdminCreateUser";

/// AdminCreateUser response errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminCreateUser.html#API_AdminCreateUser_Errors
#[derive(Display, EnumString)]
pub enum AdminCreateUserError {
    CodeDeliveryFailureException,
    InternalErrorException,
    InvalidLambdaResponseException,
    InvalidParameterException,
    InvalidPasswordException,
    InvalidSmsRoleAccessPolicyException,
    InvalidSmsRoleTrustRelationshipException,
    NotAuthorizedException,
    PreconditionNotMetException,
    ResourceNotFoundException,
    TooManyRequestsException,
    UnexpectedLambdaException,
    UnsupportedUserStateException,
    UserLambdaValidationException,
    UsernameExistsException,
    UserNotFoundException,
}

impl super::ToStatusCode for AdminCreateUserError {
    fn to_status_code(&self) -> hyper::StatusCode {
        match self {
            AdminCreateUserError::CodeDeliveryFailureException
            | AdminCreateUserError::InvalidLambdaResponseException
            | AdminCreateUserError::InvalidParameterException
            | AdminCreateUserError::InvalidPasswordException
            | AdminCreateUserError::InvalidSmsRoleAccessPolicyException
            | AdminCreateUserError::InvalidSmsRoleTrustRelationshipException
            | AdminCreateUserError::NotAuthorizedException
            | AdminCreateUserError::PreconditionNotMetException
            | AdminCreateUserError::ResourceNotFoundException
            | AdminCreateUserError::TooManyRequestsException
            | AdminCreateUserError::UnexpectedLambdaException
            | AdminCreateUserError::UnsupportedUserStateException
            | AdminCreateUserError::UserLambdaValidationException
            | AdminCreateUserError::UsernameExistsException
            | AdminCreateUserError::UserNotFoundException => http::status_code(400),
            _ => http::status_code(500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AdminCreateUserRequest {
    pub client_metadata: Option<std::collections::HashMap<String, String>>,
    pub desired_delivery_mediums: Option<Vec<String>>,
    pub force_alias_creation: Option<bool>,
    pub message_action: Option<String>,
    pub temporary_password: Option<String>,
    pub user_attributes: Option<Vec<std::collections::HashMap<String, String>>>,
    pub username: Option<String>,
    pub user_pool_id: Option<String>,
    pub validation_data: Option<Vec<std::collections::HashMap<String, String>>>,
}

impl super::ToActionName for AdminCreateUserRequest {
    fn to_action_name() -> &'static str {
        ADMIN_CREATE_USER_NAME
    }
}

impl super::ToResponse for AdminCreateUserRequest {
    fn to_response(&self) -> super::Response {
        if let Some(response) =
            super::config_response::<AdminCreateUserRequest, AdminCreateUserError>()
        {
            return response;
        };
        if !valid_request(&self) {
            let error = super::ResponseError::<AdminCreateUserError>::CommonError(
                super::CommonError::InvalidParameterValue,
            );
            return super::error_response(error);
        }

        let opt_json = templates::render_template(ADMIN_CREATE_USER_NAME, &self);
        match opt_json {
            Some(json) => warp::http::Response::builder()
                .status(http::status_code(200))
                .body(super::responses::json_body(&json))
                .unwrap(),
            _ => super::error_response(super::CommonError::InternalFailure),
        }
    }
}

/// Validates request.
fn valid_request(request: &AdminCreateUserRequest) -> bool {
    !common::is_blank(&request.username)
        && !common::is_blank(&request.user_pool_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = AdminCreateUserRequest {
            username: Some("username".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminCreateUserRequest {
            username: Some("username".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(!valid_request(&request));
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = AdminCreateUserError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AdminCreateUserError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
