use crate::{
    infrastructure::{errors::error::AppError, repositories::book_repository::BookRepository},
    queries::models::book_view::BookView,
};

pub async fn get_books_handler(repo: &BookRepository<'_>) -> Result<Vec<BookView>, AppError> {
    repo.get_list()
        .await
        .map_err(|_| AppError::InternalServerError("Something went wrong".to_string()))
}
