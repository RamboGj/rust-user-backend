# Rust Axum Backend Project

This is a backend project written in Rust using the Axum framework and Tokio runtime. The project allows you to create users, get a user by UUID, and retrieve all users. The database is in-memory, and the project provides a simple API for managing user data.

## Features

- Create a new user
- Retrieve a user by UUID
- Get a list of all users

## Technologies Used

- Rust
- Axum
- Tokio

## Prerequisites

- Rust installed (you can download it from [rustup.rs](https://rustup.rs/))

## Getting Started

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/your-axum-backend.git
   cd your-axum-backend
   ```

2. Build and run the project:

   ```bash
   cargo run
   ```

   This will start the server at `http://localhost:8080`.

3. Test the API using your favorite HTTP client (e.g., [curl](https://curl.se/) or [Postman](https://www.postman.com/)).

## API Endpoints

- **Create User**

  ```
  POST /users
  Content-Type: application/json

  {
    "name": "John Doe",
    "username": "johndoe"
  }
  ```

- **Get User by UUID**

  ```
  GET /users/{uuid}
  ```

- **Get All Users**

  ```
  GET /users
  ```

## Example Usage

```bash
# Create a new user
curl -X POST -H "Content-Type: application/json" -d '{"name": "John Doe", "username": "johndoe"}' http://localhost:8080/users

# Get a user by UUID (replace {uuid} with the actual UUID)
curl http://localhost:8080/users/{uuid}

# Get all users
curl http://localhost:8080/users
```

## In-Memory Database

The project uses an in-memory database to store user information. The `User` struct and `UsersDb` type are defined as follows:

```rust
use serde::Serialize;
use std::{
  collections::HashMap,
  sync::{Arc, RwLock},
};
use uuid::Uuid;

#[derive(Debug, Serialize, Clone)]
pub struct User {
  pub id: Uuid,
  pub name: String,
  pub username: String,
}

pub type UsersDb = Arc<RwLock<HashMap<Uuid, User>>>;
```
```
