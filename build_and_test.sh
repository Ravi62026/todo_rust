#!/bin/bash

echo "🔧 Todo Rust API - Build and Test"
echo "=================================="

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo is not installed!"
    echo "💡 Install Rust from: https://rustup.rs/"
    exit 1
fi

echo "✅ Rust/Cargo found"

# Check if MongoDB is available
echo ""
echo "🔍 Checking MongoDB..."
if command -v mongo &> /dev/null; then
    echo "✅ MongoDB CLI found"
    if mongo --eval "db.runCommand('ping').ok" localhost:27017/test --quiet 2>/dev/null; then
        echo "✅ MongoDB is running and accessible"
    else
        echo "⚠️  MongoDB CLI found but server not running"
        echo "💡 Start MongoDB or use MongoDB Atlas"
    fi
else
    echo "⚠️  MongoDB CLI not found"
    echo "💡 Install MongoDB or use MongoDB Atlas"
fi

# Build the project
echo ""
echo "🔨 Building the project..."
cargo check

if [ $? -ne 0 ]; then
    echo "❌ Build check failed!"
    exit 1
fi

echo "✅ Build check successful!"

# Run cargo build
echo ""
echo "🔨 Compiling the project..."
cargo build

if [ $? -ne 0 ]; then
    echo "❌ Compilation failed!"
    exit 1
fi

echo "✅ Compilation successful!"

# Show next steps
echo ""
echo "🎉 Build Complete!"
echo "=================="
echo ""
echo "📋 Next Steps:"
echo "1. 🗄️  Start MongoDB (if not already running)"
echo "2. 🚀 Run the server: ./start_server.sh or cargo run"
echo "3. 🧪 Test the API: ./test_api.sh"
echo "4. 📮 Import postman_collection.json into Postman for GUI testing"
echo ""
echo "📁 Project Structure:"
echo "├── src/"
echo "│   ├── main.rs           # Application entry point"
echo "│   ├── config/           # Configuration management"
echo "│   ├── database/         # MongoDB connection"
echo "│   ├── models/           # Data models (User, Todo)"
echo "│   ├── handlers/         # API route handlers"
echo "│   ├── middleware/       # Authentication middleware"
echo "│   └── utils/            # Utilities (JWT, password hashing)"
echo "├── .env                  # Environment configuration"
echo "├── README.md             # Documentation"
echo "├── postman_collection.json # Postman test collection"
echo "└── test_api.sh          # Bash test script"
echo ""
echo "🔗 API Endpoints:"
echo "Authentication:"
echo "  POST /api/auth/signup   - Register new user"
echo "  POST /api/auth/login    - Login user"
echo ""
echo "Todos (require authentication):"
echo "  GET    /api/todos       - Get all todos"
echo "  POST   /api/todos       - Create new todo"
echo "  GET    /api/todos/:id   - Get specific todo"
echo "  PUT    /api/todos/:id   - Update todo"
echo "  DELETE /api/todos/:id   - Delete todo"
