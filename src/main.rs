mod config;
mod domain;
mod routes;

use axum::Router;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
use config::Config;
use routes::book_routes::{book_router, BookAppState};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load .env file");

    let config = Config::from_env();

    let addr = format!("{}:{}", config.host, config.port);
    
    tracing_subscriber::registry()
        .with(EnvFilter::new("debug"))
        .with(fmt::layer())
        .init();

    let book_state = BookAppState {};

    let app = Router::new()
        .merge(book_router().with_state(book_state))
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind(&addr).await.unwrap();
    
    println!("Listening on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
