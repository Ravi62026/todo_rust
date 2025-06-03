# Todo Rust Backend API

A complete Rust backend API with MongoDB, JWT authentication, and CRUD operations for a Todo application.

## Features

- **Authentication**: JWT-based authentication with bcrypt password hashing
- **User Management**: User signup and login
- **Todo CRUD**: Complete Create, Read, Update, Delete operations for todos
- **MongoDB Integration**: Using MongoDB as the database
- **Middleware**: JWT authentication middleware for protected routes
- **Error Handling**: Comprehensive error handling and responses

## Project Structure

```
src/
├── main.rs                 # Application entry point
├── config/
│   └── mod.rs             # Configuration management
├── models/
│   ├── mod.rs
│   ├── user.rs            # User model
│   └── todo.rs            # Todo model
├── handlers/
│   ├── mod.rs
│   ├── auth.rs            # Authentication handlers
│   └── todo.rs            # Todo CRUD handlers
├── middleware/
│   ├── mod.rs
│   └── auth.rs            # JWT authentication middleware
├── database/
│   ├── mod.rs
│   └── connection.rs      # MongoDB connection
└── utils/
    ├── mod.rs
    ├── jwt.rs             # JWT utilities
    └── password.rs        # Password hashing utilities
```

## Prerequisites

1. **Rust**: Install Rust from [rustup.rs](https://rustup.rs/)
2. **MongoDB**: Install and run MongoDB locally or use MongoDB Atlas

## Setup

1. **Clone and navigate to the project**:

   ```bash
   cd todo_rust
   ```

2. **Install dependencies**:

   ```bash
   cargo build
   ```

3. **Configure environment variables**:
   Update the `.env` file with your MongoDB connection string:

   ```env
   MONGODB_URI=mongodb://localhost:27017
   DATABASE_NAME=todo_rust_db
   JWT_SECRET=your_super_secret_jwt_key_here_change_in_production
   ROCKET_PORT=8000
   ROCKET_ADDRESS=127.0.0.1
   ```

4. **Run the application**:
   ```bash
   cargo run
   ```

The server will start on `http://127.0.0.1:8000`

## API Endpoints

### Authentication

#### POST /api/auth/signup

Register a new user.

**Request Body**:

```json
{
  "email": "user@example.com",
  "password": "password123",
  "name": "John Doe"
}
```

**Response**:

```json
{
  "token": "jwt_token_here",
  "user": {
    "id": "user_id",
    "email": "user@example.com",
    "name": "John Doe",
    "created_at": "2024-01-01T00:00:00Z"
  }
}
```

#### POST /api/auth/login

Login with existing credentials.

**Request Body**:

```json
{
  "email": "user@example.com",
  "password": "password123"
}
```

**Response**: Same as signup

### Todos (Protected Routes)

All todo endpoints require the `Authorization: Bearer <token>` header.

#### POST /api/todos

Create a new todo.

**Request Body**:

```json
{
  "title": "Buy groceries",
  "description": "Milk, bread, eggs"
}
```

#### GET /api/todos

Get all todos for the authenticated user.

#### GET /api/todos/{id}

Get a specific todo by ID.

#### PUT /api/todos/{id}

Update a todo.

**Request Body** (all fields optional):

```json
{
  "title": "Updated title",
  "description": "Updated description",
  "completed": true
}
```

#### DELETE /api/todos/{id}

Delete a todo.

## Testing the API

You can test the API using curl, Postman, or any HTTP client.

### Example curl commands:

1. **Signup**:

```bash
curl -X POST http://127.0.0.1:8000/api/auth/signup \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"password123","name":"Test User"}'
```

2. **Create Todo** (replace TOKEN with actual token):

```bash
curl -X POST http://127.0.0.1:8000/api/todos \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer TOKEN" \
  -d '{"title":"Learn Rust","description":"Build a todo API"}'
```

## Quick Start

1. **Build and check the project**:

   ```bash
   chmod +x build_and_test.sh
   ./build_and_test.sh
   ```

2. **Start the server**:

   ```bash
   chmod +x start_server.sh
   ./start_server.sh
   ```

3. **Test the API**:
   ```bash
   chmod +x test_api.sh
   ./test_api.sh
   ```

## Testing with Postman

1. **Import the collection**:

   - Open Postman
   - Click "Import"
   - Select `postman_collection.json`

2. **Set up environment** (optional):

   - Create a new environment in Postman
   - Add variable `base_url` with value `http://127.0.0.1:8000`

3. **Run the tests**:
   - The collection includes automated tests
   - Run requests in order: Signup → Login → Create Todo → Get Todos → Update → Delete
   - Check the "Test Results" tab for validation

## Testing with curl

The `test_api.sh` script provides comprehensive curl-based testing:

```bash
# Make the script executable
chmod +x test_api.sh

# Run all tests
./test_api.sh
```

## Development

- **Run with auto-reload**: `cargo watch -x run`
- **Run tests**: `cargo test`
- **Check code**: `cargo clippy`
- **Format code**: `cargo fmt`
- **Build release**: `cargo build --release`
