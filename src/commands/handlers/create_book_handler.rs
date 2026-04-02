use sea_orm::sqlx::types::chrono::Utc;
use uuid::Uuid;

use crate::{
    commands::models::create_book_command::CreateBookCommand,
    infrastructure::{errors::error::AppError, repositories::book_repository::BookRepository},
};

pub async fn create_book_handler(
    cmd: CreateBookCommand,
    repo: &BookRepository<'_>,
) -> Result<Uuid, AppError> {
    let created_at = Utc::now();

    let id = repo
        .create(
            cmd.title,
            cmd.description,
            cmd.author,
            cmd.isbn,
            cmd.published_at,
            cmd.pages,
            cmd.language,
            created_at,
        )
        .await?;

    Ok(id)
}
