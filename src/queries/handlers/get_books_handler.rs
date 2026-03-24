
use sea_orm::DbErr;

use crate::{infrastructure::repositories::book_repository::BookRepository, queries::models::book_view::BookView};

pub async fn get_books_handler(
    repo: &BookRepository<'_>
) -> Result<Vec<BookView>, DbErr>{
    repo.get_list().await
}