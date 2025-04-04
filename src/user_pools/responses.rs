use crate::http;
use crate::templates;
use bytes::Bytes;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use validator::Validate;

const AWS_CONTENT_TYPE_HEADER_VALUE: &str = "application/x-amz-json-1.1";
const AWS_ERROR_TYPE_HEADER: &str = "x-amzn-ErrorType";
const AWS_ERROR_MESSAGE_HEADER: &str = "x-amzn-ErrorMessage";

pub type Response = warp::http::Response<warp::hyper::Body>;
pub type UserPoolsResponseResult = std::result::Result<Response, Infallible>;

pub trait ToActionName {
    fn to_action_name() -> &'static str;
}

pub trait ToResponse {
    type E: std::str::FromStr + std::fmt::Display + ToStatusCode;
    fn to_response(&self) -> Response;
}

pub trait ToStatusCode {
    fn to_status_code(&self) -> StatusCode;
}

/// Returns empty body.
pub fn empty_body() -> warp::hyper::Body {
    warp::hyper::Body::empty()
}

/// Returns body with json serialized value.
pub fn json_body(value: &str) -> warp::hyper::Body {
    warp::hyper::Body::from(value.to_string())
}

pub fn response<'a, T>(body: &'a Bytes) -> UserPoolsResponseResult
where
    T: Serialize + for<'de> Deserialize<'de> + ToResponse,
{
    match serde_json::from_slice::<T>(body) {
        Ok(req) => Ok(req.to_response()),
        Err(_) => Ok(error_response(
            super::CommonError::InternalFailure,
            Some("Cannot deserialize json body."),
        )),
    }
}

/// Generates error response for aws.
pub fn error_response<T>(error: T, message: Option<&str>) -> Response
where
    T: std::fmt::Display + ToStatusCode,
{
    warp::http::Response::builder()
        .status(warp::http::StatusCode::from_u16(error.to_status_code().as_u16()).unwrap())
        .header(AWS_ERROR_TYPE_HEADER, format!("{}", error))
        .header(AWS_ERROR_MESSAGE_HEADER, "DUMMY ERROR MESSAGE")
        .header("Content-Type", AWS_CONTENT_TYPE_HEADER_VALUE)
        .body(json_body(&format!(
            "{{\"__type\": \"{}\",\"message\":\"{}\"}}",
            error,
            message.unwrap_or("DUMMY ERROR MESSAGE")
        )))
        .unwrap()
}

pub fn config_response<R>() -> Option<Response>
where
    R: super::ToActionName + ToResponse,
{
    use std::str::FromStr;
    match super::get_config(R::to_action_name(), super::CONFIG_ERROR_TYPE) {
        Some(name) => super::ResponseError::<R::E>::from_str(name.as_str()).map_or(None, |e| {
            Some(super::error_response(
                e,
                Some("force response from config."),
            ))
        }),
        _ => None,
    }
}

pub fn to_json_response<R>(request: &R, template_name: &str) -> Response
where
    R: super::ToActionName + ToResponse + serde::Serialize + Validate,
{
    if let Some(response) = super::config_response::<R>() {
        return response;
    };
    if request.validate().is_err() {
        let error =
            super::ResponseError::<R::E>::CommonError(super::CommonError::InvalidParameterValue);
        return super::error_response(error, Some("Parameters validation error."));
    }

    let hook_result = crate::hooks::call_request_hook(
        template_name,
        &request,
        crate::opts::get_opt_hooks().map(|o| o.as_ref()),
    );
    let opt_json = templates::render_template(
        template_name,
        &request,
        hook_result.unwrap_or_else(|e| {
            log::warn!("hook script error: {}", e);
            "{}".to_string()
        }),
    );
    match opt_json {
        Some(json) => warp::http::Response::builder()
            .status(warp::http::StatusCode::from_u16(http::status_code(200).as_u16()).unwrap())
            .body(super::responses::json_body(&json))
            .unwrap(),
        _ => super::error_response(
            super::CommonError::InternalFailure,
            Some("Error during rendering response template."),
        ),
    }
}

pub fn to_empty_response<R>(request: &R) -> Response
where
    R: super::ToActionName + ToResponse + serde::Serialize + Validate,
{
    if let Some(response) = super::config_response::<R>() {
        return response;
    };
    if request.validate().is_err() {
        let error =
            super::ResponseError::<R::E>::CommonError(super::CommonError::InvalidParameterValue);
        return super::error_response(error, Some("Parameters validation error."));
    }

    warp::http::Response::builder()
        .status(warp::http::StatusCode::from_u16(http::status_code(200).as_u16()).unwrap())
        .body(super::responses::empty_body())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http;
    use crate::user_pools::{CommonError, ResponseError};
    use pretty_assertions::assert_eq;
    use strum_macros::{Display, EnumString};

    #[derive(Display, EnumString)]
    enum TestError {}
    impl ToStatusCode for TestError {
        fn to_status_code(&self) -> StatusCode {
            http::status_code(400)
        }
    }

    #[test]
    fn error_response_has_status_code() {
        let response = error_response(CommonError::AccessDeniedException, None);
        assert_eq!(400, response.status().as_u16());
    }

    #[test]
    fn error_response_has_error_header() {
        let response = error_response::<ResponseError<TestError>>(
            ResponseError::CommonError(CommonError::AccessDeniedException),
            None,
        );
        assert!(response.headers().contains_key(AWS_ERROR_TYPE_HEADER));
        assert_eq!(
            "AccessDeniedException",
            response.headers()[AWS_ERROR_TYPE_HEADER]
        );
    }
}
