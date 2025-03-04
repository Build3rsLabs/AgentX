//! Protocol Adapter for ElizaOS
//!
//! Provides a standardized interface for interacting with different DeFi protocols
//! on the MultiversX blockchain.

use crate::blockchain::MultiversXClient;
use crate::error::{AppError, AppResult};
use crate::models::position::{Position, PositionStrategy};
use crate::smart_contracts::protocol_interface::ProtocolInterface;
use crate::elizaos::transaction_manager::{TransactionManager, TransactionStatus};
use crate::wallet::Wallet;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, debug, error, warn};

/// Protocol adapter for a specific DeFi protocol
pub struct ProtocolAdapter {
    /// The underlying protocol implementation
    protocol: Arc<dyn ProtocolInterface + Send + Sync>,
    /// Transaction manager for handling blockchain transactions
    tx_manager: Arc<TransactionManager>,
    /// Blockchain client
    blockchain_client: MultiversXClient,
    /// Cache for protocol data
    cache: HashMap<String, CacheEntry>,
}

/// Cache entry for protocol data
struct CacheEntry {
    /// When the data was last updated
    last_updated: chrono::DateTime<chrono::Utc>,
    /// The cached data
    data: serde_json::Value,
    /// Time-to-live in seconds
    ttl: u64,
}

impl ProtocolAdapter {
    /// Create a new protocol adapter
    pub fn new(
        protocol: Arc<dyn ProtocolInterface + Send + Sync>,
        tx_manager: Arc<TransactionManager>,
        blockchain_client: MultiversXClient,
    ) -> Self {
        Self {
            protocol,
            tx_manager,
            blockchain_client,
            cache: HashMap::new(),
        }
    }
    
    /// Get the protocol ID
    pub fn get_id(&self) -> &str {
        self.protocol.get_id()
    }
    
    /// Get the protocol name
    pub fn get_name(&self) -> &str {
        self.protocol.get_name()
    }
    
    /// Get the protocol contract address
    pub fn get_contract_address(&self) -> &str {
        self.protocol.get_contract_address()
    }
    
    /// Get the protocol risk level
    pub fn get_risk_level(&self) -> &str {
        self.protocol.get_risk_level()
    }
    
    /// Get the current TVL (Total Value Locked) in the protocol
    pub async fn get_tvl(&self) -> AppResult<f64> {
        // Check cache first
        if let Some(entry) = self.cache.get("tvl") {
            if entry.last_updated + chrono::Duration::seconds(entry.ttl as i64) > chrono::Utc::now() {
                if let Some(tvl) = entry.data.as_f64() {
                    return Ok(tvl);
                }
            }
        }
        
        // Cache miss or expired, get from protocol
        let tvl = self.protocol.get_tvl().await?;
        
        // Update cache
        let cache_entry = CacheEntry {
            last_updated: chrono::Utc::now(),
            data: serde_json::Value::from(tvl),
            ttl: 300, // 5 minutes
        };
        
        let mut cache = self.cache.clone();
        cache.insert("tvl".to_string(), cache_entry);
        
        Ok(tvl)
    }
    
    /// Get the current APY (Annual Percentage Yield) for the protocol
    pub async fn get_apy(&self) -> AppResult<f64> {
        // Check cache first
        if let Some(entry) = self.cache.get("apy") {
            if entry.last_updated + chrono::Duration::seconds(entry.ttl as i64) > chrono::Utc::now() {
                if let Some(apy) = entry.data.as_f64() {
                    return Ok(apy);
                }
            }
        }
        
        // Cache miss or expired, get from protocol
        let apy = self.protocol.get_apy().await?;
        
        // Update cache
        let cache_entry = CacheEntry {
            last_updated: chrono::Utc::now(),
            data: serde_json::Value::from(apy),
            ttl: 300, // 5 minutes
        };
        
        let mut cache = self.cache.clone();
        cache.insert("apy".to_string(), cache_entry);
        
        Ok(apy)
    }
    
    /// Get the list of supported tokens
    pub async fn get_supported_tokens(&self) -> AppResult<Vec<String>> {
        // Check cache first
        if let Some(entry) = self.cache.get("tokens") {
            if entry.last_updated + chrono::Duration::seconds(entry.ttl as i64) > chrono::Utc::now() {
                if let Some(tokens_array) = entry.data.as_array() {
                    let tokens = tokens_array.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect();
                    return Ok(tokens);
                }
            }
        }
        
        // Cache miss or expired, get from protocol
        let tokens = self.protocol.get_supported_tokens().await?;
        
        // Update cache
        let cache_entry = CacheEntry {
            last_updated: chrono::Utc::now(),
            data: serde_json::Value::from(tokens.clone()),
            ttl: 3600, // 1 hour
        };
        
        let mut cache = self.cache.clone();
        cache.insert("tokens".to_string(), cache_entry);
        
        Ok(tokens)
    }
    
