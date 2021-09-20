use crate::http;
use crate::templates;
use bytes::Bytes;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

const AWS_CONTENT_TYPE_HEADER_VALUE: &str = "application/x-amz-json-1.1";
const AWS_ERROR_TYPE_HEADER: &str = "x-amzn-ErrorType";
const AWS_ERROR_MESSAGE_HEADER: &str = "x-amzn-ErrorMessage";

pub type Response = warp::http::Response<hyper::Body>;
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
pub fn empty_body() -> hyper::Body {
    hyper::Body::empty()
}

/// Returns body with json serialized value.
pub fn json_body(value: &str) -> hyper::Body {
    hyper::Body::from(value.to_string())
}

pub fn response<'a, T>(body: &'a Bytes) -> UserPoolsResponseResult
where
    T: Serialize + for<'de> Deserialize<'de> + ToResponse,
{
    match serde_json::from_slice::<T>(body) {
        Ok(req) => Ok(req.to_response()),
        Err(_) => Ok(error_response(super::CommonError::InternalFailure)),
    }
}

/// Generates error response for aws.
pub fn error_response<T>(error: T) -> Response
where
    T: std::fmt::Display + ToStatusCode,
{
    warp::http::Response::builder()
        .status(error.to_status_code())
        .header(AWS_ERROR_TYPE_HEADER, format!("{}", error))
        .header(AWS_ERROR_MESSAGE_HEADER, "DUMMY ERROR MESSAGE")
        .header("Content-Type", AWS_CONTENT_TYPE_HEADER_VALUE)
        .body(json_body(&format!(
            "{{\"__type\": \"{}\",\"message\":\"DUMMY ERROR MESSAGE\"}}",
            error
        )))
        .unwrap()
}

pub fn config_response<R>() -> Option<Response>
where
    R: super::ToActionName + ToResponse,
{
    use std::str::FromStr;
    match super::get_config(R::to_action_name(), &super::CONFIG_ERROR_TYPE.to_string()) {
        Some(name) => super::ResponseError::<R::E>::from_str(name.as_str())
            .map_or(None, |e| Some(super::error_response(e))),
        _ => None,
    }
}

pub fn to_json_response<R, F>(request: &R, template_name: &str, validation_callback: F) -> Response
where
    R: super::ToActionName + ToResponse + serde::Serialize,
    F: Fn(&R) -> bool,
{
    if let Some(response) = super::config_response::<R>() {
        return response;
    };
    if !validation_callback(request) {
        let error =
            super::ResponseError::<R::E>::CommonError(super::CommonError::InvalidParameterValue);
        return super::error_response(error);
    }

    let opt_json = templates::render_template(template_name, &request);
    match opt_json {
        Some(json) => warp::http::Response::builder()
            .status(http::status_code(200))
            .body(super::responses::json_body(&json))
            .unwrap(),
        _ => super::error_response(super::CommonError::InternalFailure),
    }
}

pub fn to_empty_response<R, F>(request: &R, validation_callback: F) -> Response
where
    R: super::ToActionName + ToResponse + serde::Serialize,
    F: Fn(&R) -> bool,
{
    if let Some(response) = super::config_response::<R>() {
        return response;
    };
    if !validation_callback(request) {
        let error =
            super::ResponseError::<R::E>::CommonError(super::CommonError::InvalidParameterValue);
        return super::error_response(error);
    }

    warp::http::Response::builder()
        .status(crate::http::status_code(200))
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
        let response = error_response(CommonError::AccessDeniedException);
        assert_eq!(http::status_code(400), response.status());
    }

    #[test]
    fn error_response_has_error_header() {
        let response = error_response::<ResponseError<TestError>>(ResponseError::CommonError(
            CommonError::AccessDeniedException,
        ));
        assert!(response.headers().contains_key(AWS_ERROR_TYPE_HEADER));
        assert_eq!(
            "AccessDeniedException",
            response.headers()[AWS_ERROR_TYPE_HEADER]
        );
    }
}
