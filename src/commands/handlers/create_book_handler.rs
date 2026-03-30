use sea_orm::{sqlx::types::chrono::{Utc}};
use uuid::Uuid;

use crate::{commands::models::create_book_command::CreateBookCommand, infrastructure::{errors::error::AppError, repositories::book_repository::BookRepository}, queries::models::book_view::BookView};

pub async fn create_book_handler(
    cmd: CreateBookCommand,
    repo: &BookRepository<'_>
) -> Result<BookView, AppError> {
    Ok(BookView {
        id: Uuid::new_v4(),
        title: "a".to_string(),
        description: Some("a".to_string()),
        author: "a".to_string(),
        isbn: Some("a".to_string()),
        published_at: None,
        pages: Some(0),
        language: Some("a".to_string()),
        created_at: Utc::now(),
        updated_at: Utc::now()
    })
}