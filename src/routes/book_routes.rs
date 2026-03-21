use axum::{
    Router, routing::{get}
};

use uuid::Uuid;

use crate::domain::book::Book;

#[derive(Clone)]
pub struct BookAppState {}


pub fn book_router() -> Router<BookAppState> {
    return Router::new()
        .route("/book", get(get_books));
}

async fn get_books() -> axum::Json<Vec<Book>>{
     axum::Json(vec![
        Book { id: Uuid::new_v4() },
        Book { id: Uuid::new_v4() },
        Book { id: Uuid::new_v4() },
    ])
}