use crate::common::{NAME_REGEX, USER_POOL_ID_REGEX};
use crate::{
    http,
    validator::{includes, includes_in_array},
};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use validator::{Validate, ValidationError};

pub const ADMIN_CREATE_USER_NAME: &str = "AdminCreateUser";
pub const ADMIN_CREATE_USER_ACTION_NAME: &str = "AWSCognitoIdentityProviderService.AdminCreateUser";

static TEMPORARY_PASSWORD_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[\S]+").unwrap());

fn validate_desired_delivery_mediums(value: &[String]) -> Result<(), ValidationError> {
    includes_in_array(value, vec!["SMS", "EMAIL"])
}

fn validate_message_action(value: &str) -> Result<(), ValidationError> {
    includes(value, vec!["RESEND", "SUPPRESS"])
}

super::gen_response_err!(
    AdminCreateUserError,
    CodeDeliveryFailureException
    | InvalidLambdaResponseException
    | InvalidParameterException
    | InvalidPasswordException
    | InvalidSmsRoleAccessPolicyException
    | InvalidSmsRoleTrustRelationshipException
    | NotAuthorizedException
    | PreconditionNotMetException
    | ResourceNotFoundException
    | TooManyRequestsException
    | UnexpectedLambdaException
    | UnsupportedUserStateException
    | UserLambdaValidationException
    | UsernameExistsException
    | UserNotFoundException => http::status_code(400),
    InternalErrorException => http::status_code(500)
);

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct AdminCreateUserRequest {
    pub client_metadata: Option<std::collections::HashMap<String, String>>,
    #[validate(custom(function = "validate_desired_delivery_mediums"))]
    pub desired_delivery_mediums: Option<Vec<String>>,
    pub force_alias_creation: Option<bool>,
    #[validate(custom(function = "validate_message_action"))]
    pub message_action: Option<String>,
    #[validate(length(max = 256))]
    #[validate(regex = "TEMPORARY_PASSWORD_REGEX")]
    pub temporary_password: Option<String>,
    pub user_attributes: Option<Vec<std::collections::HashMap<String, String>>>,
    #[validate(required)]
    #[validate(length(min = 1, max = 128))]
    #[validate(regex = "NAME_REGEX")]
    pub username: Option<String>,
    #[validate(required)]
    #[validate(length(min = 1, max = 55))]
    #[validate(regex = "USER_POOL_ID_REGEX")]
    pub user_pool_id: Option<String>,
    pub validation_data: Option<Vec<std::collections::HashMap<String, String>>>,
}

impl super::ToActionName for AdminCreateUserRequest {
    fn to_action_name() -> &'static str {
        ADMIN_CREATE_USER_NAME
    }
}

impl super::ToResponse for AdminCreateUserRequest {
    type E = AdminCreateUserError;
    fn to_response(&self) -> super::Response {
        super::to_json_response(self, ADMIN_CREATE_USER_NAME)
    }
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
        assert_eq!(request.validate(), Ok(()));
        assert!(request.validate().is_ok());
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminCreateUserRequest {
            username: Some("username".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(request.validate().is_err());
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
