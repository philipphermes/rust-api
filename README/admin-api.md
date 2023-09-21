# Rust API - Admin-Api

### [Auth](https://github.com/philipphermes/rust-api/blob/main/README/Admin/auth.md)
Docs for the Admin-Api Authentication

### Category
* http://127.0.0.1:3000/admin-api/category
    * POST
    * Create Category
    * Bearer Token required
    * read_category and write_category scopes required
    * Body:
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
    * Response
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

* http://127.0.0.1:3000/admin-api/category/{id}
    * GET
    * get Category by id
    * Bearer Token required
    * read_category scope required
    * Response
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
* http://127.0.0.1:3000/admin-api/category
    * PATCH
    * update Category, Products, Variant,
    * Bearer Token required
    * read_category and write category scopes required
    * Body
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
    * Response:
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
* http://127.0.0.1:3000/admin-api/category/{id}
    * DELETE
    * delete Category, Products, Variant,
    * Bearer Token required
    * read_category and write category scopes required
    * Response
  ```json
  {
    "deletedCount": 1
  }
  ```