use crate::common;
use crate::user_pools;
use bytes::Bytes;
use std::collections::HashMap;
use warp::Filter;

const AWS_ACTION_TARGET_HEADER: &str = "x-amz-target";
const AWS_ACTION_TAREGET_KEY: &str = "Action";

#[derive(thiserror::Error, Debug, Clone)]
struct MissingActionError;
impl std::fmt::Display for MissingActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "MissingActionError")
    }
}

/// Returns action.
fn take_action(
    action_header: Option<String>,
    body: &Bytes,
    queries: HashMap<String, String>,
) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let json: serde_json::Value = serde_json::from_slice(&body)?;
    let body_action = match json {
        serde_json::Value::Object(map) => {
            if map.contains_key(AWS_ACTION_TAREGET_KEY) {
                match map[AWS_ACTION_TAREGET_KEY].clone() {
                    serde_json::Value::String(s) => Some(s),
                    _ => None,
                }
            }
            else {
                None
            }
        }
        _ => None,
    };

    if common::is_blank(&body_action)
        && common::is_blank(&action_header)
        && !queries.contains_key(AWS_ACTION_TAREGET_KEY)
    {
        Err(MissingActionError {})?;
    }

    let action = if !common::is_blank(&body_action) {
        body_action.unwrap()
    } else if !common::is_blank(&action_header) {
        action_header.unwrap()
    } else {
        queries[AWS_ACTION_TAREGET_KEY].clone()
    };

    Ok(action)
}

/// POST routes.
fn post_routes(
    action_header: Option<String>,
    body: &Bytes,
    queries: HashMap<String, String>,
) -> user_pools::UserPoolsResponseResult {
    let target = match take_action(action_header, body, queries) {
        Ok(action) => action,
        Err(_) => {
            return Ok(user_pools::error_response(
                user_pools::CommonError::MissingAction,
            ))
        }
    };

    match target.as_str() {
        user_pools::ADMIN_ADD_USER_TO_GROUP_ACTION_NAME => {
            user_pools::response::<user_pools::AdminAddUserToGroupRequest>(body)
        }
        user_pools::ADMIN_CONFIRM_SIGN_UP_ACTION_NAME => {
            user_pools::response::<user_pools::AdminConfirmSignUpRequest>(body)
        }
        user_pools::ADMIN_CREATE_USER_ACTION_NAME => {
            user_pools::response::<user_pools::AdminCreateUserRequest>(body)
        }

        _ => Ok(user_pools::error_response(
            user_pools::CommonError::InvalidAction,
        )),
    }
}

pub fn user_pools_routes(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // FIXME: unwrap directed
    warp::path::end()
        .and(warp::post())
        .and(warp::body::bytes())
        .and(warp::header::optional::<String>(AWS_ACTION_TARGET_HEADER))
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::header::headers_cloned())
        .map(|bytes: Bytes, target, queries, headers| {
            log::debug!("request headers: {:?}", &headers);
            log::debug!("request body: {:?}", &bytes);
            let result = post_routes(target, &bytes, queries).unwrap();
            log::debug!("headers: {:?}", &result.headers());
            log::debug!("status: {:?}", &result.status());
            result
        })
        .with(warp::log("info"))
}
