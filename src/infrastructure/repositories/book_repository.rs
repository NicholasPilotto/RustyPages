use sea_orm::{DatabaseConnection, DbErr, EntityTrait};
use crate::domain::book::{Entity as BookEntity};
use crate::queries::models::book_view::BookView;

pub struct BookRepository<'a> {
    pub db: &'a DatabaseConnection,
}

impl<'a> BookRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_list(&self) -> Result<Vec<BookView>, DbErr> {
        let books = BookEntity::find()
            .all(self.db)
            .await
            .map_err(|e| e)?;

        let result: Vec<BookView> = books
            .into_iter()
            .map(BookView::from)
            .collect();

        Ok(result)
    }
}