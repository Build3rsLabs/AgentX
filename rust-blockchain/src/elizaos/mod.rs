//! ElizaOS - Specialized operating system layer for secure blockchain operations
//! 
//! ElizaOS provides a secure, efficient environment for interacting with the MultiversX
//! blockchain, with a focus on DeFi protocol integration and yield optimization.

pub mod transaction_manager;
pub mod protocol_adapter;
pub mod yield_optimizer;

use crate::blockchain::{BlockchainProvider, MultiversXClient};
use crate::error::{AppError, AppResult};
use crate::models::position::{Position, PositionStrategy};
use crate::smart_contracts::ProtocolRegistry;
use crate::wallet::Wallet;
use transaction_manager::{TransactionManager, TransactionStatus};
use protocol_adapter::ProtocolAdapter;
use yield_optimizer::{YieldOptimizer, YieldOpportunity, RiskMetrics};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, debug, error};

/// Core ElizaOS system that coordinates all blockchain operations
pub struct ElizaOS {
    blockchain_client: MultiversXClient,
    protocol_registry: ProtocolRegistry,
    tx_manager: Arc<TransactionManager>,
    protocol_adapters: HashMap<String, Arc<ProtocolAdapter>>,
    yield_optimizer: YieldOptimizer,
    config: ElizaOSConfig,
}

/// Configuration for ElizaOS
#[derive(Debug, Clone)]
pub struct ElizaOSConfig {
    /// Network to connect to (devnet, testnet, mainnet)
    pub network: String,
    /// Maximum gas price to use for transactions
    pub max_gas_price: u64,
    /// Default gas limit for transactions
    pub default_gas_limit: u64,
    /// Rebalancing threshold (percentage difference that triggers rebalancing)
    pub rebalance_threshold: f64,
    /// Maximum number of concurrent transactions
    pub max_concurrent_txs: usize,
    /// Transaction timeout in seconds
    pub tx_timeout_seconds: u64,
}

impl Default for ElizaOSConfig {
    fn default() -> Self {
        Self {
            network: "devnet".to_string(),
            max_gas_price: 1_000_000_000, // 1 Gwei
            default_gas_limit: 500_000,
            rebalance_threshold: 0.05, // 5%
            max_concurrent_txs: 10,
            tx_timeout_seconds: 300, // 5 minutes
        }
    }
}

impl ElizaOS {
    /// Create a new ElizaOS instance
    pub fn new(blockchain_client: MultiversXClient, config: Option<ElizaOSConfig>) -> Self {
        let config = config.unwrap_or_default();
        let protocol_registry = ProtocolRegistry::new(blockchain_client.clone());
        
        // Create transaction manager
        let tx_manager = Arc::new(TransactionManager::new(
            blockchain_client.clone(),
            config.max_concurrent_txs,
            config.tx_timeout_seconds,
        ));
        
        // Create protocol adapters
        let mut protocol_adapters = HashMap::new();
        for protocol in protocol_registry.get_all_protocols() {
            let adapter = Arc::new(ProtocolAdapter::new(
                protocol.clone(),
                tx_manager.clone(),
                blockchain_client.clone(),
            ));
            protocol_adapters.insert(protocol.get_id().to_string(), adapter.clone());
        }
        
        // Create yield optimizer
        let yield_optimizer = YieldOptimizer::new(
            protocol_adapters.values().cloned().collect(),
        );
        
        Self {
            blockchain_client,
            protocol_registry,
            tx_manager,
            protocol_adapters,
            yield_optimizer,
            config,
        }
    }
    
    /// Initialize ElizaOS and connect to the blockchain
    pub async fn initialize(&self) -> AppResult<()> {
        info!("Initializing ElizaOS on {} network", self.config.network);
        
        // Check blockchain connection
        let network_status = self.blockchain_client.get_network_status().await?;
        info!("Connected to MultiversX blockchain. Current round: {}, Epoch: {}", 
              network_status.erd_current_round, 
              network_status.erd_epoch_number);
        
        // Initialize protocol registry
        let protocols = self.protocol_registry.get_all_protocols();
        info!("Loaded {} protocols", protocols.len());
        
        for protocol in protocols {
            debug!("Initialized protocol: {}", protocol.get_name());
        }
        
        info!("ElizaOS initialization complete");
        Ok(())
    }
    
