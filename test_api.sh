#!/bin/bash

# Test script for Todo Rust API
# This script tests all the API endpoints using curl

BASE_URL="http://127.0.0.1:8000"
EMAIL="test$(date +%s)@example.com"
PASSWORD="password123"
NAME="Test User"

echo "🚀 Testing Todo Rust API"
echo "========================="
echo "Base URL: $BASE_URL"
echo "Test Email: $EMAIL"
echo ""

# Function to make HTTP requests and show results
make_request() {
    local method=$1
    local url=$2
    local data=$3
    local headers=$4
    
    echo "📡 $method $url"
    if [ -n "$data" ]; then
        echo "📤 Request Body: $data"
    fi
    
    if [ -n "$headers" ]; then
        response=$(curl -s -w "\n%{http_code}" -X "$method" "$url" -H "Content-Type: application/json" $headers -d "$data")
    else
        response=$(curl -s -w "\n%{http_code}" -X "$method" "$url" -H "Content-Type: application/json" -d "$data")
    fi
    
    http_code=$(echo "$response" | tail -n1)
    body=$(echo "$response" | head -n -1)
    
    echo "📥 Response Code: $http_code"
    echo "📥 Response Body: $body"
    echo ""
    
    # Return the response body for further processing
    echo "$body"
}

echo "1️⃣  Testing User Signup"
echo "----------------------"
signup_data="{\"email\":\"$EMAIL\",\"password\":\"$PASSWORD\",\"name\":\"$NAME\"}"
signup_response=$(make_request "POST" "$BASE_URL/api/auth/signup" "$signup_data")

# Extract token from signup response
TOKEN=$(echo "$signup_response" | grep -o '"token":"[^"]*' | cut -d'"' -f4)
USER_ID=$(echo "$signup_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4)

if [ -n "$TOKEN" ]; then
    echo "✅ Signup successful! Token: ${TOKEN:0:20}..."
    echo "✅ User ID: $USER_ID"
else
    echo "❌ Signup failed!"
    exit 1
fi

echo ""
echo "2️⃣  Testing User Login"
echo "--------------------"
login_data="{\"email\":\"$EMAIL\",\"password\":\"$PASSWORD\"}"
login_response=$(make_request "POST" "$BASE_URL/api/auth/login" "$login_data")

echo ""
echo "3️⃣  Testing Create Todo"
echo "----------------------"
todo_data="{\"title\":\"Learn Rust Programming\",\"description\":\"Build a complete todo API with authentication\"}"
todo_response=$(make_request "POST" "$BASE_URL/api/todos" "$todo_data" "-H \"Authorization: Bearer $TOKEN\"")

# Extract todo ID
TODO_ID=$(echo "$todo_response" | grep -o '"id":"[^"]*' | cut -d'"' -f4)

if [ -n "$TODO_ID" ]; then
    echo "✅ Todo created! ID: $TODO_ID"
else
    echo "❌ Todo creation failed!"
fi

echo ""
echo "4️⃣  Testing Get All Todos"
echo "------------------------"
make_request "GET" "$BASE_URL/api/todos" "" "-H \"Authorization: Bearer $TOKEN\""

echo ""
echo "5️⃣  Testing Get Todo by ID"
echo "-------------------------"
if [ -n "$TODO_ID" ]; then
    make_request "GET" "$BASE_URL/api/todos/$TODO_ID" "" "-H \"Authorization: Bearer $TOKEN\""
else
    echo "❌ Skipping - no todo ID available"
fi

echo ""
echo "6️⃣  Testing Update Todo"
echo "----------------------"
if [ -n "$TODO_ID" ]; then
    update_data="{\"title\":\"Learn Rust Programming - UPDATED\",\"description\":\"UPDATED: Build a complete todo API\",\"completed\":true}"
    make_request "PUT" "$BASE_URL/api/todos/$TODO_ID" "$update_data" "-H \"Authorization: Bearer $TOKEN\""
else
    echo "❌ Skipping - no todo ID available"
fi

echo ""
echo "7️⃣  Testing Delete Todo"
echo "----------------------"
if [ -n "$TODO_ID" ]; then
    make_request "DELETE" "$BASE_URL/api/todos/$TODO_ID" "" "-H \"Authorization: Bearer $TOKEN\""
else
    echo "❌ Skipping - no todo ID available"
fi

echo ""
echo "8️⃣  Testing Error Cases"
echo "----------------------"

echo "🔒 Testing unauthorized access (no token):"
make_request "GET" "$BASE_URL/api/todos" ""

echo "🔒 Testing invalid token:"
make_request "GET" "$BASE_URL/api/todos" "" "-H \"Authorization: Bearer invalid_token\""

echo "🔒 Testing duplicate user registration:"
make_request "POST" "$BASE_URL/api/auth/signup" "$signup_data"

echo "🔒 Testing invalid login credentials:"
invalid_login="{\"email\":\"$EMAIL\",\"password\":\"wrong_password\"}"
make_request "POST" "$BASE_URL/api/auth/login" "$invalid_login"

echo ""
echo "🎉 API Testing Complete!"
echo "========================"
echo "✅ All tests executed"
echo "📋 Check the responses above for any errors"
echo "🔧 Import the postman_collection.json file into Postman for GUI testing"
