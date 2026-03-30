use axum::Json;
use axum::extract::{Path, State};
use axum::routing::post;
use axum::{Router, response::IntoResponse, routing::get};

use sea_orm::DatabaseConnection;
use utoipa::OpenApi;
use uuid::Uuid;

use crate::commands::handlers::create_book_handler::*;
use crate::commands::models::create_book_command::CreateBookCommand;
use crate::infrastructure::errors::error::{AppError, ErrorResponse};
use crate::infrastructure::extractors::extractor::ValidatedJson;
use crate::infrastructure::repositories::book_repository::BookRepository;
use crate::queries::handlers::get_book_by_id_handler::*;
use crate::queries::handlers::get_books_handler::*;
use crate::queries::models::book_view::BookView;

#[derive(Clone)]
pub struct BookAppState {
    pub db: DatabaseConnection,
}

#[derive(OpenApi)]
#[openapi(
    paths(get_books, get_book_by_id, create_book),
    components(schemas(BookView, ErrorResponse)),
    tags(
        (name = "Books", description = "Books management API")
    )
)]
pub struct BookApi;

pub fn book_router() -> Router<BookAppState> {
    return Router::new()
        .route("/book", get(get_books))
        .route("/book/{id}", get(get_book_by_id))
        .route("/book", post(create_book));
}

#[utoipa::path(
    get,
    path = "/book",
    responses(
        (status = 200, description = "List of books", body = [BookView]),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    description = "Get the list of books",
    tag = "Books"
)]
async fn get_books(State(state): State<BookAppState>) -> Result<impl IntoResponse, AppError> {
    let repo = BookRepository::new(&state.db);
    let view = get_books_handler(&repo).await?;

    Ok(Json(view))
}

#[utoipa::path(
    get,
    path = "/book/{id}",
    tag = "Books",
    params(("id" = Uuid, Path, description = "Book ID")),
    description = "Get the detail of an existing book",
    responses(
        (status = 200, description = "Book found", body = BookView),
        (status = 404, description = "Book not found", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    )
)]
async fn get_book_by_id(
    State(state): State<BookAppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let repo = BookRepository::new(&state.db);
    let view = get_book_by_id_handler(id, &repo).await?;

    Ok(Json(view))
}

#[utoipa::path(
    post,
    path = "/book",
    tag = "Books",
    request_body = CreateBookCommand,
    responses(
        (status = 201, description = "User created", body = String),
        (status = 400, description = "Bad request", body = ErrorResponse),
    )
)]
async fn create_book(
    State(state): State<BookAppState>,
    ValidatedJson(cmd): ValidatedJson<CreateBookCommand>,
) -> Result<impl IntoResponse, AppError> {
    let repo = BookRepository::new(&state.db);
    let view = create_book_handler(cmd, &repo).await?;

    Ok(Json(view))
}
