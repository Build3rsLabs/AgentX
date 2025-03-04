//! Transaction Manager for ElizaOS
//!
//! Handles the lifecycle of blockchain transactions, including creation,
//! signing, submission, confirmation, and error handling.

use crate::blockchain::{BlockchainProvider, MultiversXClient, TransactionRequest};
use crate::error::{AppError, AppResult};
use crate::wallet::Wallet;
use crate::utils::{egld_to_denomination, denomination_to_egld};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::time::{timeout, Duration};
use tracing::{info, debug, error, warn};
use uuid::Uuid;

/// Status of a transaction in the transaction manager
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionStatus {
    /// Transaction is queued for processing
    Queued,
    /// Transaction is being prepared (nonce allocation, etc.)
    Preparing,
    /// Transaction has been signed and is ready to be submitted
    Signed,
    /// Transaction has been submitted to the blockchain
    Submitted,
    /// Transaction has been confirmed on the blockchain
    Confirmed,
    /// Transaction failed
    Failed(String),
}

/// A transaction being managed by the transaction manager
#[derive(Debug, Clone)]
pub struct ManagedTransaction {
    /// Unique identifier for this transaction
    pub id: String,
    /// Transaction hash once submitted
    pub hash: Option<String>,
    /// Current status of the transaction
    pub status: TransactionStatus,
    /// Sender address
    pub sender: String,
    /// Receiver address
    pub receiver: String,
    /// Value to transfer (in smallest denomination)
    pub value: String,
    /// Transaction data (for smart contract calls)
    pub data: Option<String>,
    /// Gas price
    pub gas_price: u64,
    /// Gas limit
    pub gas_limit: u64,
    /// Nonce
    pub nonce: Option<u64>,
    /// Timestamp when the transaction was created
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Timestamp when the transaction was last updated
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl ManagedTransaction {
    /// Create a new managed transaction
    pub fn new(
        sender: String,
        receiver: String,
        value: String,
        data: Option<String>,
        gas_price: u64,
        gas_limit: u64,
    ) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            hash: None,
            status: TransactionStatus::Queued,
            sender,
            receiver,
            value,
            data,
            gas_price,
            gas_limit,
            nonce: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Update the status of the transaction
    pub fn update_status(&mut self, status: TransactionStatus) {
        self.status = status;
        self.updated_at = chrono::Utc::now();
    }
    
    /// Set the transaction hash
    pub fn set_hash(&mut self, hash: String) {
        self.hash = Some(hash);
        self.updated_at = chrono::Utc::now();
    }
    
    /// Set the nonce
    pub fn set_nonce(&mut self, nonce: u64) {
        self.nonce = Some(nonce);
        self.updated_at = chrono::Utc::now();
    }
}

/// Command sent to the transaction processor
enum TxCommand {
    /// Submit a new transaction
    Submit(ManagedTransaction, mpsc::Sender<AppResult<String>>),
    /// Check the status of a transaction
    CheckStatus(String, mpsc::Sender<AppResult<TransactionStatus>>),
    /// Shutdown the processor
    Shutdown,
}

/// Transaction Manager for ElizaOS
pub struct TransactionManager {
    blockchain_client: MultiversXClient,
    transactions: Arc<Mutex<HashMap<String, ManagedTransaction>>>,
    command_tx: mpsc::Sender<TxCommand>,
    max_concurrent_txs: usize,
    tx_timeout_seconds: u64,
}

impl TransactionManager {
    /// Create a new transaction manager
    pub fn new(
        blockchain_client: MultiversXClient,
        max_concurrent_txs: usize,
        tx_timeout_seconds: u64,
    ) -> Self {
        let transactions = Arc::new(Mutex::new(HashMap::new()));
        let (command_tx, command_rx) = mpsc::channel(100);
        
        // Start the transaction processor
        let processor_transactions = transactions.clone();
        let processor_client = blockchain_client.clone();
        tokio::spawn(async move {
            Self::transaction_processor(
                processor_client,
                processor_transactions,
                command_rx,
                max_concurrent_txs,
                tx_timeout_seconds,
            ).await;
        });
        
        Self {
            blockchain_client,
            transactions,
            command_tx,
            max_concurrent_txs,
            tx_timeout_seconds,
        }
    }
    
