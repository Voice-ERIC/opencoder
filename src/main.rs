mod config;
mod handlers;
mod models;
mod services;
mod gitea;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "qwen_coder_web=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Create CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router
    let app = Router::new()
        // Health check
        .route("/health", get(handlers::health_check))
        
        // API configuration endpoints
        .route("/api/config", get(handlers::get_config).post(handlers::save_config))
        
        // Chat/completion endpoint (OpenAI-compatible)
        .route("/api/chat/completions", post(handlers::chat_completions))
        
        // Task decomposition
        .route("/api/tasks/decompose", post(handlers::decompose_task))
        
        // Multi-prompt management
        .route("/api/prompts", get(handlers::list_prompts).post(handlers::create_prompt))
        .route("/api/prompts/:id", get(handlers::get_prompt).put(handlers::update_prompt).delete(handlers::delete_prompt))
        
        // Gitea integration
        .route("/api/gitea/repos", get(gitea::handlers::list_repos))
        .route("/api/gitea/repos/:owner/:repo/issues", get(gitea::handlers::list_issues).post(gitea::handlers::create_issue))
        .route("/api/gitea/repos/:owner/:repo/pulls", get(gitea::handlers::list_pulls))
        .route("/api/gitea/config", get(gitea::handlers::get_config).post(gitea::handlers::save_config))
        
        // Serve static files (frontend)
        .nest_service("/", tower_http::services::ServeDir::new("static").append_index_html_on_directories(true))
        .layer(cors);

    // Get port from environment or default to 3000
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
