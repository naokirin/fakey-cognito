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
            "AWSCognitoIdentityProviderService.AdminInitiateAuth",
        )
        .body(
            r#"{
            "AnalyticsMetadata": { 
               "AnalyticsEndpointId": "string"
            },
            "AuthFlow": "USER_SRP_AUTH",
            "AuthParameters": { 
               "string" : "string" 
            },
            "ClientId": "string",
            "ClientMetadata": { 
               "string" : "string" 
            },
            "ContextData": { 
               "EncodedData": "string",
               "HttpHeaders": [ 
                  { 
                     "headerName": "string",
                     "headerValue": "string"
                  }
               ],
               "IpAddress": "string",
               "ServerName": "string",
               "ServerPath": "string"
            },
            "UserPoolId": "string_09"
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
            "AWSCognitoIdentityProviderService.AdminInitiateAuth",
        )
        .body(
            r#"{
            "AnalyticsMetadata": { 
               "AnalyticsEndpointId": "string"
            },
            "AuthFlow": "USER_SRP_AUTH",
            "AuthParameters": { 
               "string" : "string" 
            },
            "ClientId": "string",
            "ClientMetadata": { 
               "string" : "string" 
            },
            "ContextData": { 
               "EncodedData": "string",
               "HttpHeaders": [ 
                  { 
                     "headerName": "string",
                     "headerValue": "string"
                  }
               ],
               "IpAddress": "string",
               "ServerName": "string",
               "ServerPath": "string"
            },
            "UserPoolId": ""
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
