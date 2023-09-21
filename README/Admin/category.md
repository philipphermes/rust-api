# Rust API - Admin-Api

## Create Category
__Url:__ http://127.0.0.1:3000/admin-api/category \
__Method:__ POST \
__Required:__
* Bearer Token
* read_category and write_category scopes

__Body:__
```json
{
  "description": "Test Category",
  "products": [
    {
      "sku": "100",
      "description": "Test Prod",
      "variants": [
        {
          "sku": "100-001",
          "description": "Variante 1",
          "price": 69.99,
          "image_url": "/100-001.jpeg"
        }
      ]
    }
  ]
}
```
__Response:__
```json
{
  "_id": "cafb016a-6dbc-40cc-b166-7b8b2b9cd452",
  "description": "Test Category",
  "products": [
    {
      "_id": "b603364d-ba15-447a-ad3b-9457ff36a503",
      "sku": "100",
      "description": "Test Prod",
      "variants": [
        {
          "_id": "1840c4d1-0bbf-45ac-9f9c-bd5c851abc29",
          "sku": "100-001",
          "description": "Variante 1",
          "price": 69.99,
          "sale_price": null,
          "image_url": "/100-001.jpeg"
        }
      ]
    }
  ]
}
```

## Update Category
__Url:__ http://127.0.0.1:3000/admin-api/category \
__Method:__ PATCH \
__Required:__
* Bearer Token
* read_category and write_category scopes

__Body:__
```json
{
  "_id": "cafb016a-6dbc-40cc-b166-7b8b2b9cd452",
  "description": "Test Category Update",
  "products": [
    {
      "_id": "b603364d-ba15-447a-ad3b-9457ff36a503",
      "sku": "101",
      "description": "Test Prod",
      "variants": [
        {
          "_id": "1840c4d1-0bbf-45ac-9f9c-bd5c851abc29",
          "sku": "101-001",
          "description": "Variante 1 Update",
          "price": 99.99,
          "sale_price": null,
          "image_url": "/101-001.jpeg"
        }
      ]
    }
  ]
}
```
__Response:__
```json
{
  "_id": "cafb016a-6dbc-40cc-b166-7b8b2b9cd452",
  "description": "Test Category Update",
  "products": [
    {
      "_id": "b603364d-ba15-447a-ad3b-9457ff36a503",
      "sku": "101",
      "description": "Test Prod",
      "variants": [
        {
          "_id": "1840c4d1-0bbf-45ac-9f9c-bd5c851abc29",
          "sku": "101-001",
          "description": "Variante 1 Update",
          "price": 99.99,
          "sale_price": null,
          "image_url": "/101-001.jpeg"
        }
      ]
    }
  ]
}
```

## Delete Category
__Url:__ http://127.0.0.1:3000/admin-api/category/{id} \
__Method:__ DELETE \
__Required:__
* Bearer Token
* read_category and write_category scopes

__Response:__
```json
{
  "deletedCount": 1
}
```

## Get Category
__Url:__ http://127.0.0.1:3000/admin-api/category/{id} \
__Method:__ GET \
__Required:__
* Bearer Token
* read_category scope

__Response:__
```json
{
  "_id": "cafb016a-6dbc-40cc-b166-7b8b2b9cd452",
  "description": "Test Category",
  "products": [
    {
      "_id": "b603364d-ba15-447a-ad3b-9457ff36a503",
      "sku": "100",
      "description": "Test Prod",
      "variants": [
        {
          "_id": "1840c4d1-0bbf-45ac-9f9c-bd5c851abc29",
          "sku": "100-001",
          "description": "Variante 1",
          "price": 69.99,
          "sale_price": null,
          "image_url": "/100-001.jpeg"
        }
      ]
    }
  ]
}
```