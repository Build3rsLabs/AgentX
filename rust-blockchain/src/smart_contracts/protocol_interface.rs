use crate::error::AppResult;
use crate::models::position::{Position, PositionStrategy};
use async_trait::async_trait;

/// Protocol interface that all protocol implementations must follow
#[async_trait]
pub trait ProtocolInterface {
    /// Get the protocol ID
    fn get_id(&self) -> &str;
    
    /// Get the protocol name
    fn get_name(&self) -> &str;
    
    /// Get the protocol contract address
    fn get_contract_address(&self) -> &str;
    
    /// Get the current TVL (Total Value Locked) in the protocol
    async fn get_tvl(&self) -> AppResult<f64>;
    
    /// Get the current APY (Annual Percentage Yield) for the protocol
    async fn get_apy(&self) -> AppResult<f64>;
    
    /// Get the list of supported tokens
    async fn get_supported_tokens(&self) -> AppResult<Vec<String>>;
    
    /// Get the list of available pools
    async fn get_pools(&self) -> AppResult<Vec<String>>;
    
    /// Get the APY for a specific pool
    async fn get_pool_apy(&self, pool_id: &str) -> AppResult<f64>;
    
    /// Get the TVL for a specific pool
    async fn get_pool_tvl(&self, pool_id: &str) -> AppResult<f64>;
    
    /// Deposit funds into the protocol
    async fn deposit(&self, 
                    user_address: &str, 
                    amount: f64, 
                    token: &str) -> AppResult<String>;
    
    /// Withdraw funds from the protocol
    async fn withdraw(&self, 
                     user_address: &str, 
                     amount: f64, 
                     token: &str) -> AppResult<String>;
    
    /// Rebalance a position according to the strategy
    async fn rebalance(&self, 
                      position: &Position) -> AppResult<String>;
    
    /// Claim rewards from the protocol
    async fn claim_rewards(&self, 
                          user_address: &str, 
                          position_id: &str) -> AppResult<String>;
    
    /// Get the current value of a position
    async fn get_position_value(&self, 
                               position_id: &str) -> AppResult<f64>;
    
    /// Get the optimal allocation for a strategy
    async fn get_optimal_allocation(&self, 
                                   strategy: &PositionStrategy) -> AppResult<Vec<(String, f64)>>;
    
    /// Check if a token is supported by the protocol
    fn is_token_supported(&self, token: &str) -> bool;
    
    /// Get the gas limit for a specific operation
    fn get_gas_limit_for_operation(&self, operation: &str) -> u64;
    
    /// Get the protocol risk level
    fn get_risk_level(&self) -> &str;
}