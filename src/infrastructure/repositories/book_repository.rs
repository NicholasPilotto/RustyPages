use crate::domain::book::Entity as BookEntity;
use crate::infrastructure::errors::error::AppError;
use crate::queries::models::book_view::BookView;
use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;

pub struct BookRepository<'a> {
    pub db: &'a DatabaseConnection,
}

impl<'a> BookRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<BookView, AppError> {
        let book = BookEntity::find_by_id(id)
            .one(self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Book not found".to_string()))?;

        Ok(BookView::from(book))
    }

    pub async fn get_list(&self) -> Result<Vec<BookView>, String> {
        let books = BookEntity::find()
            .all(self.db)
            .await
            .map_err(|e| e.to_string())?;

        let result: Vec<BookView> = books.into_iter().map(BookView::from).collect();

        Ok(result)
    }
}
