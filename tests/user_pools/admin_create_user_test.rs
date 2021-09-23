#[cfg(not(test))]
use log::{info, warn};

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
            "AWSCognitoIdentityProviderService.AdminCreateUser",
        )
        .body(
            r#"{
            "ClientMetadata":{},
            "DesiredDeliveryMediums":[],
            "ForceAliasCreation": true,
            "MessageAction": "string",
            "TemporaryPassword": "RESEND",
            "UserAttributes": [{"Name": "string","Value": "string"}],
            "Username":"username",
            "UserPoolId":"user_pool_id",
            "ValidationData": []
        }"#,
        )
        .reply(&filter)
        .await;

    assert_eq!(200, res.status());
    assert!(!res.body().is_empty())
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
            "AWSCognitoIdentityProviderService.AdminCreateUser",
        )
        .body(
            r#"{
            "ClientMetadata":{},
            "DesiredDeliveryMediums":[],
            "ForceAliasCreation": true,
            "MessageAction": "string",
            "TemporaryPassword": "RESEND",
            "UserAttributes": [{"Name": "string","Value": "string"}],
            "Username":"",
            "UserPoolId":"user_pool_id",
            "ValidationData": []
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
