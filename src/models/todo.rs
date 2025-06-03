use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub user_id: ObjectId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct TodoResponse {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Todo> for TodoResponse {
    fn from(todo: Todo) -> Self {
        TodoResponse {
            id: todo.id.unwrap().to_hex(),
            title: todo.title,
            description: todo.description,
            completed: todo.completed,
            user_id: todo.user_id.to_hex(),
            created_at: todo.created_at,
            updated_at: todo.updated_at,
        }
    }
}