    /// Get the list of available pools
    pub async fn get_pools(&self) -> AppResult<Vec<String>> {
        // Check cache first
        if let Some(entry) = self.cache.get("pools") {
            if entry.last_updated + chrono::Duration::seconds(entry.ttl as i64) > chrono::Utc::now() {
                if let Some(pools_array) = entry.data.as_array() {
                    let pools = pools_array.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect();
                    return Ok(pools);
                }
            }
        }
        
        // Cache miss or expired, get from protocol
        let pools = self.protocol.get_pools().await?;
        
        // Update cache
        let cache_entry = CacheEntry {
            last_updated: chrono::Utc::now(),
            data: serde_json::Value::from(pools.clone()),
            ttl: 3600, // 1 hour
        };
        
        let mut cache = self.cache.clone();
        cache.insert("pools".to_string(), cache_entry);
        
        Ok(pools)
    }
    
    /// Get the APY for a specific pool
    pub async fn get_pool_apy(&self, pool_id: &str) -> AppResult<f64> {
        // Check cache first
        let cache_key = format!("pool_apy_{}", pool_id);
        if let Some(entry) = self.cache.get(&cache_key) {
            if entry.last_updated + chrono::Duration::seconds(entry.ttl as i64) > chrono::Utc::now() {
                if let Some(apy) = entry.data.as_f64() {
                    return Ok(apy);
                }
            }
        }
        
        // Cache miss or expired, get from protocol
        let apy = self.protocol.get_pool_apy(pool_id).await?;
        
        // Update cache
        let cache_entry = CacheEntry {
            last_updated: chrono::Utc::now(),
            data: serde_json::Value::from(apy),
            ttl: 300, // 5 minutes
        };
        
        let mut cache = self.cache.clone();
        cache.insert(cache_key, cache_entry);
        
        Ok(apy)
    }
    
    /// Get the TVL for a specific pool
    pub async fn get_pool_tvl(&self, pool_id: &str) -> AppResult<f64> {
        // Check cache first
        let cache_key = format!("pool_tvl_{}", pool_id);
        if let Some(entry) = self.cache.get(&cache_key) {
            if entry.last_updated + chrono::Duration::seconds(entry.ttl as i64) > chrono::Utc::now() {
                if let Some(tvl) = entry.data.as_f64() {
                    return Ok(tvl);
                }
            }
        }
        
        // Cache miss or expired, get from protocol
        let tvl = self.protocol.get_pool_tvl(pool_id).await?;
        
        // Update cache
        let cache_entry = CacheEntry {
            last_updated: chrono::Utc::now(),
            data: serde_json::Value::from(tvl),
            ttl: 300, // 5 minutes
        };
        
        let mut cache = self.cache.clone();
        cache.insert(cache_key, cache_entry);
        
        Ok(tvl)
    }
    
    /// Deposit funds into the protocol
    pub async fn deposit(
        &self,
        wallet: &Wallet,
        amount: f64,
        token: &str,
    ) -> AppResult<String> {
        debug!("Depositing {} {} into {} protocol", amount, token, self.get_name());
        
        // Check if token is supported
        if !self.protocol.is_token_supported(token) {
            return Err(AppError::Validation(format!("Token {} is not supported by {} protocol", token, self.get_name())));
        }
        
        // Get contract address
        let contract_address = self.get_contract_address();
        
        // Prepare transaction data
        let data = Some(format!("deposit@{}", token));
        
        // Get gas limit for deposit operation
        let gas_limit = self.protocol.get_gas_limit_for_operation("deposit");
        
        // Submit transaction
        let tx_hash = self.tx_manager.submit_transaction(
            wallet,
            contract_address,
            amount,
            data,
            Some(gas_limit),
        ).await?;
        
        info!("Deposit transaction submitted: {}", tx_hash);
        Ok(tx_hash)
    }
    
    /// Withdraw funds from the protocol
    pub async fn withdraw(
        &self,
        wallet: &Wallet,
        amount: f64,
        token: &str,
    ) -> AppResult<String> {
        debug!("Withdrawing {} {} from {} protocol", amount, token, self.get_name());
        
        // Check if token is supported
        if !self.protocol.is_token_supported(token) {
            return Err(AppError::Validation(format!("Token {} is not supported by {} protocol", token, self.get_name())));
        }
        
        // Get contract address
        let contract_address = self.get_contract_address();
        
        // Prepare transaction data
        let data = Some(format!("withdraw@{}", token));
        
        // Get gas limit for withdraw operation
        let gas_limit = self.protocol.get_gas_limit_for_operation("withdraw");
        
        // Submit transaction
        let tx_hash = self.tx_manager.submit_transaction(
            wallet,
            contract_address,
            0.0, // No value transfer for withdraw
            data,
            Some(gas_limit),
        ).await?;
        
        info!("Withdraw transaction submitted: {}", tx_hash);
        Ok(tx_hash)
    }
    
