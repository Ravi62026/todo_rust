use chrono::Utc;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};
use rocket::{delete, get, post, put, serde::json::Json, State};

use crate::{
    database::connection::DatabaseConnection,
    handlers::auth::ErrorResponse,
    middleware::auth::AuthenticatedUser,
    models::todo::{CreateTodoRequest, Todo, TodoResponse, UpdateTodoRequest},
};

#[post("/todos", data = "<request>")]
pub async fn create_todo(
    request: Json<CreateTodoRequest>,
    user: AuthenticatedUser,
    db: &State<DatabaseConnection>,
) -> Result<Json<TodoResponse>, rocket::response::status::BadRequest<Json<ErrorResponse>>> {
    let collection: Collection<Todo> = db.database.collection("todos");

    let user_id = match ObjectId::parse_str(&user.user_id) {
        Ok(id) => id,
        Err(_) => {
            return Err(rocket::response::status::BadRequest(Json(ErrorResponse {
                error: "Invalid user ID".to_string(),
            })));
        }
    };

    let now = Utc::now();
    let todo = Todo {
        id: None,
        title: request.title.clone(),
        description: request.description.clone(),
        completed: false,
        user_id,
        created_at: now,
        updated_at: now,
    };

    match collection.insert_one(&todo).await {
        Ok(result) => {
            let todo_id = result.inserted_id.as_object_id().unwrap().to_hex();
            let todo_response = TodoResponse {
                id: todo_id,
                title: request.title.clone(),
                description: request.description.clone(),
                completed: false,
                user_id: user.user_id,
                created_at: now,
                updated_at: now,
            };
            Ok(Json(todo_response))
        }
        Err(_) => Err(rocket::response::status::BadRequest(Json(ErrorResponse {
            error: "Failed to create todo".to_string(),
        }))),
    }
}

