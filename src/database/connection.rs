use crate::config::Config;
use anyhow::Result;
use mongodb::{Client, Database};

pub struct DatabaseConnection {
    pub client: Client,
    pub database: Database,
}

impl DatabaseConnection {
    pub async fn new(config: &Config) -> Result<Self> {
        let client = Client::with_uri_str(&config.mongodb_uri).await?;
        let database = client.database(&config.database_name);

        // Test the connection
        database
            .run_command(mongodb::bson::doc! {"ping": 1})
            .await?;
        println!("Successfully connected to MongoDB!");

        Ok(DatabaseConnection { client, database })
    }

    pub fn get_database(&self) -> &Database {
        &self.database
    }
}
