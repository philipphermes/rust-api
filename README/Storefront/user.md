# Rust API - Storefront

## Register
__Url:__ http://127.0.0.1:3000/storefront/register \
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
{
  "$oid": "64e7491fd75be453220a4b91"
}
```

## Get Profile
__Url:__ http://127.0.0.1:3000/storefront/user \
__Method:__ GET \
__Required:__
* Bearer Token

__Response:__
```
//TODO
```

## Update
__Url:__ http://127.0.0.1:3000/storefront/user/update \
__Method:__ PATCH \
__Required:__
* Bearer Token

__Body:__
```json
{
  "email": "test@email.com",
  "password": "password"
}
```
__Response:__
```
//TODO
```

## Delete
__Url:__ http://127.0.0.1:3000/storefront/user/delete \
__Method:__ DELETE \
__Required:__
* Bearer Token

__Response:__
```
//TODO
```