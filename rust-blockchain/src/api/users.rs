use crate::api::middleware::with_auth;
use crate::services::ServiceContainer;
use std::sync::Arc;
use warp::{Filter, Rejection, Reply};

pub fn routes(
    services: Arc<ServiceContainer>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let user_service = services.user_service.clone();
    
    let get_me = warp::path!("users" / "me")
        .and(warp::get())
        .and(with_auth(services.clone()))
        .and_then(move |user_id| {
            let user_service = user_service.clone();
            async move {
                user_service
                    .get_user_by_id(user_id)
                    .await
                    .map(|user| warp::reply::json(&user))
                    .map_err(|e| warp::reject::custom(e))
            }
        });
    
    warp::path("users").and(get_me)
}