    /// Submit a transaction to the blockchain
    pub async fn submit_transaction(
        &self,
        wallet: &Wallet,
        receiver: &str,
        value: f64,
        data: Option<String>,
        gas_limit: Option<u64>,
    ) -> AppResult<String> {
        // Convert EGLD to smallest denomination
        let value_denomination = egld_to_denomination(value)?;
        
        // Create managed transaction
        let tx = ManagedTransaction::new(
            wallet.address(),
            receiver.to_string(),
            value_denomination,
            data,
            self.blockchain_client.get_network_config().min_gas_price,
            gas_limit.unwrap_or(self.blockchain_client.get_network_config().min_gas_limit),
        );
        
        // Store transaction
        let tx_id = tx.id.clone();
        {
            let mut txs = self.transactions.lock().unwrap();
            txs.insert(tx_id.clone(), tx);
        }
        
        // Send command to processor
        let (response_tx, response_rx) = mpsc::channel(1);
        let tx = {
            let txs = self.transactions.lock().unwrap();
            txs.get(&tx_id).cloned().ok_or_else(|| AppError::Internal("Transaction not found".to_string()))?
        };
        
        self.command_tx.send(TxCommand::Submit(tx, response_tx)).await
            .map_err(|e| AppError::Internal(format!("Failed to send transaction command: {}", e)))?;
        
        // Wait for response
        let result = response_rx.await
            .map_err(|e| AppError::Internal(format!("Failed to receive transaction response: {}", e)))?;
        
        result
    }
    
    /// Get the status of a transaction
    pub async fn get_transaction_status(&self, tx_id: &str) -> AppResult<TransactionStatus> {
        // Check if we have the transaction in memory
        {
            let txs = self.transactions.lock().unwrap();
            if let Some(tx) = txs.get(tx_id) {
                return Ok(tx.status.clone());
            }
        }
        
        // If not, send command to processor to check
        let (response_tx, response_rx) = mpsc::channel(1);
        self.command_tx.send(TxCommand::CheckStatus(tx_id.to_string(), response_tx)).await
            .map_err(|e| AppError::Internal(format!("Failed to send status check command: {}", e)))?;
        
        // Wait for response
        let result = response_rx.await
            .map_err(|e| AppError::Internal(format!("Failed to receive status check response: {}", e)))?;
        
        result
    }
    
    /// Get all transactions
    pub fn get_all_transactions(&self) -> Vec<ManagedTransaction> {
        let txs = self.transactions.lock().unwrap();
        txs.values().cloned().collect()
    }
    
    /// Get a transaction by ID
    pub fn get_transaction(&self, tx_id: &str) -> Option<ManagedTransaction> {
        let txs = self.transactions.lock().unwrap();
        txs.get(tx_id).cloned()
    }
    
    /// Shutdown the transaction manager
    pub async fn shutdown(&self) -> AppResult<()> {
        self.command_tx.send(TxCommand::Shutdown).await
            .map_err(|e| AppError::Internal(format!("Failed to send shutdown command: {}", e)))?;
        Ok(())
    }
    
