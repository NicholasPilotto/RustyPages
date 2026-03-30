use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, ToSchema, Validate)]
pub struct CreateBookCommand {
    #[validate(length(min = 2, message = "Title must be at least 2 characters"))]
    pub title: String,

    #[validate(length(min = 2, message = "Description must be at least 2 characters"))]
    pub description: String,

    #[validate(length(min = 2, message = "Author must be at least 2 characters"))]
    pub author: String,

    #[validate(length(min = 2, message = "ISBN must be at least 2 characters"))]
    pub isbn: Option<String>,

    #[validate(range(
        min = 1,
        max = 1_000_000,
        message = "Pages must be at least one and less than 1000000"
    ))]
    pub pages: Option<i32>,

    #[validate(length(min = 2, message = "Language must be at least 2 characters"))]
    pub language: Option<String>,
}
