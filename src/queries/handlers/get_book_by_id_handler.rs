use uuid::Uuid;

use crate::{
    infrastructure::{errors::error::AppError, repositories::book_repository::BookRepository},
    queries::models::book_view::BookView,
};

pub async fn get_book_by_id_handler(
    id: Uuid,
    repo: &BookRepository<'_>,
) -> Result<BookView, AppError> {
    let view = repo.get_by_id(id).await?;

    Ok(view)
}
