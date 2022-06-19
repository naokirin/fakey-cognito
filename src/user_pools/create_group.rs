use crate::common::{ARN_REGEX, USER_POOL_ID_REGEX};
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use validator::Validate;

pub const CREATE_GROUP_NAME: &str = "CreateGroup";
pub const CREATE_GROUP_ACTION_NAME: &str = "AWSCognitoIdentityProviderService.CreateGroup";

super::gen_response_err!(
    CreateGroupError,
    GroupExistsException
    | InvalidParameterException
    | LimitExceededException
    | NotAuthorizedException
    | ResourceNotFoundException
    | TooManyRequestsException => http::status_code(400),
    InternalErrorException => http::status_code(500)
);

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct CreateGroupRequest {
    #[validate(length(min = 1, max = 2048))]
    pub description: Option<String>,
    #[validate(required)]
    #[validate(length(min = 1, max = 128))]
    pub group_name: Option<String>,
    #[validate(range(min = 0, max = 2147483647))]
    pub precedence: Option<i64>,
    #[validate(length(min = 20, max = 2048))]
    #[validate(regex = "ARN_REGEX")]
    pub role_arn: Option<String>,
    #[validate(required)]
    #[validate(length(min = 1, max = 55))]
    #[validate(regex = "USER_POOL_ID_REGEX")]
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for CreateGroupRequest {
    fn to_action_name() -> &'static str {
        CREATE_GROUP_NAME
    }
}

impl super::ToResponse for CreateGroupRequest {
    type E = CreateGroupError;
    fn to_response(&self) -> super::Response {
        super::to_json_response(self, CREATE_GROUP_NAME)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = CreateGroupRequest {
            group_name: Some("group_name".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn failure_to_valid_request() {
        let request = CreateGroupRequest {
            group_name: Some("group_name".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = CreateGroupError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = CreateGroupError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
