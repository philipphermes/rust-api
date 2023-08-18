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
* http://127.0.0.1:8000/user/{email}
  * GET
  * Get user with email
* http://127.0.0.1:8000/user/create
  * POST
  * Create new User
  * Body: `{"email": "test@email.com","password": "password"}`
* http://127.0.0.1:8000/auth/login
  * POST
  * Login
  * Body: `{"email": "test@email.com","password": "password","token": ""}` TODO remove token from user and make own doc
  * Response: `"5f6e8bd6-3cc6-4cab-9d09-9bd5d0e056e7"`
* http://127.0.0.1:8000/auth
  * POST
  * Auth
  * Body: `{"token": "5f6e8bd6-3cc6-4cab-9d09-9bd5d0e056e7"}` TODO remove token from user and make own doc
  * Response: `"5f6e8bd6-3cc6-4cab-9d09-9bd5d0e056e7"`
* http://127.0.0.1:8000/auth/logout
  * POST
  * Logout
  * Body: `{"token": "5f6e8bd6-3cc6-4cab-9d09-9bd5d0e056e7"}` TODO remove token from user and make own doc
  * Response: `"logged out"`