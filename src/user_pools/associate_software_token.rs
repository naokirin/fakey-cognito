use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const ASSOCIATE_SOFTWARE_TOKEN_NAME: &str = "AssociateSoftwareToken";
pub const ASSOCIATE_SOFTWARE_TOKEN_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AssociateSoftwareToken";

/// AssociateSoftwareToken response errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AssociateSoftwareToken.html#API_AssociateSoftwareToken_Errors
#[allow(clippy::enum_variant_names)]
#[derive(Display, EnumString)]
pub enum AssociateSoftwareTokenError {
    ConcurrentModificationException,
    InternalErrorException,
    InvalidParameterException,
    NotAuthorizedException,
    ResourceNotFoundException,
    SoftwareTokenMFANotFoundException,
}

impl super::ToStatusCode for AssociateSoftwareTokenError {
    fn to_status_code(&self) -> hyper::StatusCode {
        match self {
            AssociateSoftwareTokenError::InvalidParameterException
            | AssociateSoftwareTokenError::NotAuthorizedException
            | AssociateSoftwareTokenError::ResourceNotFoundException
            | AssociateSoftwareTokenError::SoftwareTokenMFANotFoundException => {
                http::status_code(400)
            }
            _ => http::status_code(500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AssociateSoftwareTokenRequest {
    access_token: Option<String>,
    session: Option<String>,
}

impl super::ToActionName for AssociateSoftwareTokenRequest {
    fn to_action_name() -> &'static str {
        ASSOCIATE_SOFTWARE_TOKEN_NAME
    }
}

impl super::ToResponse for AssociateSoftwareTokenRequest {
    type E = AssociateSoftwareTokenError;
    fn to_response(&self) -> super::Response {
        super::to_json_response(self, ASSOCIATE_SOFTWARE_TOKEN_NAME, valid_request)
    }
}

/// Validates request.
fn valid_request(_request: &AssociateSoftwareTokenRequest) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = AssociateSoftwareTokenRequest {
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = AssociateSoftwareTokenError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AssociateSoftwareTokenError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
