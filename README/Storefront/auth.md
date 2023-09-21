# Rust API - Storefront

## Login
__Url:__ http://127.0.0.1:3000/auth/login \
__Method:__ POST \
__Body:__
```json
{
  "email": "test@email.com",
  "password": "password"
}
```
__Response:__
```json
"5f6e8bd6-3cc6-4cab-9d09-9bd5d0e056e7"
```

## Logout
__Url:__ http://127.0.0.1:3000/auth/logout \
__Method:__ POST \
__Required:__
* Bearer Token

__Response:__
```json
"logged out"
```