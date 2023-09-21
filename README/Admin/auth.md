# Rust API - Admin-Api

## Create Auth
**Url:** http://127.0.0.1:3000/admin-api/auth/create
**Method:** POST
**Required:**
* Admin User
* Bearer Token
**Body:**
```json
{
    "name": "Test",
    "scopes": [
        "read_user"
    ]
}
```
**Response:**
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