#[get("/todos")]
pub async fn get_todos(
    user: AuthenticatedUser,
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<TodoResponse>>, rocket::response::status::BadRequest<Json<ErrorResponse>>> {
    let collection: Collection<Todo> = db.database.collection("todos");

    let user_id = match ObjectId::parse_str(&user.user_id) {
        Ok(id) => id,
        Err(_) => {
            return Err(rocket::response::status::BadRequest(Json(ErrorResponse {
                error: "Invalid user ID".to_string(),
            })));
        }
    };

    match collection.find(doc! {"user_id": user_id}).await {
        Ok(mut cursor) => {
            let mut todos = Vec::new();
            while cursor.advance().await.unwrap_or(false) {
                if let Ok(todo) = cursor.deserialize_current() {
                    todos.push(TodoResponse::from(todo));
                }
            }
            Ok(Json(todos))
        }
        Err(_) => Err(rocket::response::status::BadRequest(Json(ErrorResponse {
            error: "Failed to fetch todos".to_string(),
        }))),
    }
}

#[get("/todos/<id>")]
pub async fn get_todo(
    id: String,
    user: AuthenticatedUser,
    db: &State<DatabaseConnection>,
) -> Result<Json<TodoResponse>, rocket::response::status::BadRequest<Json<ErrorResponse>>> {
    let collection: Collection<Todo> = db.database.collection("todos");

    let todo_id = match ObjectId::parse_str(&id) {
        Ok(id) => id,
        Err(_) => {
            return Err(rocket::response::status::BadRequest(Json(ErrorResponse {
                error: "Invalid todo ID".to_string(),
            })));
        }
    };

    let user_id = match ObjectId::parse_str(&user.user_id) {
        Ok(id) => id,
        Err(_) => {
            return Err(rocket::response::status::BadRequest(Json(ErrorResponse {
                error: "Invalid user ID".to_string(),
            })));
        }
    };

    match collection
        .find_one(doc! {"_id": todo_id, "user_id": user_id})
        .await
    {
        Ok(Some(todo)) => Ok(Json(TodoResponse::from(todo))),
        Ok(None) => Err(rocket::response::status::BadRequest(Json(ErrorResponse {
            error: "Todo not found".to_string(),
        }))),
        Err(_) => Err(rocket::response::status::BadRequest(Json(ErrorResponse {
            error: "Failed to fetch todo".to_string(),
        }))),
    }
}

#[put("/todos/<id>", data = "<request>")]
pub async fn update_todo(
    id: String,
    request: Json<UpdateTodoRequest>,
    user: AuthenticatedUser,
    db: &State<DatabaseConnection>,
) -> Result<Json<TodoResponse>, rocket::response::status::BadRequest<Json<ErrorResponse>>> {
    let collection: Collection<Todo> = db.database.collection("todos");

    let todo_id = match ObjectId::parse_str(&id) {
        Ok(id) => id,
        Err(_) => {
            return Err(rocket::response::status::BadRequest(Json(ErrorResponse {
                error: "Invalid todo ID".to_string(),
            })));
        }
    };

    let user_id = match ObjectId::parse_str(&user.user_id) {
        Ok(id) => id,
        Err(_) => {
            return Err(rocket::response::status::BadRequest(Json(ErrorResponse {
                error: "Invalid user ID".to_string(),
            })));
        }
    };

    // Build update document
    let mut update_doc = doc! {"updated_at": mongodb::bson::DateTime::now()};

    if let Some(title) = &request.title {
        update_doc.insert("title", title);
    }
    if let Some(description) = &request.description {
        update_doc.insert("description", description);
    }
    if let Some(completed) = request.completed {
        update_doc.insert("completed", completed);
    }

    match collection
        .update_one(
            doc! {"_id": todo_id, "user_id": user_id},
            doc! {"$set": update_doc},
        )
        .await
    {
        Ok(result) => {
            if result.matched_count == 0 {
                return Err(rocket::response::status::BadRequest(Json(ErrorResponse {
                    error: "Todo not found".to_string(),
                })));
            }

            // Fetch updated todo
            match collection
                .find_one(doc! {"_id": todo_id, "user_id": user_id})
                .await
            {
                Ok(Some(todo)) => Ok(Json(TodoResponse::from(todo))),
                Ok(None) => Err(rocket::response::status::BadRequest(Json(ErrorResponse {
                    error: "Todo not found after update".to_string(),
                }))),
                Err(_) => Err(rocket::response::status::BadRequest(Json(ErrorResponse {
                    error: "Failed to fetch updated todo".to_string(),
                }))),
            }
        }
        Err(_) => Err(rocket::response::status::BadRequest(Json(ErrorResponse {
            error: "Failed to update todo".to_string(),
        }))),
    }
}

#[delete("/todos/<id>")]
pub async fn delete_todo(
    id: String,
    user: AuthenticatedUser,
    db: &State<DatabaseConnection>,
) -> Result<Json<serde_json::Value>, rocket::response::status::BadRequest<Json<ErrorResponse>>> {
    let collection: Collection<Todo> = db.database.collection("todos");

    let todo_id = match ObjectId::parse_str(&id) {
        Ok(id) => id,
        Err(_) => {
            return Err(rocket::response::status::BadRequest(Json(ErrorResponse {
                error: "Invalid todo ID".to_string(),
            })));
        }
    };

    let user_id = match ObjectId::parse_str(&user.user_id) {
        Ok(id) => id,
        Err(_) => {
            return Err(rocket::response::status::BadRequest(Json(ErrorResponse {
                error: "Invalid user ID".to_string(),
            })));
        }
    };

    match collection
        .delete_one(doc! {"_id": todo_id, "user_id": user_id})
        .await
    {
        Ok(result) => {
            if result.deleted_count == 0 {
                return Err(rocket::response::status::BadRequest(Json(ErrorResponse {
                    error: "Todo not found".to_string(),
                })));
            }
            Ok(Json(
                serde_json::json!({"message": "Todo deleted successfully"}),
            ))
        }
        Err(_) => Err(rocket::response::status::BadRequest(Json(ErrorResponse {
            error: "Failed to delete todo".to_string(),
        }))),
    }
}
