use anyhow::Result;
use chrono::Utc;
use mongodb::{bson::doc, Collection};
use rocket::{post, serde::json::Json, State};

use crate::{
    config::Config,
    database::connection::DatabaseConnection,
    models::user::{AuthResponse, CreateUserRequest, LoginRequest, User, UserResponse},
    utils::{
        jwt::create_jwt,
        password::{hash_password, verify_password},
    },
};

#[derive(rocket::serde::Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[post("/signup", data = "<request>")]
pub async fn signup(
    request: Json<CreateUserRequest>,
    db: &State<DatabaseConnection>,
    config: &State<Config>,
) -> Result<Json<AuthResponse>, rocket::response::status::BadRequest<Json<ErrorResponse>>> {
    let collection: Collection<User> = db.database.collection("users");

    // Check if user already exists
    let existing_user = collection.find_one(doc! {"email": &request.email}).await;

    match existing_user {
        Ok(Some(_)) => {
            return Err(rocket::response::status::BadRequest(Json(ErrorResponse {
                error: "User with this email already exists".to_string(),
            })));
        }
        Ok(None) => {
            // User doesn't exist, proceed with creation
        }
        Err(_) => {
            return Err(rocket::response::status::BadRequest(Json(ErrorResponse {
                error: "Database error".to_string(),
            })));
        }
    }

    // Hash password
    let password_hash = match hash_password(&request.password) {
        Ok(hash) => hash,
        Err(_) => {
            return Err(rocket::response::status::BadRequest(Json(ErrorResponse {
                error: "Failed to hash password".to_string(),
            })));
        }
    };

    let now = Utc::now();
    let user = User {
        id: None,
        email: request.email.clone(),
        password_hash,
        name: request.name.clone(),
        created_at: now,
        updated_at: now,
    };

    // Insert user
    let insert_result = collection.insert_one(&user).await;

    match insert_result {
        Ok(result) => {
            let user_id = result.inserted_id.as_object_id().unwrap().to_hex();

            // Create JWT token
            let token = match create_jwt(user_id.clone(), request.email.clone(), &config.jwt_secret)
            {
                Ok(token) => token,
                Err(_) => {
                    return Err(rocket::response::status::BadRequest(Json(ErrorResponse {
                        error: "Failed to create token".to_string(),
                    })));
                }
            };

            let user_response = UserResponse {
                id: user_id,
                email: request.email.clone(),
                name: request.name.clone(),
                created_at: now,
            };

            Ok(Json(AuthResponse {
                token,
                user: user_response,
            }))
        }
        Err(_) => Err(rocket::response::status::BadRequest(Json(ErrorResponse {
            error: "Failed to create user".to_string(),
        }))),
    }
}

#[post("/login", data = "<request>")]
pub async fn login(
    request: Json<LoginRequest>,
    db: &State<DatabaseConnection>,
    config: &State<Config>,
) -> Result<Json<AuthResponse>, rocket::response::status::BadRequest<Json<ErrorResponse>>> {
    let collection: Collection<User> = db.database.collection("users");

    // Find user by email
    let user = collection.find_one(doc! {"email": &request.email}).await;

    match user {
        Ok(Some(user)) => {
            // Verify password
            match verify_password(&request.password, &user.password_hash) {
                Ok(true) => {
                    let user_id = user.id.unwrap().to_hex();

                    // Create JWT token
                    let token =
                        match create_jwt(user_id.clone(), user.email.clone(), &config.jwt_secret) {
                            Ok(token) => token,
                            Err(_) => {
                                return Err(rocket::response::status::BadRequest(Json(
                                    ErrorResponse {
                                        error: "Failed to create token".to_string(),
                                    },
                                )));
                            }
                        };

                    let user_response = UserResponse {
                        id: user_id,
                        email: user.email,
                        name: user.name,
                        created_at: user.created_at,
                    };

                    Ok(Json(AuthResponse {
                        token,
                        user: user_response,
                    }))
                }
                Ok(false) => Err(rocket::response::status::BadRequest(Json(ErrorResponse {
                    error: "Invalid credentials".to_string(),
                }))),
                Err(_) => Err(rocket::response::status::BadRequest(Json(ErrorResponse {
                    error: "Password verification failed".to_string(),
                }))),
            }
        }
        Ok(None) => Err(rocket::response::status::BadRequest(Json(ErrorResponse {
            error: "Invalid credentials".to_string(),
        }))),
        Err(_) => Err(rocket::response::status::BadRequest(Json(ErrorResponse {
            error: "Database error".to_string(),
        }))),
    }
}
