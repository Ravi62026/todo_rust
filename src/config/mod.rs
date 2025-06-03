use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub mongodb_uri: String,
    pub database_name: String,
    pub jwt_secret: String,
    pub port: u16,
    pub address: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv::dotenv().ok();
        
        Ok(Config {
            mongodb_uri: env::var("MONGODB_URI")
                .unwrap_or_else(|_| "mongodb://localhost:27017".to_string()),
            database_name: env::var("DATABASE_NAME")
                .unwrap_or_else(|_| "todo_rust_db".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "default_secret_change_me".to_string()),
            port: env::var("ROCKET_PORT")
                .unwrap_or_else(|_| "8000".to_string())
                .parse()
                .unwrap_or(8000),
            address: env::var("ROCKET_ADDRESS")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
        })
    }
}
