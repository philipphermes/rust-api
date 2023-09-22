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
  "_id": "bb134252-ebe6-4ea9-858f-28d620a2c4d4",
  "email": "test2@email.com",
  "password": "$2b$10$bCg4fGc/GI3/fDSG0lUj1OTfbNzvaaoBW681x/Qi8jgp1u/SMj6ii",
  "token": null,
  "roles": [
    "ROLE_USER",
    "ROLE_ADMIN"
  ]
}
```

## Get Profile
__Url:__ http://127.0.0.1:3000/storefront/user \
__Method:__ GET \
__Required:__
* Bearer Token

__Response:__
```json
{
    "_id": "346bffff-ae14-4fe7-b5f6-94b040778315",
    "email": "test@email.com",
    "password": "$2b$10$tNFsFYmJei.Se00pmjd0WuzOSfmuUPnzU5gg5DSHk7SSLSwdTAez2",
    "token": "471b3437-b392-4adf-9be2-8c0f31b3f7f6",
    "roles": [
        "ROLE_USER",
        "ROLE_ADMIN"
    ]
}
```

## Update
__Url:__ http://127.0.0.1:3000/storefront/user/update \
__Method:__ PATCH \
__Required:__
* Bearer Token

__Body:__
```json
{
  "email": "test123@email.com",
  "password": "password"
}
```
__Response:__
```json
{
    "_id": "346bffff-ae14-4fe7-b5f6-94b040778315",
    "email": "test123@email.com",
    "password": "$2b$10$Ludm.QhzZ4WfAEvKheIuMuuQitpfL1iivvg8FMUdaAnhyap6NIWv2",
    "token": "471b3437-b392-4adf-9be2-8c0f31b3f7f6",
    "roles": [
        "ROLE_USER",
        "ROLE_ADMIN"
    ]
}
```

## Delete
__Url:__ http://127.0.0.1:3000/storefront/user/delete \
__Method:__ DELETE \
__Required:__
* Bearer Token

__Response:__
```json
"User was deleted"
```