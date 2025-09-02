# Actix Web Server Example

This project is a simple **Rust Actix Web server** that demonstrates how to:

- Create routes with **GET** and **POST**.
- Store users in an in-memory database using `HashMap` wrapped with `Arc<Mutex<...>>`.
- Handle JSON serialization and deserialization with `serde`.

## Features

- `GET /greet/{id}` – Returns a greeting for the given user ID.
- `POST /users` – Creates a new user and stores it in memory.

## Project Structure

````

```bash
src/
├── main.rs # Entry point of the Actix Web server
Cargo.toml # Project dependencies and metadata
README.md  # Project documentation
````

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable recommended)
- Cargo (comes with Rust)

## Getting Started

### 1. Clone the Repository

````bash
git clone https://github.com/your-username/actix-web-server.git
cd actix-web-server


Run the Server

```bash
cargo run
````

The server will start at:

```cpp
http://127.0.0.1:8080
```

API Endpoints

Request

```http
GET /greet/{id}
```

Example

```curl
curl http://127.0.0.1:8080/greet/1
```

Response

```json
{
  "message": "Hello, User 1!"
}
```

### 2. Create a New User

Request

```http
POST /users

Content-Type: application/json
```

Notes

The database is in-memory only (cleared when the server restarts).
This is a minimal example for learning purposes.
