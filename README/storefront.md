# Rust API - Storefront
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