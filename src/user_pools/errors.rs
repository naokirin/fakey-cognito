use crate::http;
use std::fmt::Display;
use strum_macros::{Display, EnumString};

/// Generate response error enum and function to_status_code.
#[macro_export]
macro_rules! gen_response_err {
    ($t:ident,$($( $errs:ident )|+ => $s:expr),*) => {
        use super::ToStatusCode;
        use hyper::StatusCode;

        #[allow(clippy::enum_variant_names)]
        #[derive(Display, EnumString)]
        pub enum $t {
            $( $( $errs ),+ ),*
        }

        impl ToStatusCode for $t {
            fn to_status_code(&self) -> StatusCode {
                match self {
                    $(
                        $(
                            $t::$errs => $s,
                        )+
                    )*
                }
            }
        }
    }
}
pub(crate) use gen_response_err;

gen_response_err!(
    CommonError,
    AccessDeniedException
    | IncompleteSignature
    | InvalidAction
    | InvalidParameterCombination
    | InvalidParameterValue
    | InvalidQueryParameter
    | MissingAction
    | MissingParameter
    | NotAuthorized
    | RequestExpired
    | ThrottlingException
    | ValidationError => http::status_code(400),
    InvalidClientTokenId
    | MissingAuthenticationToken
    | OptInRequired => http::status_code(403),
    MalformedQueryString => http::status_code(404),
    InternalFailure => http::status_code(500),
    ServiceUnavailable => http::status_code(503)
);

/// Response errors for any actions.
pub enum ResponseError<T>
where
    T: std::fmt::Display + ToStatusCode + std::str::FromStr,
{
    ActionError(T),
    CommonError(CommonError),
}

impl<T> ToStatusCode for ResponseError<T>
where
    T: std::fmt::Display + ToStatusCode + std::str::FromStr,
{
    fn to_status_code(&self) -> StatusCode {
        match self {
            ResponseError::<T>::CommonError(err) => err.to_status_code(),
            ResponseError::ActionError(err) => err.to_status_code(),
        }
    }
}

impl<T> Display for ResponseError<T>
where
    T: std::fmt::Display + ToStatusCode + std::str::FromStr,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ResponseError::<T>::CommonError(err) => write!(f, "{}", err),
            ResponseError::ActionError(err) => write!(f, "{}", err),
        }
    }
}

impl<T> std::str::FromStr for ResponseError<T>
where
    T: std::fmt::Display + ToStatusCode + std::str::FromStr,
{
    type Err = strum::ParseError;

    fn from_str(s: &str) -> Result<Self, strum::ParseError> {
        T::from_str(s)
            .map(ResponseError::ActionError)
            .or_else(|_| CommonError::from_str(s).map(ResponseError::CommonError))
    }
}
