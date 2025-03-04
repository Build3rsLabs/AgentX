use crate::models::user::AuthRequest;
use crate::services::ServiceContainer;
use std::sync::Arc;
use warp::{Filter, Rejection, Reply};

pub fn routes(
    services: Arc<ServiceContainer>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let auth_service = services.auth_service.clone();
    
    let register = warp::path!("auth" / "register")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |address: String| {
            let auth_service = auth_service.clone();
            async move {
                auth_service
                    .register_user(&address)
                    .await
                    .map(|user| warp::reply::json(&user))
                    .map_err(|e| warp::reject::custom(e))
            }
        });
    
    let login = warp::path!("auth" / "login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |auth_request: AuthRequest| {
            let auth_service = services.auth_service.clone();
            async move {
                auth_service
                    .authenticate(auth_request)
                    .await
                    .map(|auth_response| warp::reply::json(&auth_response))
                    .map_err(|e| warp::reject::custom(e))
            }
        });
    
    register.or(login)
}