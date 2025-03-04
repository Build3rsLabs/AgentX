use crate::services::ServiceContainer;
use std::sync::Arc;
use warp::{Filter, Rejection, Reply};

pub fn routes(
    services: Arc<ServiceContainer>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let protocol_service = services.protocol_service.clone();
    
    let get_all_protocols = warp::path!("protocols")
        .and(warp::get())
        .and_then(move || {
            let protocol_service = protocol_service.clone();
            async move {
                protocol_service
                    .get_all_protocols()
                    .await
                    .map(|protocols| warp::reply::json(&protocols))
                    .map_err(|e| warp::reject::custom(e))
            }
        });
    
    let get_protocol_by_id = warp::path!("protocols" / String)
        .and(warp::get())
        .and_then(move |id| {
            let protocol_service = protocol_service.clone();
            async move {
                protocol_service
                    .get_protocol_by_id(&id)
                    .await
                    .map(|protocol| warp::reply::json(&protocol))
                    .map_err(|e| warp::reject::custom(e))
            }
        });
    
    let protocol_service_pools = services.protocol_service.clone();
    
    let get_pools_by_protocol = warp::path!("protocols" / String / "pools")
        .and(warp::get())
        .and_then(move |protocol_id| {
            let protocol_service = protocol_service_pools.clone();
            async move {
                protocol_service
                    .get_pools_by_protocol(&protocol_id)
                    .await
                    .map(|pools| warp::reply::json(&pools))
                    .map_err(|e| warp::reject::custom(e))
            }
        });
    
    let protocol_service_pools_all = services.protocol_service.clone();
    
    let get_all_pools = warp::path!("pools")
        .and(warp::get())
        .and_then(move || {
            let protocol_service = protocol_service_pools_all.clone();
            async move {
                protocol_service
                    .get_all_pools()
                    .await
                    .map(|pools| warp::reply::json(&pools))
                    .map_err(|e| warp::reject::custom(e))
            }
        });
    
    let protocol_service_pool = services.protocol_service.clone();
    
    let get_pool_by_id = warp::path!("pools" / String)
        .and(warp::get())
        .and_then(move |id| {
            let protocol_service = protocol_service_pool.clone();
            async move {
                protocol_service
                    .get_pool_by_id(&id)
                    .await
                    .map(|pool| warp::reply::json(&pool))
                    .map_err(|e| warp::reject::custom(e))
            }
        });
    
    get_all_protocols
        .or(get_protocol_by_id)
        .or(get_pools_by_protocol)
        .or(get_all_pools)
        .or(get_pool_by_id)
}