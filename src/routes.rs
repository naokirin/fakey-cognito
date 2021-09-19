use crate::common;
use crate::user_pools;
use crate::user_pools::UserPoolsResponseResult;
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
            map.get(AWS_ACTION_TAREGET_KEY).and_then(move |s| match s {
                serde_json::Value::String(s) => Some(s.clone()),
                _ => None,
            })
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

fn post_action_routes(action: &str, body: &Bytes) -> UserPoolsResponseResult {
    match action {
        user_pools::ADMIN_ADD_USER_TO_GROUP_ACTION_NAME => {
            user_pools::response::<user_pools::AdminAddUserToGroupRequest>(body)
        }
        user_pools::ADMIN_CONFIRM_SIGN_UP_ACTION_NAME => {
            user_pools::response::<user_pools::AdminConfirmSignUpRequest>(body)
        }
        user_pools::ADMIN_CREATE_USER_ACTION_NAME => {
            user_pools::response::<user_pools::AdminCreateUserRequest>(body)
        }
        user_pools::ADMIN_DELETE_USER_ACTION_NAME => {
            user_pools::response::<user_pools::AdminDeleteUserRequest>(body)
        }
        user_pools::ADMIN_DELETE_USER_ATTRIBUTES_ACTION_NAME => {
            user_pools::response::<user_pools::AdminDeleteUserAttributesRequest>(body)
        }
        user_pools::ADMIN_DISABLE_PROVIDER_FOR_USER_ACTION_NAME => {
            user_pools::response::<user_pools::AdminDisableProviderForUserRequest>(body)
        }
        user_pools::ADMIN_DISABLE_USER_ACTION_NAME => {
            user_pools::response::<user_pools::AdminDisableUserRequest>(body)
        }
        user_pools::ADMIN_ENABLE_USER_ACTION_NAME => {
            user_pools::response::<user_pools::AdminEnableUserRequest>(body)
        }
        user_pools::ADMIN_FORGET_DEVICE_ACTION_NAME => {
            user_pools::response::<user_pools::AdminForgetDeviceRequest>(body)
        }
        user_pools::ADMIN_GET_DEVICE_ACTION_NAME => {
            user_pools::response::<user_pools::AdminGetDeviceRequest>(body)
        }
        user_pools::ADMIN_GET_USER_ACTION_NAME => {
            user_pools::response::<user_pools::AdminGetUserRequest>(body)
        }
        user_pools::ADMIN_INITIATE_AUTH_ACTION_NAME => {
            user_pools::response::<user_pools::AdminInitiateAuthRequest>(body)
        }
        user_pools::ADMIN_LINK_PROVIDER_FOR_USER_ACTION_NAME => {
            user_pools::response::<user_pools::AdminLinkProviderForUserRequest>(body)
        }
        user_pools::ADMIN_LIST_DEVICES_ACTION_NAME => {
            user_pools::response::<user_pools::AdminListDevicesRequest>(body)
        }
        user_pools::ADMIN_LIST_GROUPS_FOR_USER_ACTION_NAME => {
            user_pools::response::<user_pools::AdminListGroupsForUserRequest>(body)
        }
        user_pools::ADMIN_LIST_USER_AUTH_EVENTS_ACTION_NAME => {
            user_pools::response::<user_pools::AdminListUserAuthEventsRequest>(body)
        }
        user_pools::ADMIN_REMOVE_USER_FROM_GROUP_ACTION_NAME => {
            user_pools::response::<user_pools::AdminRemoveUserFromGroupRequest>(body)
        }
        user_pools::ADMIN_RESET_USER_PASSWORD_ACTION_NAME => {
            user_pools::response::<user_pools::AdminResetUserPasswordRequest>(body)
        }
        user_pools::ADMIN_RESPOND_TO_AUTH_CHALLENGE_ACTION_NAME => {
            user_pools::response::<user_pools::AdminRespondToAuthChallengeRequest>(body)
        }
        user_pools::ADMIN_SET_USER_MFA_PREFERENCE_ACTION_NAME => {
            user_pools::response::<user_pools::AdminSetUserMFAPreferenceRequest>(body)
        }
        user_pools::ADMIN_SET_USER_PASSWORD_ACTION_NAME => {
            user_pools::response::<user_pools::AdminSetUserPasswordRequest>(body)
        }
        user_pools::ADMIN_SET_USER_SETTINGS_ACTION_NAME => {
            user_pools::response::<user_pools::AdminSetUserSettingsRequest>(body)
        }
        user_pools::ADMIN_UPDATE_AUTH_EVENT_FEEDBACK_ACTION_NAME => {
            user_pools::response::<user_pools::AdminUpdateAuthEventFeedbackRequest>(body)
        }

        _ => Ok(user_pools::error_response(
            user_pools::CommonError::InvalidAction,
        )),
    }
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

    post_action_routes(target.as_ref(), &body)
}

pub fn user_pools_routes(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end()
        .and(warp::post())
        .and(warp::body::bytes())
        .and(warp::header::optional::<String>(AWS_ACTION_TARGET_HEADER))
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::header::headers_cloned())
        .map(move |bytes: Bytes, target, queries, headers| {
            log::debug!("request headers: {:?}", &headers);
            log::debug!("request body: {:?}", &bytes);
            let result = post_routes(target, &bytes, queries).unwrap();
            log::debug!("response status: {:?}", &result.status());
            log::debug!("response headers: {:?}", &result.headers());
            result
        })
        .with(warp::log("info"))
}
