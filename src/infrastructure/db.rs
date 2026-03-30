use sea_orm::{Database, DatabaseConnection};

pub async fn create_db_connection(database_url: &str) -> DatabaseConnection {
    Database::connect(database_url)
        .await
        .expect("Failed to connect to Postgres")
}