    /// Create a new position
    pub async fn create_position(
        &self,
        wallet: &Wallet,
        protocol_id: &str,
        pool_id: &str,
        amount: f64,
        strategy: PositionStrategy,
    ) -> AppResult<String> {
        info!("Creating position in protocol {} pool {} with amount {} and strategy {:?}", 
              protocol_id, pool_id, amount, strategy);
        
        // Get protocol adapter
        let adapter = self.get_protocol_adapter(protocol_id)?;
        
        // Check if amount is sufficient
        if amount <= 0.0 {
            return Err(AppError::Validation("Amount must be greater than zero".to_string()));
        }
        
        // Get user address
        let user_address = wallet.address();
        
        // Check user balance
        let balance = self.blockchain_client.get_balance(&user_address).await?;
        let balance_f64 = crate::utils::denomination_to_egld(&balance)?;
        
        if balance_f64 < amount {
            return Err(AppError::Validation(format!(
                "Insufficient balance. Required: {} EGLD, Available: {} EGLD",
                amount, balance_f64
            )));
        }
        
        // Deposit funds into protocol
        let tx_hash = adapter.deposit(wallet, amount, "EGLD").await?;
        
        info!("Position created successfully with transaction hash: {}", tx_hash);
        Ok(tx_hash)
    }
    
    /// Rebalance a position
    pub async fn rebalance_position(
        &self,
        wallet: &Wallet,
        position: &Position,
    ) -> AppResult<String> {
        info!("Rebalancing position {} with strategy {:?}", position.id, position.strategy);
        
        // Get protocol adapter
        let adapter = self.get_protocol_adapter(&position.protocol_id)?;
        
        // Rebalance position
        let tx_hash = adapter.rebalance(wallet, position).await?;
        
        info!("Position rebalanced successfully with transaction hash: {}", tx_hash);
        Ok(tx_hash)
    }
    
    /// Withdraw from a position
    pub async fn withdraw_from_position(
        &self,
        wallet: &Wallet,
        position: &Position,
        amount: f64,
    ) -> AppResult<String> {
        info!("Withdrawing {} from position {}", amount, position.id);
        
        // Get protocol adapter
        let adapter = self.get_protocol_adapter(&position.protocol_id)?;
        
        // Check if amount is valid
        if amount <= 0.0 || amount > position.current_value {
            return Err(AppError::Validation(format!(
                "Invalid withdrawal amount. Must be between 0 and {} EGLD",
                position.current_value
            )));
        }
        
        // Withdraw funds from protocol
        let tx_hash = adapter.withdraw(wallet, amount, "EGLD").await?;
        
        info!("Withdrawal successful with transaction hash: {}", tx_hash);
        Ok(tx_hash)
    }
    
    /// Claim rewards from a position
    pub async fn claim_rewards(
        &self,
        wallet: &Wallet,
        position: &Position,
    ) -> AppResult<String> {
        info!("Claiming rewards for position {}", position.id);
        
        // Get protocol adapter
        let adapter = self.get_protocol_adapter(&position.protocol_id)?;
        
        // Claim rewards
        let tx_hash = adapter.claim_rewards(wallet, &position.id.to_string()).await?;
        
        info!("Rewards claimed successfully with transaction hash: {}", tx_hash);
        Ok(tx_hash)
    }
    
    /// Get the current value of a position
    pub async fn get_position_value(
        &self,
        position: &Position,
    ) -> AppResult<f64> {
        debug!("Getting current value for position {}", position.id);
        
        // Get protocol adapter
        let adapter = self.get_protocol_adapter(&position.protocol_id)?;
        
        // Get position value
        let value = adapter.get_position_value(&position.id.to_string()).await?;
        
        debug!("Position {} current value: {}", position.id, value);
        Ok(value)
    }
    
    /// Find the best yield opportunities based on strategy and token
    pub async fn find_best_opportunities(
        &self,
        strategy: &PositionStrategy,
        token: Option<String>,
        limit: usize,
    ) -> AppResult<Vec<YieldOpportunity>> {
        debug!("Finding best yield opportunities for strategy {:?}, token: {:?}", strategy, token);
        
        // Use yield optimizer to find opportunities
        let opportunities = self.yield_optimizer.find_best_opportunities(
            strategy,
            token,
            limit,
        ).await?;
        
        Ok(opportunities)
    }
    
