use axum::{
    Router, routing::{get}
};

use sea_orm::DatabaseConnection;
use utoipa::OpenApi;
use uuid::Uuid;

use crate::domain::book::Book;

#[derive(Clone)]
pub struct BookAppState {
    pub db: DatabaseConnection
}

#[derive(OpenApi)]
#[openapi(
    paths(get_books),
    components(schemas(Book)),
    tags(
        (name = "Books", description = "Books management API")
    )
)]
pub struct BookApi;

pub fn book_router() -> Router<BookAppState> {
    return Router::new()
        .route("/book", get(get_books));
}

#[utoipa::path(
    get,
    path = "/book",
    responses(
        (status = 200, description = "List of books", body = [Book]),
    ),
    description = "Get the list of books",
    tag = "Books"
)]
async fn get_books() -> axum::Json<Vec<Book>>{
     axum::Json(vec![
        Book { id: Uuid::new_v4() },
        Book { id: Uuid::new_v4() },
        Book { id: Uuid::new_v4() },
    ])
}