# Fakey Cognito 

![test](https://github.com/naokirin/fakey-cognito/actions/workflows/test.yml/badge.svg)


:house_with_garden: [Homepage](https://github.com/naokirin/fakey-cognito)

Simple fake AWS Cognito API server for development.

### :white_check_mark: Implemented features

* [x] AdminXxx on User Pools API.

## Get Started

```sh
# run with cargo on port 8080
cargo run &

# or run with docker
docker build -t fakey-cognito .
docker run -p 8080:8080 fakey-cognito

# request AdminGetUser
curl -X POST http://localhost:8080 \
     -H 'Content-Type:application/x-amz-json-1.1' \
     -H 'X-Amz-Target:AWSCognitoIdentityProviderService.AdminGetUser' \
     -d '{"Username":"${USER_NAME}","UserPoolId":"${USER_POOL_ID}"}'
```

## Prerequisites

* rust >= 1.55.0

OR

* docker >= 19.03

## Usage

First read "[Get Started](#get-started)".

### Configurations

You use configuration with file (YAML).  
Default configuration path is `/repository_root/config.yml` .

Following configuration file format.
```yaml
ActionName:
  error_type: <force response error type (e.g. InternalFailure)>

...
```

When use custom path you specify a command line arguments.

```sh
cargo run -- --config /path/to/config.yml
```

### Response templates

Fackey-cognito returns simple response by default.  
It also returns rendering templates if you need custom responses.

Default template path is `/repository_root/templates/**/*.json` .  
Template file name is a action name and a directory is `user_pools`, `user_pools_auth`.  

You can use request parameters in template.

Example for template file (e.g.`user_pools/AdminGetDevice.json`).
```json
{
   "Device": { 
      "DeviceAttributes": [ 
         { 
            "Name": "DeviceName",
            "Value": "DeviceValue"
         }
      ],
      "DeviceCreateDate": {{ now() | date(format="%Y%m%d%H%M%S" )}},
      "DeviceKey": "{{DeviceKey}}",
      "DeviceLastAuthenticatedDate": {{ now() | date(format="%Y%m%d%H%M%S" )}},
      "DeviceLastModifiedDate": {{ now() | date(format="%Y%m%d%H%M%S" )}}
   }
}
```

Rendering template by [tera](https://github.com/Keats/tera) (Tera is Jinja2 like template engine).

When use custom path you specify a command line arguments.

```sh
cargo run -- --templates /path/to/templates
```

### Hooks

Can run python hook scripts before responses.

fakey-cognito calls a `hook` function in snake_case action name python script.  
A `hook` function takes a request json body string and should return a json string for using a response template.

Example for AdminGetUser action hook script (e.g.`hooks/admin_get_user.py`).
```py
def hook(request):
    return '{ "json": "response" }'
```

Default hook scripts directory path is `./hooks`.  
When use custom path you specify a command line arguments.

```sh
cargo run -- --hooks /path/to/hooks
```

## :warning: Unsupported features

* Request parameter validations
* Error response with corrected messages (but supported error types)
* No check any authentication and authorization

## License

Fakey Cognito(fakey-cognito) is under [MIT License](https://github.com/naokirin/fakey-cognito/blob/master/LICENSE)

## Authors

* [naokirin](https://github.com/naokirin)