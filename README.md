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