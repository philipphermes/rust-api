# Rust API

## Usage
```shell
docker-compose up -d
cargo run
```

## Routes
* http://127.0.0.1:8000/
  * GET
  * Hello World
* http://127.0.0.1:8000/user
  * GET
  * Get your user profile
  * Bearer Token required
* http://127.0.0.1:8000/user/create
  * POST
  * Create new User
  * Body: `{"email": "test@email.com","password": "password"}`
* http://127.0.0.1:8000/user/delete
  * DELETE
  * Delete your user
  * Bearer Token required
* http://127.0.0.1:8000/user/update
  * PATCH
  * Update your user
  * Bearer Token required
  * Body: `{"email": "test@email.com","password": "password"}`

## Auth
* http://127.0.0.1:8000/auth/login
  * POST
  * Login
  * Body: `{"email": "test@email.com","password": "password"}`
  * Response: `"5f6e8bd6-3cc6-4cab-9d09-9bd5d0e056e7"`
* http://127.0.0.1:8000/auth/logout
  * POST
  * Logout
  * Bearer Token required
  * Response: `"logged out"`