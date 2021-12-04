use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

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

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct CreateGroupRequest {
    pub description: Option<String>,
    pub group_name: Option<String>,
    pub precedence: Option<i64>,
    pub role_arn: Option<String>,
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
        super::to_json_response(self, CREATE_GROUP_NAME, valid_request)
    }
}

/// Validates request.
fn valid_request(request: &CreateGroupRequest) -> bool {
    !common::is_blank(&request.group_name) && !common::is_blank(&request.user_pool_id)
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
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = CreateGroupRequest {
            group_name: Some("group_name".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(!valid_request(&request));
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
