use crate::api::middleware::with_auth;
use crate::models::transaction::CreateTransactionRequest;
use crate::services::ServiceContainer;
use std::sync::Arc;
use uuid::Uuid;
use warp::{Filter, Rejection, Reply};

pub fn routes(
    services: Arc<ServiceContainer>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let transaction_service = services.transaction_service.clone();
    
    let create_transaction = warp::path!("transactions")
        .and(warp::post())
        .and(with_auth(services.clone()))
        .and(warp::body::json())
        .and_then(move |user_id, mut request: CreateTransactionRequest| {
            let transaction_service = transaction_service.clone();
            // Ensure the user_id in the request matches the authenticated user
            request.user_id = user_id;
            async move {
                transaction_service
                    .create_transaction(request)
                    .await
                    .map(|transaction| warp::reply::json(&transaction))
                    .map_err(|e| warp::reject::custom(e))
            }
        });
    
    let get_transactions = warp::path!("transactions")
        .and(warp::get())
        .and(with_auth(services.clone()))
        .and_then(move |user_id| {
            let transaction_service = transaction_service.clone();
            async move {
                transaction_service
                    .get_transactions_by_user(user_id)
                    .await
                    .map(|transactions| warp::reply::json(&transactions))
                    .map_err(|e| warp::reject::custom(e))
            }
        });
    
    let get_transaction = warp::path!("transactions" / Uuid)
        .and(warp::get())
        .and(with_auth(services.clone()))
        .and_then(move |id, user_id| {
            let transaction_service = transaction_service.clone();
            async move {
                transaction_service
                    .get_transaction_by_id(id, user_id)
                    .await
                    .map(|transaction| warp::reply::json(&transaction))
                    .map_err(|e| warp::reject::custom(e))
            }
        });
    
    let get_transaction_by_hash = warp::path!("transactions" / "hash" / String)
        .and(warp::get())
        .and_then(move |tx_hash| {
            let transaction_service = transaction_service.clone();
            async move {
                transaction_service
                    .get_transaction_by_hash(&tx_hash)
                    .await
                    .map(|transaction| warp::reply::json(&transaction))
                    .map_err(|e| warp::reject::custom(e))
            }
        });
    
    create_transaction
        .or(get_transactions)
        .or(get_transaction)
        .or(get_transaction_by_hash)
}