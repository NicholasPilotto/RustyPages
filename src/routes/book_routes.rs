use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{
    Router, response::IntoResponse, routing::get
};

use sea_orm::DatabaseConnection;
use utoipa::OpenApi;

use crate::infrastructure::repositories::book_repository::BookRepository;
use crate::queries::handlers::get_books_handler::*;
use crate::queries::models::book_view::BookView;

#[derive(Clone)]
pub struct BookAppState {
    pub db: DatabaseConnection
}

#[derive(OpenApi)]
#[openapi(
    paths(get_books),
    components(schemas(BookView)),
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
        (status = 200, description = "List of books", body = [BookView]),
    ),
    description = "Get the list of books",
    tag = "Books"
)]
async fn get_books(
    State(state): State<BookAppState>,
) -> impl IntoResponse {
    let repo = BookRepository::new(&state.db);
    match get_books_handler(&repo).await {
        Ok(books) => Json(books).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}