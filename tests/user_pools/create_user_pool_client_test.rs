use fakey_cognito::*;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn test_success_to_request() {
    super::setup().await;

    let filter = routes::user_pools_routes();
    let res = warp::test::request()
        .method("POST")
        .path("/")
        .header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.CreateUserPoolClient",
        )
        .body(
            r#"{
            "AccessTokenValidity": 1,
            "AllowedOAuthFlows": [ "string" ],
            "AllowedOAuthFlowsUserPoolClient": true,
            "AllowedOAuthScopes": [ "string" ],
            "AnalyticsConfiguration": { 
               "ApplicationArn": "string",
               "ApplicationId": "string",
               "ExternalId": "string",
               "RoleArn": "string",
               "UserDataShared": true
            },
            "CallbackURLs": [ "string" ],
            "ClientName": "string",
            "DefaultRedirectURI": "string",
            "EnableTokenRevocation": true,
            "ExplicitAuthFlows": [ "string" ],
            "GenerateSecret": true,
            "IdTokenValidity": 1,
            "LogoutURLs": [ "string" ],
            "PreventUserExistenceErrors": "string",
            "ReadAttributes": [ "string" ],
            "RefreshTokenValidity": 1,
            "SupportedIdentityProviders": [ "string" ],
            "TokenValidityUnits": { 
               "AccessToken": "string",
               "IdToken": "string",
               "RefreshToken": "string"
            },
            "UserPoolId": "string",
            "WriteAttributes": [ "string" ]
         }"#,
        )
        .reply(&filter)
        .await;

    assert_eq!(200, res.status());
    assert!(!res.body().is_empty());
}

#[tokio::test]
async fn test_failure_to_request() {
    super::setup().await;

    let filter = routes::user_pools_routes();
    let res = warp::test::request()
        .method("POST")
        .path("/")
        .header(
            "x-amz-target",
            "AWSCognitoIdentityProviderService.CreateUserPoolClient",
        )
        .body(
            r#"{
            "AccessTokenValidity": 1,
            "AllowedOAuthFlows": [ "string" ],
            "AllowedOAuthFlowsUserPoolClient": true,
            "AllowedOAuthScopes": [ "string" ],
            "AnalyticsConfiguration": { 
               "ApplicationArn": "string",
               "ApplicationId": "string",
               "ExternalId": "string",
               "RoleArn": "string",
               "UserDataShared": true
            },
            "CallbackURLs": [ "string" ],
            "ClientName": "string",
            "DefaultRedirectURI": "string",
            "EnableTokenRevocation": true,
            "ExplicitAuthFlows": [ "string" ],
            "GenerateSecret": true,
            "IdTokenValidity": 1,
            "LogoutURLs": [ "string" ],
            "PreventUserExistenceErrors": "string",
            "ReadAttributes": [ "string" ],
            "RefreshTokenValidity": 1,
            "SupportedIdentityProviders": [ "string" ],
            "TokenValidityUnits": { 
               "AccessToken": "string",
               "IdToken": "string",
               "RefreshToken": "string"
            },
            "UserPoolId": "",
            "WriteAttributes": [ "string" ]
         }"#,
        )
        .reply(&filter)
        .await;

    assert_eq!(400, res.status());
    assert_eq!(
        "InvalidParameterValue",
        res.headers().get("x-amzn-ErrorType").unwrap()
    )
}
