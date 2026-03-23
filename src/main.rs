mod config;
mod domain;
mod routes;
mod infrastructure;

use axum::Router;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
use config::Config;
use routes::book_routes::{book_router, BookAppState};
use tokio::net::TcpListener;

use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};
use infrastructure::db::create_db_connection;

use crate::routes::book_routes::BookApi;

#[derive(OpenApi)]
#[openapi(
    info(title = "RustyPages", version = "1.0.0"),
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load .env file");

    let config = Config::from_env();

    let addr = format!("{}:{}", config.host, config.port);
    
    tracing_subscriber::registry()
        .with(EnvFilter::new("debug"))
        .with(fmt::layer())
        .init();

    let db = create_db_connection(&config.database_url).await;

    let book_state = BookAppState {
        db: db
    };

    let mut doc = ApiDoc::openapi();
    doc.merge(BookApi::openapi());

    let app = Router::new()
        .merge(book_router().with_state(book_state))
        .merge(Scalar::with_url("/scalar", doc))
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind(&addr).await.unwrap();
    
    println!("Listening on http://{}", addr);
    println!("Scalar docs at http://{}/scalar", addr);

    axum::serve(listener, app).await.unwrap();
}