    /// Rebalance a position according to the strategy
    pub async fn rebalance(
        &self,
        wallet: &Wallet,
        position: &Position,
    ) -> AppResult<String> {
        debug!("Rebalancing position {} with strategy {:?} in {} protocol", 
               position.id, position.strategy, self.get_name());
        
        // Get optimal allocation based on strategy
        let optimal_allocation = self.protocol.get_optimal_allocation(&position.strategy).await?;
        
        // Get contract address
        let contract_address = self.get_contract_address();
        
        // Prepare transaction data
        let mut data_parts = vec!["rebalance"];
        for (token, percentage) in optimal_allocation {
            data_parts.push(&token);
            data_parts.push(&format!("{}", (percentage * 100.0) as u64));
        }
        
        let data = Some(data_parts.join("@"));
        
        // Get gas limit for rebalance operation
        let gas_limit = self.protocol.get_gas_limit_for_operation("rebalance");
        
        // Submit transaction
        let tx_hash = self.tx_manager.submit_transaction(
            wallet,
            contract_address,
            0.0, // No value transfer for rebalance
            data,
            Some(gas_limit),
        ).await?;
        
        info!("Rebalance transaction submitted: {}", tx_hash);
        Ok(tx_hash)
    }
    
    /// Claim rewards from the protocol
    pub async fn claim_rewards(
        &self,
        wallet: &Wallet,
        position_id: &str,
    ) -> AppResult<String> {
        debug!("Claiming rewards for position {} from {} protocol", position_id, self.get_name());
        
        // Get contract address
        let contract_address = self.get_contract_address();
        
        // Prepare transaction data
        let data = Some(format!("claimRewards@{}", position_id));
        
        // Get gas limit for claim rewards operation
        let gas_limit = self.protocol.get_gas_limit_for_operation("claimRewards");
        
        // Submit transaction
        let tx_hash = self.tx_manager.submit_transaction(
            wallet,
            contract_address,
            0.0, // No value transfer for claim rewards
            data,
            Some(gas_limit),
        ).await?;
        
        info!("Claim rewards transaction submitted: {}", tx_hash);
        Ok(tx_hash)
    }
    
    /// Get the current value of a position
    pub async fn get_position_value(
        &self,
        position_id: &str,
    ) -> AppResult<f64> {
        // Check cache first
        let cache_key = format!("position_value_{}", position_id);
        if let Some(entry) = self.cache.get(&cache_key) {
            if entry.last_updated + chrono::Duration::seconds(entry.ttl as i64) > chrono::Utc::now() {
                if let Some(value) = entry.data.as_f64() {
                    return Ok(value);
                }
            }
        }
        
        // Cache miss or expired, get from protocol
        let value = self.protocol.get_position_value(position_id).await?;
        
        // Update cache
        let cache_entry = CacheEntry {
            last_updated: chrono::Utc::now(),
            data: serde_json::Value::from(value),
            ttl: 60, // 1 minute
        };
        
        let mut cache = self.cache.clone();
        cache.insert(cache_key, cache_entry);
        
        Ok(value)
    }
    
    /// Get the optimal allocation for a strategy
    pub async fn get_optimal_allocation(
        &self,
        strategy: &PositionStrategy,
    ) -> AppResult<Vec<(String, f64)>> {
        // Check cache first
        let cache_key = format!("optimal_allocation_{:?}", strategy);
        if let Some(entry) = self.cache.get(&cache_key) {
            if entry.last_updated + chrono::Duration::seconds(entry.ttl as i64) > chrono::Utc::now() {
                if let Some(allocation_array) = entry.data.as_array() {
                    let allocation = allocation_array.iter()
                        .filter_map(|v| {
                            if let Some(obj) = v.as_object() {
                                if let (Some(token), Some(percentage)) = (
                                    obj.get("token").and_then(|t| t.as_str()),
                                    obj.get("percentage").and_then(|p| p.as_f64())
                                ) {
                                    return Some((token.to_string(), percentage));
                                }
                            }
                            None
                        })
                        .collect();
                    return Ok(allocation);
                }
            }
        }
        
        // Cache miss or expired, get from protocol
        let allocation = self.protocol.get_optimal_allocation(strategy).await?;
        
        // Update cache
        let allocation_json: Vec<serde_json::Value> = allocation.iter()
            .map(|(token, percentage)| {
                serde_json::json!({
                    "token": token,
                    "percentage": percentage
                })
            })
            .collect();
        
        let cache_entry = CacheEntry {
            last_updated: chrono::Utc::now(),
            data: serde_json::Value::from(allocation_json),
            ttl: 3600, // 1 hour
        };
        
        let mut cache = self.cache.clone();
        cache.insert(cache_key, cache_entry);
        
        Ok(allocation)
    }
    
    /// Check if a token is supported by the protocol
    pub fn is_token_supported(&self, token: &str) -> bool {
        self.protocol.is_token_supported(token)
    }
    
    /// Clear the cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
    
    /// Clear a specific cache entry
    pub fn clear_cache_entry(&mut self, key: &str) {
        self.cache.remove(key);
    }
}