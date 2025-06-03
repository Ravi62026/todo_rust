#!/bin/bash

echo "🚀 Starting Todo Rust API Server"
echo "================================="

# Check if MongoDB is running
echo "🔍 Checking MongoDB connection..."
if ! mongo --eval "db.runCommand('ping').ok" localhost:27017/test --quiet; then
    echo "❌ MongoDB is not running!"
    echo "💡 Please start MongoDB first:"
    echo "   - On Windows: net start MongoDB"
    echo "   - On macOS: brew services start mongodb-community"
    echo "   - On Linux: sudo systemctl start mongod"
    echo ""
    echo "🔗 Or use MongoDB Atlas (cloud) and update MONGODB_URI in .env"
    exit 1
fi

echo "✅ MongoDB is running!"

# Build the project
echo ""
echo "🔨 Building the project..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build successful!"

# Start the server
echo ""
echo "🚀 Starting the server..."
echo "📍 Server will be available at: http://127.0.0.1:8000"
echo "📋 API Documentation:"
echo "   - POST /api/auth/signup - Register new user"
echo "   - POST /api/auth/login  - Login user"
echo "   - GET    /api/todos     - Get all todos"
echo "   - POST   /api/todos     - Create todo"
echo "   - GET    /api/todos/:id - Get todo by ID"
echo "   - PUT    /api/todos/:id - Update todo"
echo "   - DELETE /api/todos/:id - Delete todo"
echo ""
echo "🛑 Press Ctrl+C to stop the server"
echo ""

cargo run --release
