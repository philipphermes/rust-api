# Rust API

## Usage
```shell
docker-compose up -d
cargo run
```

## Storefront
### Auth
* http://127.0.0.1:8000/auth/login
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
* http://127.0.0.1:8000/auth/logout
  * POST
  * Logout
  * Bearer Token required
  * Response:
  ```json
  "logged out"
  ```

### Routes
* http://127.0.0.1:8000/storefront
  * GET
  * Shows all Storefront routes
* http://127.0.0.1:8000/storefront/user
  * GET
  * Get your user profile
  * Bearer Token required
* http://127.0.0.1:8000/storefront/register
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
* http://127.0.0.1:8000/storefront/user/delete
  * DELETE
  * Delete your user
  * Bearer Token required
* http://127.0.0.1:8000/storefront/user/update
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
* http://127.0.0.1:8000/admin-api/auth/create
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