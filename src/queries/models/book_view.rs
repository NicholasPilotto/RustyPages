use sea_orm::prelude::DateTimeUtc;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct BookView {
    /// The unique book identifier.
    pub id: Uuid,

    /// The book title.
    pub title: String,
    
    /// The book description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The book author name.
    pub author: String,
    
    /// The book isbn code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isbn: Option<String>,

    /// The book publish date.
    #[schema(value_type = String, format = "date-time", nullable = true)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<DateTimeUtc>,
    
    /// The number of pages of the book.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<i32>,
    
    /// The book language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    
    #[schema(value_type = String, format = "date-time")]
    pub created_at: DateTimeUtc,
    
    #[schema(value_type = String, format = "date-time")]
    pub updated_at: DateTimeUtc,
}