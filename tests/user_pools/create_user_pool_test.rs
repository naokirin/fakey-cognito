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
            "AWSCognitoIdentityProviderService.CreateUserPool",
        )
        .body(
            r#"
        {
            "AccountRecoverySetting": { 
               "RecoveryMechanisms": [ 
                  { 
                     "Name": "string",
                     "Priority": 1
                  }
               ]
            },
            "AdminCreateUserConfig": { 
               "AllowAdminCreateUserOnly": false,
               "InviteMessageTemplate": { 
                  "EmailMessage": "string",
                  "EmailSubject": "string",
                  "SMSMessage": "string"
               },
               "UnusedAccountValidityDays": 7
            },
            "AliasAttributes": [ "string" ],
            "AutoVerifiedAttributes": [ "string" ],
            "DeviceConfiguration": { 
               "ChallengeRequiredOnNewDevice": true,
               "DeviceOnlyRememberedOnUserPrompt": true 
            },
            "EmailConfiguration": { 
               "ConfigurationSet": "string",
               "EmailSendingAccount": "string",
               "From": "string",
               "ReplyToEmailAddress": "string",
               "SourceArn": "string"
            },
            "EmailVerificationMessage": "string",
            "EmailVerificationSubject": "string",
            "LambdaConfig": { 
               "CreateAuthChallenge": "string",
               "CustomEmailSender": { 
                  "LambdaArn": "string",
                  "LambdaVersion": "string"
               },
               "CustomMessage": "string",
               "CustomSMSSender": { 
                  "LambdaArn": "string",
                  "LambdaVersion": "string"
               },
               "DefineAuthChallenge": "string",
               "KMSKeyID": "string",
               "PostAuthentication": "string",
               "PostConfirmation": "string",
               "PreAuthentication": "string",
               "PreSignUp": "string",
               "PreTokenGeneration": "string",
               "UserMigration": "string",
               "VerifyAuthChallengeResponse": "string"
            },
            "MfaConfiguration": "string",
            "Policies": { 
               "PasswordPolicy": { 
                  "MinimumLength": 10,
                  "RequireLowercase": false,
                  "RequireNumbers": false,
                  "RequireSymbols": false,
                  "RequireUppercase": false,
                  "TemporaryPasswordValidityDays": 7
               }
            },
            "PoolName": "string",
            "Schema": [ 
               { 
                  "AttributeDataType": "string",
                  "DeveloperOnlyAttribute": false,
                  "Mutable": true,
                  "Name": "string",
                  "NumberAttributeConstraints": { 
                     "MaxValue": "string",
                     "MinValue": "string"
                  },
                  "Required": true,
                  "StringAttributeConstraints": { 
                     "MaxLength": "string",
                     "MinLength": "string"
                  }
               }
            ],
            "SmsAuthenticationMessage": "string",
            "SmsConfiguration": { 
               "ExternalId": "string",
               "SnsCallerArn": "string"
            },
            "SmsVerificationMessage": "string",
            "UsernameAttributes": [ "string" ],
            "UsernameConfiguration": { 
               "CaseSensitive": false
            },
            "UserPoolAddOns": { 
               "AdvancedSecurityMode": "string"
            },
            "UserPoolTags": { 
               "string" : "string" 
            },
            "VerificationMessageTemplate": { 
               "DefaultEmailOption": "string",
               "EmailMessage": "string",
               "EmailMessageByLink": "string",
               "EmailSubject": "string",
               "EmailSubjectByLink": "string",
               "SmsMessage": "string"
            }
         }
        "#,
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
            "AWSCognitoIdentityProviderService.CreateUserPool",
        )
        .body(
            r#"
        {
            "AccountRecoverySetting": { 
               "RecoveryMechanisms": [ 
                  { 
                     "Name": "string",
                     "Priority": 1
                  }
               ]
            },
            "AdminCreateUserConfig": { 
               "AllowAdminCreateUserOnly": false,
               "InviteMessageTemplate": { 
                  "EmailMessage": "string",
                  "EmailSubject": "string",
                  "SMSMessage": "string"
               },
               "UnusedAccountValidityDays": 7
            },
            "AliasAttributes": [ "string" ],
            "AutoVerifiedAttributes": [ "string" ],
            "DeviceConfiguration": { 
               "ChallengeRequiredOnNewDevice": true,
               "DeviceOnlyRememberedOnUserPrompt": true 
            },
            "EmailConfiguration": { 
               "ConfigurationSet": "string",
               "EmailSendingAccount": "string",
               "From": "string",
               "ReplyToEmailAddress": "string",
               "SourceArn": "string"
            },
            "EmailVerificationMessage": "string",
            "EmailVerificationSubject": "string",
            "LambdaConfig": { 
               "CreateAuthChallenge": "string",
               "CustomEmailSender": { 
                  "LambdaArn": "string",
                  "LambdaVersion": "string"
               },
               "CustomMessage": "string",
               "CustomSMSSender": { 
                  "LambdaArn": "string",
                  "LambdaVersion": "string"
               },
               "DefineAuthChallenge": "string",
               "KMSKeyID": "string",
               "PostAuthentication": "string",
               "PostConfirmation": "string",
               "PreAuthentication": "string",
               "PreSignUp": "string",
               "PreTokenGeneration": "string",
               "UserMigration": "string",
               "VerifyAuthChallengeResponse": "string"
            },
            "MfaConfiguration": "string",
            "Policies": { 
               "PasswordPolicy": { 
                  "MinimumLength": 10,
                  "RequireLowercase": false,
                  "RequireNumbers": false,
                  "RequireSymbols": false,
                  "RequireUppercase": false,
                  "TemporaryPasswordValidityDays": 7
               }
            },
            "PoolName": "",
            "Schema": [ 
               { 
                  "AttributeDataType": "string",
                  "DeveloperOnlyAttribute": false,
                  "Mutable": true,
                  "Name": "string",
                  "NumberAttributeConstraints": { 
                     "MaxValue": "string",
                     "MinValue": "string"
                  },
                  "Required": true,
                  "StringAttributeConstraints": { 
                     "MaxLength": "string",
                     "MinLength": "string"
                  }
               }
            ],
            "SmsAuthenticationMessage": "string",
            "SmsConfiguration": { 
               "ExternalId": "string",
               "SnsCallerArn": "string"
            },
            "SmsVerificationMessage": "string",
            "UsernameAttributes": [ "string" ],
            "UsernameConfiguration": { 
               "CaseSensitive": false
            },
            "UserPoolAddOns": { 
               "AdvancedSecurityMode": "string"
            },
            "UserPoolTags": { 
               "string" : "string" 
            },
            "VerificationMessageTemplate": { 
               "DefaultEmailOption": "string",
               "EmailMessage": "string",
               "EmailMessageByLink": "string",
               "EmailSubject": "string",
               "EmailSubjectByLink": "string",
               "SmsMessage": "string"
            }
         }
        "#,
        )
        .reply(&filter)
        .await;

    assert_eq!(400, res.status());
    assert_eq!(
        "InvalidParameterValue",
        res.headers().get("x-amzn-ErrorType").unwrap()
    )
}
