use bytes::Bytes;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

const AWS_ERROR_TYPE_HEADER: &str = "x-amzn-ErrorType";

pub type Response = warp::http::Response<hyper::Body>;
pub type UserPoolsResponseResult = std::result::Result<Response, Infallible>;

pub trait IntoResponse {
    fn into_response(&self) -> Response;
}

pub trait ToStatusCode {
    fn to_status_code(&self) -> StatusCode;
}

/// Returns empty body.
pub fn empty_body() -> hyper::Body {
    hyper::Body::empty()
}

/// Returns body with json serialized value.
pub fn json_body<T>(value: T) -> hyper::Body
where
    T: Serialize,
{
    hyper::Body::from(serde_json::to_string(&value).unwrap())
}

// TODO
pub fn response<'a, T>(body: &'a Bytes) -> UserPoolsResponseResult
where
    T: Serialize + for<'de> Deserialize<'de> + IntoResponse,
{
    match serde_json::from_slice::<T>(body) {
        Ok(req) => Ok(req.into_response()),
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
        .body(empty_body())
        .unwrap()
}

pub fn config_response<T>() -> Option<Response>
where
    T: std::str::FromStr + std::fmt::Display + ToStatusCode + super::GetConfig,
{
    use std::str::FromStr;
    if let Some(name) = T::get_config(&super::CONFIG_STATUS_NAME.to_string()) {
        let error = super::ResponseError::<T>::from_str(name.as_str());
        if let Ok(e) = error {
            return Some(super::error_response(e));
        }
    }

    return None;
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
