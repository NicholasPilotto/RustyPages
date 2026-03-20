use axum::Router;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use std::env;

struct Config {
    host: String,
    port: u16,
}

impl Config {
    fn from_env() -> Self {
        Config {
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("PORT must be a valid number"),
        }
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load .env file");

    let config = Config::from_env();

    let addr = format!("{}:{}", config.host, config.port);
    
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("debug"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    
    println!("Listening on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
