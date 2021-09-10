use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

use super::CommonError;
use super::ToStatusCode;

/// AdminAddUserToGroup response errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminAddUserToGroup.html#API_AdminAddUserToGroup_Errors
#[derive(Display, EnumString)]
pub enum AdminAddUserToGroupError {
    InternalErrorException,
    InvalidParameterException,
    NotAuthorizedException,
    ResourceNotFoundException,
    TooManyRequestsException,
    UserNotFoundException,
}

impl ToStatusCode for AdminAddUserToGroupError {
    fn to_status_code(&self) -> hyper::StatusCode {
        match self {
            AdminAddUserToGroupError::InvalidParameterException
            | AdminAddUserToGroupError::NotAuthorizedException
            | AdminAddUserToGroupError::ResourceNotFoundException
            | AdminAddUserToGroupError::TooManyRequestsException
            | AdminAddUserToGroupError::UserNotFoundException => http::status_code(400),
            _ => http::status_code(500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AdminAddUserToGroupRequest {
    pub group_name: Option<String>,
    pub user_name: Option<String>,
    pub user_pool_id: Option<String>,
    pub version: Option<String>,
}

impl super::IntoResponse for AdminAddUserToGroupRequest {
    fn into_response(&self) -> super::Response {
        if let Some(name) = config(&super::CONFIG_STATUS_NAME.to_string()) {
            let error = super::ResponseError::<AdminAddUserToGroupError>::from_str(name.as_str());
            if let Ok(e) = error {
                return super::error_response(e);
            }
        };

        if valid_request(&self) {
            let error = super::ResponseError::<AdminAddUserToGroupError>::CommonError(
                CommonError::InvalidParameterValue,
            );
            return super::error_response(error);
        }

        warp::http::Response::builder()
            .status(http::status_code(200))
            .body(super::responses::empty_body())
            .unwrap()
    }
}

/// Validates request.
fn valid_request(request: &AdminAddUserToGroupRequest) -> bool {
    !common::is_blank(&request.group_name)
        && !common::is_blank(&request.user_name)
        && !common::is_blank(&request.user_pool_id)
        && !common::is_blank(&request.version)
}

/// Get config.
fn config(name: &String) -> Option<String> {
    super::config()
        .as_ref()
        .map(|c| c.admin_add_user_to_group.as_ref())
        .unwrap_or(None)
        .unwrap_or(&std::collections::HashMap::new())
        .get(name)
        .map(|c| c.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = AdminAddUserToGroupRequest {
            group_name: Some("group_name".to_string()),
            user_name: Some("user_name".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            version: Some("version".to_string()),
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminAddUserToGroupRequest {
            group_name: Some("group_name".to_string()),
            user_name: Some("user_name".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            version: Some("".to_string()),
        };
        assert!(!valid_request(&request));
    }

    #[test]
    fn error_can_convert_to_status_code() {
        let error = AdminAddUserToGroupError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AdminAddUserToGroupError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
