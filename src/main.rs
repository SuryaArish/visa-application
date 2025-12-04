use axum::{
    http::Method,
    routing::{get, post, put, patch},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

mod models;
mod handlers;
mod middleware;
mod config;

use handlers::*;
use middleware::auth::auth_middleware;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    run_local_server().await?;
    Ok(())
}

async fn run_local_server() -> Result<(), Box<dyn std::error::Error>> {
    // Database will be initialized on first use
    let _pool = std::env::var("DB_HOST"); // Just check env vars exist

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH, Method::OPTIONS])
        .allow_origin(Any)
        .allow_headers(vec![
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE,
        ]);
    let protected_routes = Router::new()
        .route("/h1b_customer/create", post(create_visa_details))
        .route("/customers", get(get_all_customers_with_status))
        .route("/get_customer_by_id/:id", get(get_customer_by_id))
        .route("/get_customer_by_email/:email", get(get_customer_by_email))
        .route("/soft_delete_customer_via_id/:id", patch(soft_delete_customer_by_id))
        .route("/update_customer_by_id/:id", put(update_customer_by_id))
        .route("/h1b_customer/by_login_email/:login_email", get(get_customer_by_login_email))
        .route("/h1b_customer/all", get(get_all_customers_no_filter))
        .route("/h1b_customer/activate/:customer_id", patch(activate_customer_by_id))
        .layer(axum::middleware::from_fn(auth_middleware));

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/hello", get(test_connection))
        .merge(protected_routes)
        .layer(cors);

    println!("Starting local server on http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}