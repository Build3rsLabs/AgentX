mod auth_service;
mod position_service;
mod protocol_service;
mod transaction_service;
mod user_service;
mod yield_optimizer_service;

pub use auth_service::AuthService;
pub use position_service::PositionService;
pub use protocol_service::ProtocolService;
pub use transaction_service::TransactionService;
pub use user_service::UserService;
pub use yield_optimizer_service::YieldOptimizerService;

use crate::blockchain::MultiversXClient;
use crate::config::AppConfig;
use crate::smart_contracts::ProtocolRegistry;
use sqlx::PgPool;
use std::sync::Arc;

pub struct ServiceContainer {
    pub db_pool: PgPool,
    pub blockchain_client: MultiversXClient,
    pub protocol_registry: ProtocolRegistry,
    pub config: Arc<AppConfig>,
    pub auth_service: AuthService,
    pub user_service: UserService,
    pub protocol_service: ProtocolService,
    pub position_service: PositionService,
    pub transaction_service: TransactionService,
    pub yield_optimizer_service: YieldOptimizerService,
}

impl ServiceContainer {
    pub fn new(db_pool: PgPool, blockchain_client: MultiversXClient, config: Arc<AppConfig>) -> Self {
        let protocol_registry = ProtocolRegistry::new(blockchain_client.clone());
        
        let auth_service = AuthService::new(db_pool.clone(), config.clone());
        let user_service = UserService::new(db_pool.clone());
        let protocol_service = ProtocolService::new(db_pool.clone(), protocol_registry.clone());
        let position_service = PositionService::new(db_pool.clone(), blockchain_client.clone());
        let transaction_service = TransactionService::new(db_pool.clone(), blockchain_client.clone());
        let yield_optimizer_service = YieldOptimizerService::new(protocol_registry.clone());
        
        Self {
            db_pool,
            blockchain_client,
            protocol_registry,
            config,
            auth_service,
            user_service,
            protocol_service,
            position_service,
            transaction_service,
            yield_optimizer_service,
        }
    }
}