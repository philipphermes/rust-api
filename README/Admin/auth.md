# Rust API - Admin-Api

## Create Auth
__Url:__ http://127.0.0.1:3000/admin-api/auth/create \
__Method:__ POST \
__Required:__ 
* Admin User
* Bearer Token

__Body:__
```json
{
    "name": "Test",
    "scopes": [
        "read_user"
    ]
}
```
__Response:__
```json
{
    "_id": {
        "$oid": "64e843a532a1bdef62a1badc"
    },
    "name": "Test",
    "token": "ecf1c91f-45cf-4319-b3da-cbed11d7f1f4",
    "scopes": [
        "read_user"
    ]
}
```