    /// Calculate optimal portfolio allocation
    pub async fn calculate_optimal_allocation(
        &self,
        strategy: &PositionStrategy,
        investment_amount: f64,
    ) -> AppResult<HashMap<String, f64>> {
        debug!("Calculating optimal allocation for strategy {:?}, amount: {}", strategy, investment_amount);
        
        // Get best opportunities
        let opportunities = self.find_best_opportunities(strategy, None, 10).await?;
        
        // Calculate optimal allocation
        let allocation = self.yield_optimizer.calculate_optimal_allocation(
            strategy,
            investment_amount,
            &opportunities,
        ).await?;
        
        Ok(allocation)
    }
    
    /// Check if a position needs rebalancing
    pub async fn should_rebalance(&self, position: &Position) -> AppResult<bool> {
        debug!("Checking if position {} needs rebalancing", position.id);
        
        // Convert position allocation to HashMap
        let mut current_allocation = HashMap::new();
        let total_value = position.current_value;
        
        // In a real implementation, we would get the actual allocation from the blockchain
        // For now, we'll use the position's allocation field
        for alloc in &position.allocation {
            let protocol_id = position.protocol_id.clone();
            // Assume pool_id is the first token in the allocation
            let pool_id = alloc.token.clone();
            let key = format!("{}:{}", protocol_id, pool_id);
            let amount = total_value * (alloc.percentage / 100.0);
            current_allocation.insert(key, amount);
        }
        
        // Check if rebalancing is needed
        let should_rebalance = self.yield_optimizer.should_rebalance(
            &current_allocation,
            &position.strategy,
            total_value,
        ).await?;
        
        debug!("Position {} rebalance check: should_rebalance = {}", position.id, should_rebalance);
        
        Ok(should_rebalance)
    }
    
    /// Get system status
    pub async fn get_system_status(&self) -> AppResult<ElizaOSStatus> {
        debug!("Getting ElizaOS system status");
        
        // Get blockchain status
        let network_status = self.blockchain_client.get_network_status().await?;
        
        // Get protocol statuses
        let mut protocol_statuses = Vec::new();
        
        for (id, adapter) in &self.protocol_adapters {
            // Get TVL and APY
            let tvl = adapter.get_tvl().await?;
            let apy = adapter.get_apy().await?;
            
            protocol_statuses.push(ProtocolStatus {
                id: id.clone(),
                name: adapter.get_name().to_string(),
                tvl,
                apy,
                status: "Online".to_string(),
            });
        }
        
        // Get transaction statistics
        let transactions = self.tx_manager.get_all_transactions();
        let pending_txs = transactions.iter()
            .filter(|tx| matches!(tx.status, TransactionStatus::Queued | TransactionStatus::Preparing | TransactionStatus::Signed | TransactionStatus::Submitted))
            .count();
        
        let completed_txs = transactions.iter()
            .filter(|tx| matches!(tx.status, TransactionStatus::Confirmed))
            .count();
        
        let failed_txs = transactions.iter()
            .filter(|tx| matches!(tx.status, TransactionStatus::Failed(_)))
            .count();
        
        Ok(ElizaOSStatus {
            network: self.config.network.clone(),
            current_round: network_status.erd_current_round,
            current_epoch: network_status.erd_epoch_number,
            protocols: protocol_statuses,
            pending_transactions: pending_txs,
            completed_transactions: completed_txs,
            failed_transactions: failed_txs,
        })
    }
    
    /// Get a protocol adapter by ID
    fn get_protocol_adapter(&self, protocol_id: &str) -> AppResult<Arc<ProtocolAdapter>> {
        self.protocol_adapters.get(protocol_id)
            .cloned()
            .ok_or_else(|| AppError::NotFound(format!("Protocol {} not found", protocol_id)))
    }
    
    /// Get transaction manager
    pub fn get_transaction_manager(&self) -> Arc<TransactionManager> {
        self.tx_manager.clone()
    }
    
    /// Get yield optimizer
    pub fn get_yield_optimizer(&self) -> &YieldOptimizer {
        &self.yield_optimizer
    }
}

/// ElizaOS system status
#[derive(Debug, Clone)]
pub struct ElizaOSStatus {
    pub network: String,
    pub current_round: u64,
    pub current_epoch: u64,
    pub protocols: Vec<ProtocolStatus>,
    pub pending_transactions: usize,
    pub completed_transactions: usize,
    pub failed_transactions: usize,
}

/// Protocol status information
#[derive(Debug, Clone)]
pub struct ProtocolStatus {
    pub id: String,
    pub name: String,
    pub tvl: f64,
    pub apy: f64,
    pub status: String,
}