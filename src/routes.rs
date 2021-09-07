use crate::common;
use crate::user_pools;
use bytes::Bytes;
use std::collections::HashMap;
use warp::Filter;

const AWS_ACTION_TARGET_HEADER: &str = "X-Amz-Target";
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
        serde_json::Value::Object(map) => match map[AWS_ACTION_TAREGET_KEY].clone() {
            serde_json::Value::String(s) => Some(s),
            _ => None,
        },
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
        "AdminAddUserToGroup" => {
            user_pools::response::<user_pools::AdminAddUserToGroupRequest>(body)
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
        .map(|bytes: Bytes, target, queries| post_routes(target, &bytes, queries).unwrap())
        .with(warp::log("info"))
}