    /// Transaction processor loop
    async fn transaction_processor(
        blockchain_client: MultiversXClient,
        transactions: Arc<Mutex<HashMap<String, ManagedTransaction>>>,
        mut command_rx: mpsc::Receiver<TxCommand>,
        max_concurrent_txs: usize,
        tx_timeout_seconds: u64,
    ) {
        let mut active_txs = 0;
        
        loop {
            // Process commands
            match command_rx.recv().await {
                Some(TxCommand::Submit(tx, response_tx)) => {
                    // Check if we can process more transactions
                    if active_txs >= max_concurrent_txs {
                        let _ = response_tx.send(Err(AppError::Transaction(
                            "Too many concurrent transactions".to_string()
                        ))).await;
                        continue;
                    }
                    
                    // Increment active transactions
                    active_txs += 1;
                    
                    // Clone what we need for the task
                    let tx_id = tx.id.clone();
                    let blockchain_client = blockchain_client.clone();
                    let transactions_clone = transactions.clone();
                    
                    // Process transaction in a separate task
                    tokio::spawn(async move {
                        let result = Self::process_transaction(
                            blockchain_client,
                            transactions_clone.clone(),
                            tx,
                            tx_timeout_seconds,
                        ).await;
                        
                        // Send result back
                        let _ = response_tx.send(result).await;
                        
                        // Update active transactions count
                        active_txs -= 1;
                    });
                },
                Some(TxCommand::CheckStatus(tx_id, response_tx)) => {
                    // Check transaction status
                    let status = {
                        let txs = transactions.lock().unwrap();
                        txs.get(&tx_id).map(|tx| tx.status.clone())
                    };
                    
                    if let Some(status) = status {
                        let _ = response_tx.send(Ok(status)).await;
                    } else {
                        // If not found in memory, check on blockchain
                        let blockchain_client = blockchain_client.clone();
                        tokio::spawn(async move {
                            let result = blockchain_client.get_transaction_status(&tx_id).await
                                .map(|status| match status {
                                    crate::models::transaction::TransactionStatus::Pending => TransactionStatus::Submitted,
                                    crate::models::transaction::TransactionStatus::Success => TransactionStatus::Confirmed,
                                    crate::models::transaction::TransactionStatus::Failed => TransactionStatus::Failed("Transaction failed on blockchain".to_string()),
                                    crate::models::transaction::TransactionStatus::Unknown => TransactionStatus::Failed("Unknown transaction status".to_string()),
                                });
                            
                            let _ = response_tx.send(result).await;
                        });
                    }
                },
                Some(TxCommand::Shutdown) => {
                    info!("Shutting down transaction processor");
                    break;
                },
                None => {
                    error!("Transaction command channel closed unexpectedly");
                    break;
                }
            }
        }
    }
    
