# 🧪 Complete Testing Guide for Todo Rust API

## 🚀 Server Status
✅ **Server is running successfully on http://127.0.0.1:8000**
✅ **MongoDB connection established**
✅ **All API routes are active**

## 📋 Available API Endpoints

### Authentication Endpoints
- `POST /api/auth/signup` - Register new user
- `POST /api/auth/login` - Login user

### Todo CRUD Endpoints (Require Authentication)
- `GET /api/todos` - Get all todos for authenticated user
- `POST /api/todos` - Create new todo
- `GET /api/todos/{id}` - Get specific todo by ID
- `PUT /api/todos/{id}` - Update todo
- `DELETE /api/todos/{id}` - Delete todo

## 🔧 Testing Methods

### Method 1: Postman Collection (Recommended)

1. **Import the collection**:
   - Open Postman
   - Click "Import" button
   - Select `postman_collection.json` from the project root
   - The collection includes automated tests and environment variables

2. **Run the tests**:
   - Execute requests in this order:
     1. **Signup New User** (generates random email)
     2. **Login User** (uses the same credentials)
     3. **Create Todo** (requires auth token from login)
     4. **Get All Todos**
     5. **Get Todo by ID**
     6. **Update Todo**
     7. **Delete Todo**
   - Check the "Test Results" tab for validation

3. **Error testing**:
   - Run the "Error Test Cases" folder to test:
     - Unauthorized access
     - Invalid tokens
     - Duplicate registrations
     - Invalid credentials

### Method 2: Bash Script Testing

```bash
# Make script executable
chmod +x test_api.sh

# Run comprehensive API tests
./test_api.sh
```

The script will:
- Generate unique test user credentials
- Test all endpoints in sequence
- Show request/response details
- Test error scenarios

### Method 3: Manual curl Testing

#### 1. Register a new user
```bash
curl -X POST http://127.0.0.1:8000/api/auth/signup \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123",
    "name": "Test User"
  }'
```

#### 2. Login (save the token from response)
```bash
curl -X POST http://127.0.0.1:8000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "password123"
  }'
```

#### 3. Create a todo (replace YOUR_TOKEN)
```bash
curl -X POST http://127.0.0.1:8000/api/todos \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -d '{
    "title": "Learn Rust",
    "description": "Build a complete todo API"
  }'
```

#### 4. Get all todos
```bash
curl -X GET http://127.0.0.1:8000/api/todos \
  -H "Authorization: Bearer YOUR_TOKEN"
```

#### 5. Update todo (replace TODO_ID)
```bash
curl -X PUT http://127.0.0.1:8000/api/todos/TODO_ID \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -d '{
    "title": "Learn Rust - Updated",
    "completed": true
  }'
```

#### 6. Delete todo
```bash
curl -X DELETE http://127.0.0.1:8000/api/todos/TODO_ID \
  -H "Authorization: Bearer YOUR_TOKEN"
```

## 🔍 Expected Responses

### Successful Signup/Login Response
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "user": {
    "id": "507f1f77bcf86cd799439011",
    "email": "test@example.com",
    "name": "Test User",
    "created_at": "2024-01-01T00:00:00Z"
  }
}
```

### Todo Response
```json
{
  "id": "507f1f77bcf86cd799439012",
  "title": "Learn Rust",
  "description": "Build a complete todo API",
  "completed": false,
  "user_id": "507f1f77bcf86cd799439011",
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z"
}
```

### Error Response
```json
{
  "error": "Invalid credentials"
}
```

## 🛡️ Security Features Tested

- ✅ **JWT Authentication** - All todo endpoints require valid tokens
- ✅ **Password Hashing** - Passwords are hashed with bcrypt
- ✅ **User Isolation** - Users can only access their own todos
- ✅ **Input Validation** - Invalid requests return appropriate errors
- ✅ **Duplicate Prevention** - Cannot register with existing email

## 🐛 Troubleshooting

### Server not starting?
- Check if MongoDB is running
- Verify port 8000 is available
- Check `.env` file configuration

### Authentication errors?
- Ensure you're using the correct token format: `Bearer <token>`
- Check token hasn't expired (24 hours)
- Verify the JWT secret in `.env`

### Database errors?
- Confirm MongoDB connection string in `.env`
- Check database permissions
- Verify network connectivity

## 📊 Test Coverage

The testing suite covers:
- ✅ User registration and authentication
- ✅ JWT token generation and validation
- ✅ Complete CRUD operations for todos
- ✅ User data isolation
- ✅ Error handling and edge cases
- ✅ Input validation
- ✅ Security middleware

## 🎉 Success Criteria

All tests should pass with:
- HTTP 200 for successful operations
- HTTP 400 for bad requests
- HTTP 401 for unauthorized access
- Proper JSON responses
- Correct data validation
- Secure authentication flow
