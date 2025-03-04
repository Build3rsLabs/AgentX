mod auth;
mod middleware;
mod positions;
mod protocols;
mod transactions;
mod users;

use crate::error::AppError;
use crate::services::ServiceContainer;
use std::convert::Infallible;
use std::sync::Arc;
use warp::{Filter, Rejection, Reply};

// Helper function to convert our errors to Warp rejections
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message) = if let Some(e) = err.find::<AppError>() {
        match e {
            AppError::NotFound(_) => (warp::http::StatusCode::NOT_FOUND, e.to_string()),
            AppError::Validation(_) => (warp::http::StatusCode::BAD_REQUEST, e.to_string()),
            AppError::Auth(_) => (warp::http::StatusCode::UNAUTHORIZED, e.to_string()),
            _ => (
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
        }
    } else if err.is_not_found() {
        (
            warp::http::StatusCode::NOT_FOUND,
            "Not Found".to_string(),
        )
    } else {
        (
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    let json = warp::reply::json(&serde_json::json!({
        "error": message
    }));

    Ok(warp::reply::with_status(json, code))
}

pub async fn start_server(services: Arc<ServiceContainer>, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    // CORS configuration
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allow_headers(vec!["Content-Type", "Authorization"]);

    // Health check route
    let health_route = warp::path("health")
        .and(warp::get())
        .map(|| warp::reply::json(&serde_json::json!({"status": "ok"})));

    // API routes
    let api_routes = warp::path("api")
        .and(
            auth::routes(services.clone())
                .or(users::routes(services.clone()))
                .or(protocols::routes(services.clone()))
                .or(positions::routes(services.clone()))
                .or(transactions::routes(services.clone()))
        );

    // Combine all routes
    let routes = health_route
        .or(api_routes)
        .with(cors)
        .recover(handle_rejection);

    // Start the server
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;

    Ok(())
}