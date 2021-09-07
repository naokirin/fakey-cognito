use crate::common;
use crate::http;
use crate::user_pools;
use serde::{Deserialize, Serialize};
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

impl user_pools::IntoResponse for AdminAddUserToGroupRequest {
    fn into_response(&self) -> user_pools::Response {
        if common::is_blank(&self.group_name)
            || common::is_blank(&self.user_name)
            || common::is_blank(&self.user_pool_id)
            || common::is_blank(&self.version)
        {
            let error = user_pools::ResponseError::<AdminAddUserToGroupError>::CommonError(
                CommonError::InvalidParameterValue,
            );
            return user_pools::error_response(error);
        }

        warp::http::Response::builder()
            .status(http::status_code(200))
            .body(user_pools::response::empty_body())
            .unwrap()
    }
}
