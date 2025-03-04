use crate::error::AppError;
use crate::services::ServiceContainer;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use warp::{Filter, Rejection};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    user_id: String,
}

pub fn with_auth(
    services: Arc<ServiceContainer>,
) -> impl Filter<Extract = (Uuid,), Error = Rejection> + Clone {
    warp::header::<String>("authorization")
        .map(move |auth_header: String| (auth_header, services.clone()))
        .and_then(|(auth_header, services)| async move {
            if !auth_header.starts_with("Bearer ") {
                return Err(warp::reject::custom(AppError::Auth(
                    "Invalid authorization header format".to_string(),
                )));
            }

            let token = auth_header.trim_start_matches("Bearer ").trim();
            let jwt_secret = &services.config.api.jwt_secret;

            let token_data = decode::<Claims>(
                token,
                &DecodingKey::from_secret(jwt_secret.as_bytes()),
                &Validation::default(),
            )
            .map_err(|e| {
                warp::reject::custom(AppError::Auth(format!("Invalid token: {}", e)))
            })?;

            let user_id = Uuid::parse_str(&token_data.claims.user_id).map_err(|e| {
                warp::reject::custom(AppError::Auth(format!("Invalid user ID in token: {}", e)))
            })?;

            Ok(user_id)
        })
}