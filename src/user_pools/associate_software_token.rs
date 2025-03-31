use crate::common::TOKEN_REGEX;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use validator::Validate;

pub const ASSOCIATE_SOFTWARE_TOKEN_NAME: &str = "AssociateSoftwareToken";
pub const ASSOCIATE_SOFTWARE_TOKEN_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AssociateSoftwareToken";

super::gen_response_err!(
    AssociateSoftwareTokenError,
    InvalidParameterException
    | NotAuthorizedException
    | ResourceNotFoundException
    | SoftwareTokenMFANotFoundException => http::status_code(400),
    InternalErrorException => http::status_code(500)
);

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct AssociateSoftwareTokenRequest {
    #[validate(regex(path = *TOKEN_REGEX))]
    access_token: Option<String>,
    #[validate(length(min = 20, max = 2048))]
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
        super::to_json_response(self, ASSOCIATE_SOFTWARE_TOKEN_NAME)
    }
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
        assert!(request.validate().is_ok());
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
