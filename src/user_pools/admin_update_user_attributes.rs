use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const ADMIN_UPDATE_USER_ATTRIBUTES_NAME: &str = "AdminUpdateUserAttributes";
pub const ADMIN_UPDATE_USER_ATTRIBUTES_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminUpdateUserAttributes";

super::gen_response_err!(
    AdminUpdateUserAttributesError,
    AliasExistsException
    | InvalidEmailRoleAccessPolicyException
    | InvalidParameterException
    | InvalidLambdaResponseException
    | InvalidSmsRoleAccessPolicyException
    | InvalidSmsRoleTrustRelationshipException
    | NotAuthorizedException
    | ResourceNotFoundException
    | TooManyRequestsException
    | UnexpectedLambdaException
    | UserLambdaValidationException
    | UserNotFoundException => http::status_code(400),
    InternalErrorException => http::status_code(500)
);

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AdminUpdateUserAttributesRequest {
    pub client_metadata: Option<std::collections::HashMap<String, String>>,
    pub user_attributes: Option<Vec<super::data_types::AttributeType>>,
    pub username: Option<String>,
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for AdminUpdateUserAttributesRequest {
    fn to_action_name() -> &'static str {
        ADMIN_UPDATE_USER_ATTRIBUTES_NAME
    }
}

impl super::ToResponse for AdminUpdateUserAttributesRequest {
    type E = AdminUpdateUserAttributesError;
    fn to_response(&self) -> super::Response {
        super::to_empty_response(self, valid_request)
    }
}

/// Validates request.
fn valid_request(request: &AdminUpdateUserAttributesRequest) -> bool {
    request.user_attributes.is_some()
        && !common::is_blank(&request.username)
        && !common::is_blank(&request.user_pool_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = AdminUpdateUserAttributesRequest {
            user_attributes: Some(Default::default()),
            username: Some("username".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminUpdateUserAttributesRequest {
            user_attributes: Some(Default::default()),
            username: Some("username".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(!valid_request(&request));
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = AdminUpdateUserAttributesError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AdminUpdateUserAttributesError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
