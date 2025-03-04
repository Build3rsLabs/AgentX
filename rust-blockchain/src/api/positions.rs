use crate::api::middleware::with_auth;
use crate::models::position::{CreatePositionRequest, UpdatePositionRequest};
use crate::services::ServiceContainer;
use std::sync::Arc;
use uuid::Uuid;
use warp::{Filter, Rejection, Reply};

pub fn routes(
    services: Arc<ServiceContainer>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let position_service = services.position_service.clone();
    
    let create_position = warp::path!("positions")
        .and(warp::post())
        .and(with_auth(services.clone()))
        .and(warp::body::json())
        .and_then(move |user_id, request: CreatePositionRequest| {
            let position_service = position_service.clone();
            async move {
                position_service
                    .create_position(user_id, request)
                    .await
                    .map(|position| warp::reply::json(&position))
                    .map_err(|e| warp::reject::custom(e))
            }
        });
    
    let get_positions = warp::path!("positions")
        .and(warp::get())
        .and(with_auth(services.clone()))
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .and_then(move |user_id, params: std::collections::HashMap<String, String>| {
            let position_service = position_service.clone();
            let protocol_id = params.get("protocol_id").cloned();
            async move {
                position_service
                    .get_positions_by_user(user_id, protocol_id)
                    .await
                    .map(|positions| warp::reply::json(&positions))
                    .map_err(|e| warp::reject::custom(e))
            }
        });
    
    let get_position = warp::path!("positions" / Uuid)
        .and(warp::get())
        .and(with_auth(services.clone()))
        .and_then(move |id, user_id| {
            let position_service = position_service.clone();
            async move {
                position_service
                    .get_position_by_id(id, user_id)
                    .await
                    .map(|position| warp::reply::json(&position))
                    .map_err(|e| warp::reject::custom(e))
            }
        });
    
    let update_position = warp::path!("positions" / Uuid)
        .and(warp::put())
        .and(with_auth(services.clone()))
        .and(warp::body::json())
        .and_then(move |id, user_id, request: UpdatePositionRequest| {
            let position_service = position_service.clone();
            async move {
                position_service
                    .update_position(id, user_id, request)
                    .await
                    .map(|position| warp::reply::json(&position))
                    .map_err(|e| warp::reject::custom(e))
            }
        });
    
    let delete_position = warp::path!("positions" / Uuid)
        .and(warp::delete())
        .and(with_auth(services.clone()))
        .and_then(move |id, user_id| {
            let position_service = position_service.clone();
            async move {
                position_service
                    .delete_position(id, user_id)
                    .await
                    .map(|_| warp::reply::json(&serde_json::json!({"success": true})))
                    .map_err(|e| warp::reject::custom(e))
            }
        });
    
    let rebalance_position = warp::path!("positions" / Uuid / "rebalance")
        .and(warp::post())
        .and(with_auth(services.clone()))
        .and_then(move |id, user_id| {
            let position_service = position_service.clone();
            async move {
                position_service
                    .rebalance_position(id, user_id)
                    .await
                    .map(|position| warp::reply::json(&position))
                    .map_err(|e| warp::reject::custom(e))
            }
        });
    
    create_position
        .or(get_positions)
        .or(get_position)
        .or(update_position)
        .or(delete_position)
        .or(rebalance_position)
}