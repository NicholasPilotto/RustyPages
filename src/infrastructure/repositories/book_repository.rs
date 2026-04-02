use crate::domain::book::{ActiveModel, Entity as BookEntity};
use crate::infrastructure::errors::error::AppError;
use crate::queries::models::book_view::BookView;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{DatabaseConnection, EntityTrait, prelude::DateTimeUtc};
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

    pub async fn create(
        &self,
        title: String,
        description: Option<String>,
        author: String,
        isbn: Option<String>,
        published_at: Option<DateTimeUtc>,
        pages: Option<i32>,
        language: Option<String>,
        created_at: DateTimeUtc,
    ) -> Result<Uuid, AppError> {
        let id = Uuid::new_v4();
        let book = ActiveModel {
            id: Set(id),
            title: Set(title.to_owned()),
            description: Set(description.to_owned()),
            author: Set(author.to_owned()),
            isbn: Set(isbn.to_owned()),
            published_at: Set(published_at),
            pages: Set(pages),
            language: Set(language),
            created_at: Set(created_at),
            updated_at: NotSet,
        };

        book.insert(self.db).await?;

        Ok(id)
    }
}
