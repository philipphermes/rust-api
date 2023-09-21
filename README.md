# Rust API

## Usage
```shell
docker-compose up -d
cargo run
```

## Storefront
### Auth
* http://127.0.0.1:3000/auth/login
  * POST
  * Login
  * Body:
  ```json
  {
    "email": "test@email.com",
    "password": "password"
  }
  ```
  * Response:
  ```json
  "5f6e8bd6-3cc6-4cab-9d09-9bd5d0e056e7"
  ```
* http://127.0.0.1:3000/auth/logout
  * POST
  * Logout
  * Bearer Token required
  * Response:
  ```json
  "logged out"
  ```

### Routes
* http://127.0.0.1:3000/storefront
  * GET
  * Shows all Storefront routes
* http://127.0.0.1:3000/storefront/user
  * GET
  * Get your user profile
  * Bearer Token required
* http://127.0.0.1:3000/storefront/register
  * POST
  * Create new User
  * Body:
  ```json
  {
    "email": "test@email.com",
    "password": "password"
  }
  ```
  * Response:
  ```json
  {
    "$oid": "64e7491fd75be453220a4b91"
  }
  ```
* http://127.0.0.1:3000/storefront/user/delete
  * DELETE
  * Delete your user
  * Bearer Token required
* http://127.0.0.1:3000/storefront/user/update
  * PATCH
  * Update your user
  * Bearer Token required
  * Body:
  ```json
  {
    "email": "test@email.com",
    "password": "password"
  }
  ```

## Admin-Api
### Auth
* http://127.0.0.1:3000/admin-api/auth/create
  * POST
  * Create Api Auth
  * Admin User required
  * Bearer Token required
  * Body: 
  ```json
  {
    "name": "Test",
    "scopes": [
      "read_user"
    ]
  }
  ```
  * Response: 
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