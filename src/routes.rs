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
) -> anyhow::Result<String> {
    let json: serde_json::Value = serde_json::from_slice(body)?;
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
        return Err(MissingActionError {}.into());
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

// creating match expression helper macro for post action routing
macro_rules! post_action_routes_match {
    ($action:expr,$body:expr,$($k:pat => $v:ty),* $(,)?) => {{
        match $action {
            $($k => { user_pools::response::<$v>($body) })*
            _ => Ok(user_pools::error_response(
                user_pools::CommonError::InvalidAction,
                Some(&format!("Unknown action name '{}'.", $action)),
            ))
        }
    }}
}

fn post_action_routes(action: &str, body: &Bytes) -> UserPoolsResponseResult {
    post_action_routes_match!(action, body,
        user_pools::ADD_CUSTOM_ATTRIBUTES_ACTION_NAME => user_pools::AddCustomAttributesRequest,
        user_pools::ADMIN_ADD_USER_TO_GROUP_ACTION_NAME => user_pools::AdminAddUserToGroupRequest,
        user_pools::ADMIN_CONFIRM_SIGN_UP_ACTION_NAME => user_pools::AdminConfirmSignUpRequest,
        user_pools::ADMIN_CREATE_USER_ACTION_NAME => user_pools::AdminCreateUserRequest,
        user_pools::ADMIN_DELETE_USER_ATTRIBUTES_ACTION_NAME => user_pools::AdminDeleteUserAttributesRequest,
        user_pools::ADMIN_DELETE_USER_ACTION_NAME => user_pools::AdminDeleteUserRequest,
        user_pools::ADMIN_DISABLE_PROVIDER_FOR_USER_ACTION_NAME => user_pools::AdminDisableProviderForUserRequest,
        user_pools::ADMIN_DISABLE_USER_ACTION_NAME => user_pools::AdminDisableUserRequest,
        user_pools::ADMIN_ENABLE_USER_ACTION_NAME => user_pools::AdminEnableUserRequest,
        user_pools::ADMIN_FORGET_DEVICE_ACTION_NAME => user_pools::AdminForgetDeviceRequest,
        user_pools::ADMIN_GET_DEVICE_ACTION_NAME => user_pools::AdminGetDeviceRequest,
        user_pools::ADMIN_GET_USER_ACTION_NAME => user_pools::AdminGetUserRequest,
        user_pools::ADMIN_INITIATE_AUTH_ACTION_NAME => user_pools::AdminInitiateAuthRequest,
        user_pools::ADMIN_LINK_PROVIDER_FOR_USER_ACTION_NAME => user_pools::AdminLinkProviderForUserRequest,
        user_pools::ADMIN_LIST_DEVICES_ACTION_NAME => user_pools::AdminListDevicesRequest,
        user_pools::ADMIN_LIST_GROUPS_FOR_USER_ACTION_NAME => user_pools::AdminListGroupsForUserRequest,
        user_pools::ADMIN_LIST_USER_AUTH_EVENTS_ACTION_NAME => user_pools::AdminListUserAuthEventsRequest,
        user_pools::ADMIN_REMOVE_USER_FROM_GROUP_ACTION_NAME => user_pools::AdminRemoveUserFromGroupRequest,
        user_pools::ADMIN_RESET_USER_PASSWORD_ACTION_NAME => user_pools::AdminResetUserPasswordRequest,
        user_pools::ADMIN_RESPOND_TO_AUTH_CHALLENGE_ACTION_NAME => user_pools::AdminRespondToAuthChallengeRequest,
        user_pools::ADMIN_SET_USER_MFA_PREFERENCE_ACTION_NAME => user_pools::AdminSetUserMFAPreferenceRequest,
        user_pools::ADMIN_SET_USER_PASSWORD_ACTION_NAME => user_pools::AdminSetUserPasswordRequest,
        user_pools::ADMIN_SET_USER_SETTINGS_ACTION_NAME => user_pools::AdminSetUserSettingsRequest,
        user_pools::ADMIN_UPDATE_AUTH_EVENT_FEEDBACK_ACTION_NAME => user_pools::AdminUpdateAuthEventFeedbackRequest,
        user_pools::ADMIN_UPDATE_DEVICE_STATUS_ACTION_NAME => user_pools::AdminUpdateDeviceStatusRequest,
        user_pools::ADMIN_UPDATE_USER_ATTRIBUTES_ACTION_NAME => user_pools::AdminUpdateUserAttributesRequest,
        user_pools::ADMIN_USER_GLOBAL_SIGN_OUT_ACTION_NAME => user_pools::AdminUserGlobalSignOutRequest,
        user_pools::ASSOCIATE_SOFTWARE_TOKEN_ACTION_NAME => user_pools::AssociateSoftwareTokenRequest,
        user_pools::CHANGE_PASSWORD_ACTION_NAME => user_pools::ChangePasswordRequest,
        user_pools::CONFIRM_DEVICE_ACTION_NAME => user_pools::ConfirmDeviceRequest,
        user_pools::CONFIRM_FORGOT_PASSWORD_ACTION_NAME => user_pools::ConfirmForgotPasswordRequest,
        user_pools::CONFIRM_SIGN_UP_ACTION_NAME => user_pools::ConfirmSignUpRequest,
        user_pools::CREATE_GROUP_ACTION_NAME => user_pools::CreateGroupRequest,
        user_pools::CREATE_IDENTITY_PROVIDER_ACTION_NAME => user_pools::CreateIdentityProviderRequest,
        user_pools::CREATE_RESOURCE_SERVER_ACTION_NAME => user_pools::CreateResourceServerRequest,
        user_pools::CREATE_USER_IMPORT_JOB_ACTION_NAME => user_pools::CreateUserImportJobRequest,
    )
}

/// POST routes.
fn post_routes(
    action_header: Option<String>,
    body: &Bytes,
    queries: HashMap<String, String>,
) -> user_pools::UserPoolsResponseResult {
    let target = match take_action(action_header, body, queries) {
        Ok(action) => action,
        Err(e) => {
            if e.downcast_ref::<MissingActionError>().is_some() {
                return Ok(user_pools::error_response(
                    user_pools::CommonError::MissingAction,
                    Some("No specified action."),
                ));
            } else {
                return Ok(user_pools::error_response(
                    user_pools::CommonError::InternalFailure,
                    Some(&format!("Error: {:?}", e)),
                ));
            }
        }
    };

    post_action_routes(target.as_ref(), body)
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
            log::debug!("incoming headers: {:?}", &headers);
            log::debug!("incoming body: {:?}", &bytes);
            let result = post_routes(target, &bytes, queries).unwrap();
            log::debug!("response status: {:?}", &result.status());
            log::debug!("response headers: {:?}", &result.headers());
            log::debug!("response body: {:?}", &result.body());
            result
        })
        .with(warp::log("info"))
}