    /// Process a single transaction
    async fn process_transaction(
        blockchain_client: MultiversXClient,
        transactions: Arc<Mutex<HashMap<String, ManagedTransaction>>>,
        mut tx: ManagedTransaction,
        tx_timeout_seconds: u64,
    ) -> AppResult<String> {
        // Update status to preparing
        tx.update_status(TransactionStatus::Preparing);
        {
            let mut txs = transactions.lock().unwrap();
            txs.insert(tx.id.clone(), tx.clone());
        }
        
        // Get account nonce
        let account = match blockchain_client.get_account(&tx.sender).await {
            Ok(account) => account,
            Err(e) => {
                let error_msg = format!("Failed to get account: {}", e);
                tx.update_status(TransactionStatus::Failed(error_msg.clone()));
                {
                    let mut txs = transactions.lock().unwrap();
                    txs.insert(tx.id.clone(), tx);
                }
                return Err(AppError::Transaction(error_msg));
            }
        };
        
        // Set nonce
        tx.set_nonce(account.nonce);
        {
            let mut txs = transactions.lock().unwrap();
            txs.insert(tx.id.clone(), tx.clone());
        }
        
        // Create transaction request
        let tx_request = TransactionRequest {
            nonce: tx.nonce.unwrap(),
            value: tx.value.clone(),
            receiver: tx.receiver.clone(),
            sender: tx.sender.clone(),
            gasPrice: tx.gas_price,
            gasLimit: tx.gas_limit,
            data: tx.data.clone(),
            chainID: blockchain_client.get_network_config().chain_id.clone(),
            version: 1,
            signature: "".to_string(), // Will be filled below
        };
        
        // Serialize transaction for signing
        let tx_json = match serde_json::to_string(&tx_request) {
            Ok(json) => json,
            Err(e) => {
                let error_msg = format!("Failed to serialize transaction: {}", e);
                tx.update_status(TransactionStatus::Failed(error_msg.clone()));
                {
                    let mut txs = transactions.lock().unwrap();
                    txs.insert(tx.id.clone(), tx);
                }
                return Err(AppError::Transaction(error_msg));
            }
        };
        
        // Sign transaction (in a real implementation, this would use the wallet)
        let signature = "simulated_signature_for_demo_purposes_only".to_string();
        
        // Update status to signed
        tx.update_status(TransactionStatus::Signed);
        {
            let mut txs = transactions.lock().unwrap();
            txs.insert(tx.id.clone(), tx.clone());
        }
        
        // Create final transaction with signature
        let final_tx = TransactionRequest {
            signature,
            ..tx_request
        };
        
        // Send transaction to blockchain
        let tx_hash = match blockchain_client.send_transaction(final_tx).await {
            Ok(hash) => hash,
            Err(e) => {
                let error_msg = format!("Failed to send transaction: {}", e);
                tx.update_status(TransactionStatus::Failed(error_msg.clone()));
                {
                    let mut txs = transactions.lock().unwrap();
                    txs.insert(tx.id.clone(), tx);
                }
                return Err(AppError::Transaction(error_msg));
            }
        };
        
        // Update transaction with hash and status
        tx.set_hash(tx_hash.clone());
        tx.update_status(TransactionStatus::Submitted);
        {
            let mut txs = transactions.lock().unwrap();
            txs.insert(tx.id.clone(), tx.clone());
        }
        
        // Wait for transaction confirmation
        let timeout_duration = Duration::from_secs(tx_timeout_seconds);
        let confirmation_result = timeout(timeout_duration, Self::wait_for_confirmation(
            blockchain_client.clone(),
            tx_hash.clone(),
        )).await;
        
        match confirmation_result {
            Ok(Ok(status)) => {
                // Transaction confirmed
                match status {
                    crate::models::transaction::TransactionStatus::Success => {
                        tx.update_status(TransactionStatus::Confirmed);
                        {
                            let mut txs = transactions.lock().unwrap();
                            txs.insert(tx.id.clone(), tx);
                        }
                        Ok(tx_hash)
                    },
                    crate::models::transaction::TransactionStatus::Failed => {
                        let error_msg = "Transaction failed on blockchain".to_string();
                        tx.update_status(TransactionStatus::Failed(error_msg.clone()));
                        {
                            let mut txs = transactions.lock().unwrap();
                            txs.insert(tx.id.clone(), tx);
                        }
                        Err(AppError::Transaction(error_msg))
                    },
                    _ => {
                        let error_msg = "Transaction did not complete successfully".to_string();
                        tx.update_status(TransactionStatus::Failed(error_msg.clone()));
                        {
                            let mut txs = transactions.lock().unwrap();
                            txs.insert(tx.id.clone(), tx);
                        }
                        Err(AppError::Transaction(error_msg))
                    }
                }
            },
            Ok(Err(e)) => {
                // Error checking transaction status
                let error_msg = format!("Error checking transaction status: {}", e);
                tx.update_status(TransactionStatus::Failed(error_msg.clone()));
                {
                    let mut txs = transactions.lock().unwrap();
                    txs.insert(tx.id.clone(), tx);
                }
                Err(AppError::Transaction(error_msg))
            },
            Err(_) => {
                // Timeout waiting for confirmation
                let error_msg = format!("Timeout waiting for transaction confirmation after {} seconds", tx_timeout_seconds);
                tx.update_status(TransactionStatus::Failed(error_msg.clone()));
                {
                    let mut txs = transactions.lock().unwrap();
                    txs.insert(tx.id.clone(), tx);
                }
                Err(AppError::Transaction(error_msg))
            }
        }
    }
    
    /// Wait for transaction confirmation
    async fn wait_for_confirmation(
        blockchain_client: MultiversXClient,
        tx_hash: String,
    ) -> AppResult<crate::models::transaction::TransactionStatus> {
        let mut attempts = 0;
        let max_attempts = 30;
        let delay = Duration::from_secs(2);
        
        loop {
            attempts += 1;
            
            // Check transaction status
            let status = blockchain_client.get_transaction_status(&tx_hash).await?;
            
            match status {
                crate::models::transaction::TransactionStatus::Pending => {
                    // Still pending, wait and try again
                    if attempts >= max_attempts {
                        return Err(AppError::Transaction("Maximum attempts reached waiting for transaction confirmation".to_string()));
                    }
                    
                    tokio::time::sleep(delay).await;
                },
                _ => {
                    // Final status reached
                    return Ok(status);
                }
            }
        }
    }
}