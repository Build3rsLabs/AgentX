use crate::blockchain::{BlockchainProvider, MultiversXClient, TransactionRequest};
use crate::error::{AppError, AppResult};
use crate::models::transaction::{
    CreateTransactionRequest, Transaction, TransactionResponse, TransactionStatus, TransactionType,
};
use crate::utils::{egld_to_denomination, denomination_to_egld};
use crate::wallet::Wallet;
use chrono::Utc;
use sqlx::PgPool;
use tracing::{info, error, debug};
use uuid::Uuid;
use base64::{Engine as _, engine::general_purpose};

pub struct TransactionService {
    db_pool: PgPool,
    blockchain_client: MultiversXClient,
}

impl TransactionService {
    pub fn new(db_pool: PgPool, blockchain_client: MultiversXClient) -> Self {
        Self {
            db_pool,
            blockchain_client,
        }
    }
    
    pub async fn create_transaction(
        &self,
        request: CreateTransactionRequest,
    ) -> AppResult<TransactionResponse> {
        // Get user wallet address for blockchain operations
        let user = sqlx::query!(
            r#"
            SELECT address FROM users WHERE id = $1
            "#,
            request.user_id
        )
        .fetch_one(&self.db_pool)
        .await?;
        
        // Get protocol contract address if provided
        let contract_address = if let Some(ref protocol_id) = request.protocol_id {
            let protocol = sqlx::query!(
                r#"
                SELECT contract_address FROM protocols WHERE id = $1
                "#,
                protocol_id
            )
            .fetch_one(&self.db_pool)
            .await?;
            
            protocol.contract_address
        } else {
            None
        };
        
        // Begin database transaction
        let mut tx = self.db_pool.begin().await?;
        
        let now = Utc::now();
        let tx_id = Uuid::new_v4();
        let tx_hash = format!("pending_{}", Uuid::new_v4().to_string().replace("-", ""));
        
        // Create transaction record
        let transaction = sqlx::query_as!(
            Transaction,
            r#"
            INSERT INTO transactions (
                id, user_id, tx_hash, tx_type, amount, token,
                status, protocol_id, position_id, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING 
                id, user_id, tx_hash, tx_type as "tx_type: TransactionType",
                amount, token, status as "status: TransactionStatus",
                protocol_id, position_id, created_at, updated_at
            "#,
            tx_id,
            request.user_id,
            tx_hash,
            request.tx_type as _,
            request.amount,
            request.token,
            TransactionStatus::Pending as _,
            request.protocol_id,
            request.position_id,
            now,
            now
        )
        .fetch_one(&mut *tx)
        .await?;
        
        // Commit database transaction
        tx.commit().await?;
        
        info!("Created transaction: {} for user: {}", tx_id, request.user_id);
        
        // Process the transaction on the blockchain
        self.process_blockchain_transaction(transaction.clone(), user.address, contract_address).await?;
        
        Ok(TransactionResponse::from(transaction))
    }
    
    pub async fn get_transactions_by_user(
        &self,
        user_id: Uuid,
    ) -> AppResult<Vec<TransactionResponse>> {
        let transactions = sqlx::query_as!(
            Transaction,
            r#"
            SELECT 
                id, user_id, tx_hash, tx_type as "tx_type: TransactionType",
                amount, token, status as "status: TransactionStatus",
                protocol_id, position_id, created_at, updated_at
            FROM transactions
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.db_pool)
        .await?;
        
        // Update transaction statuses from blockchain
        let updated_transactions = self.update_transaction_statuses(transactions).await?;
        
        Ok(updated_transactions.into_iter().map(TransactionResponse::from).collect())
    }
    
    pub async fn get_transaction_by_id(
        &self,
        id: Uuid,
        user_id: Uuid,
    ) -> AppResult<TransactionResponse> {
        let transaction = sqlx::query_as!(
            Transaction,
            r#"
            SELECT 
                id, user_id, tx_hash, tx_type as "tx_type: TransactionType",
                amount, token, status as "status: TransactionStatus",
                protocol_id, position_id, created_at, updated_at
            FROM transactions
            WHERE id = $1 AND user_id = $2
            "#,
            id,
            user_id
        )
        .fetch_one(&self.db_pool)
        .await?;
        
        // Update transaction status from blockchain if needed
        let updated_transaction = self.update_transaction_status(transaction).await?;
        
        Ok(TransactionResponse::from(updated_transaction))
    }
    
    pub async fn get_transaction_by_hash(&self, tx_hash: &str) -> AppResult<TransactionResponse> {
        let transaction = sqlx::query_as!(
            Transaction,
            r#"
            SELECT 
                id, user_id, tx_hash, tx_type as "tx_type: TransactionType",
                amount, token, status as "status: TransactionStatus",
                protocol_id, position_id, created_at, updated_at
            FROM transactions
            WHERE tx_hash = $1
            "#,
            tx_hash
        )
        .fetch_one(&self.db_pool)
        .await?;
        
        // Update transaction status from blockchain if needed
        let updated_transaction = self.update_transaction_status(transaction).await?;
        
        Ok(TransactionResponse::from(updated_transaction))
    }
    
    pub async fn update_transaction_status(
        &self,
        id: Uuid,
        status: TransactionStatus,
    ) -> AppResult<TransactionResponse> {
        let transaction = sqlx::query_as!(
            Transaction,
            r#"
            UPDATE transactions
            SET status = $1, updated_at = $2
            WHERE id = $3
            RETURNING 
                id, user_id, tx_hash, tx_type as "tx_type: TransactionType",
                amount, token, status as "status: TransactionStatus",
                protocol_id, position_id, created_at, updated_at
            "#,
            status as _,
            Utc::now(),
            id
        )
        .fetch_one(&self.db_pool)
        .await?;
        
        info!("Updated transaction status: {} to {:?}", id, status);
        Ok(TransactionResponse::from(transaction))
    }
    
    // Helper method to process a transaction on the blockchain
    async fn process_blockchain_transaction(
        &self,
        transaction: Transaction,
        sender_address: String,
        receiver_address: Option<String>,
    ) -> AppResult<()> {
        // Skip processing for pending transactions that don't have a real hash yet
        if transaction.tx_hash.starts_with("pending_") {
            tokio::spawn({
                let db_pool = self.db_pool.clone();
                let blockchain_client = self.blockchain_client.clone();
                let transaction = transaction.clone();
                let sender_address = sender_address.clone();
                let receiver_address = receiver_address.clone();
                
                async move {
                    // In a real implementation, this would create and submit a blockchain transaction
                    debug!("Processing blockchain transaction for transaction ID {}", transaction.id);
                    
                    // Simulate blockchain transaction processing
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    
                    // Generate a realistic transaction hash
                    let tx_hash = format!("tx_{}", Uuid::new_v4().to_string().replace("-", ""));
                    
                    // Update transaction record with hash and status
                    let result = sqlx::query!(
                        r#"
                        UPDATE transactions
                        SET tx_hash = $1, status = $2, updated_at = $3
                        WHERE id = $4
                        "#,
                        tx_hash,
                        TransactionStatus::Success as _,
                        Utc::now(),
                        transaction.id
                    )
                    .execute(&db_pool)
                    .await;
                    
                    if let Err(e) = result {
                        error!("Failed to update transaction status: {}", e);
                    } else {
                        info!("Transaction {} processed successfully with hash {}", transaction.id, tx_hash);
                    }
                }
            });
            
            return Ok(());
        }
        
        // For real blockchain transactions (not pending)
        match transaction.tx_type {
            TransactionType::Deposit => {
                self.process_deposit_transaction(&transaction, &sender_address, receiver_address).await?;
            },
            TransactionType::Withdraw => {
                self.process_withdraw_transaction(&transaction, &sender_address, receiver_address).await?;
            },
            TransactionType::Rebalance => {
                self.process_rebalance_transaction(&transaction, &sender_address, receiver_address).await?;
            },
            TransactionType::Claim => {
                self.process_claim_transaction(&transaction, &sender_address, receiver_address).await?;
            },
            TransactionType::Stake => {
                self.process_stake_transaction(&transaction, &sender_address, receiver_address).await?;
            },
            TransactionType::Unstake => {
                self.process_unstake_transaction(&transaction, &sender_address, receiver_address).await?;
            },
        }
        
        Ok(())
    }
    
    // Helper methods for specific transaction types
    async fn process_deposit_transaction(
        &self,
        transaction: &Transaction,
        sender_address: &str,
        receiver_address: Option<String>,
    ) -> AppResult<()> {
        // In a real implementation, this would create and submit a deposit transaction to the protocol's smart contract
        
        // For now, we'll simulate the transaction
        tokio::spawn({
            let db_pool = self.db_pool.clone();
            let transaction_id = transaction.id;
            
            async move {
                // Simulate blockchain confirmation delay
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                
                // Update transaction status to success
                let result = sqlx::query!(
                    r#"
                    UPDATE transactions
                    SET status = $1, updated_at = $2
                    WHERE id = $3
                    "#,
                    TransactionStatus::Success as _,
                    Utc::now(),
                    transaction_id
                )
                .execute(&db_pool)
                .await;
                
                if let Err(e) = result {
                    error!("Failed to update transaction status: {}", e);
                }
            }
        });
        
        Ok(())
    }
    
    async fn process_withdraw_transaction(
        &self,
        transaction: &Transaction,
        sender_address: &str,
        receiver_address: Option<String>,
    ) -> AppResult<()> {
        // In a real implementation, this would create and submit a withdrawal transaction from the protocol's smart contract
        
        // For now, we'll simulate the transaction
        tokio::spawn({
            let db_pool = self.db_pool.clone();
            let transaction_id = transaction.id;
            
            async move {
                // Simulate blockchain confirmation delay
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                
                // Update transaction status to success
                let result = sqlx::query!(
                    r#"
                    UPDATE transactions
                    SET status = $1, updated_at = $2
                    WHERE id = $3
                    "#,
                    TransactionStatus::Success as _,
                    Utc::now(),
                    transaction_id
                )
                .execute(&db_pool)
                .await;
                
                if let Err(e) = result {
                    error!("Failed to update transaction status: {}", e);
                }
            }
        });
        
        Ok(())
    }
    
    async fn process_rebalance_transaction(
        &self,
        transaction: &Transaction,
        sender_address: &str,
        receiver_address: Option<String>,
    ) -> AppResult<()> {
        // In a real implementation, this would create and submit a rebalance transaction to the protocol's smart contract
        
        // For now, we'll simulate the transaction
        tokio::spawn({
            let db_pool = self.db_pool.clone();
            let transaction_id = transaction.id;
            
            async move {
                // Simulate blockchain confirmation delay
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                
                // Update transaction status to success
                let result = sqlx::query!(
                    r#"
                    UPDATE transactions
                    SET status = $1, updated_at = $2
                    WHERE id = $3
                    "#,
                    TransactionStatus::Success as _,
                    Utc::now(),
                    transaction_id
                )
                .execute(&db_pool)
                .await;
                
                if let Err(e) = result {
                    error!("Failed to update transaction status: {}", e);
                }
            }
        });
        
        Ok(())
    }
    
    async fn process_claim_transaction(
        &self,
        transaction: &Transaction,
        sender_address: &str,
        receiver_address: Option<String>,
    ) -> AppResult<()> {
        // In a real implementation, this would create and submit a claim rewards transaction to the protocol's smart contract
        
        // For now, we'll simulate the transaction
        tokio::spawn({
            let db_pool = self.db_pool.clone();
            let transaction_id = transaction.id;
            
            async move {
                // Simulate blockchain confirmation delay
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                
                // Update transaction status to success
                let result = sqlx::query!(
                    r#"
                    UPDATE transactions
                    SET status = $1, updated_at = $2
                    WHERE id = $3
                    "#,
                    TransactionStatus::Success as _,
                    Utc::now(),
                    transaction_id
                )
                .execute(&db_pool)
                .await;
                
                if let Err(e) = result {
                    error!("Failed to update transaction status: {}", e);
                }
            }
        });
        
        Ok(())
    }
    
    async fn process_stake_transaction(
        &self,
        transaction: &Transaction,
        sender_address: &str,
        receiver_address: Option<String>,
    ) -> AppResult<()> {
        // In a real implementation, this would create and submit a stake transaction to the protocol's smart contract
        
        // For now, we'll simulate the transaction
        tokio::spawn({
            let db_pool = self.db_pool.clone();
            let transaction_id = transaction.id;
            
            async move {
                // Simulate blockchain confirmation delay
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                
                // Update transaction status to success
                let result = sqlx::query!(
                    r#"
                    UPDATE transactions
                    SET status = $1, updated_at = $2
                    WHERE id = $3
                    "#,
                    TransactionStatus::Success as _,
                    Utc::now(),
                    transaction_id
                )
                .execute(&db_pool)
                .await;
                
                if let Err(e) = result {
                    error!("Failed to update transaction status: {}", e);
                }
            }
        });
        
        Ok(())
    }
    
    async fn process_unstake_transaction(
        &self,
        transaction: &Transaction,
        sender_address: &str,
        receiver_address: Option<String>,
    ) -> AppResult<()> {
        // In a real implementation, this would create and submit an unstake transaction to the protocol's smart contract
        
        // For now, we'll simulate the transaction
        tokio::spawn({
            let db_pool = self.db_pool.clone();
            let transaction_id = transaction.id;
            
            async move {
                // Simulate blockchain confirmation delay
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                
                // Update transaction status to success
                let result = sqlx::query!(
                    r#"
                    UPDATE transactions
                    SET status = $1, updated_at = $2
                    WHERE id = $3
                    "#,
                    TransactionStatus::Success as _,
                    Utc::now(),
                    transaction_id
                )
                .execute(&db_pool)
                .await;
                
                if let Err(e) = result {
                    error!("Failed to update transaction status: {}", e);
                }
            }
        });
        
        Ok(())
    }
    
    // Helper method to update transaction statuses from blockchain
    async fn update_transaction_statuses(&self, transactions: Vec<Transaction>) -> AppResult<Vec<Transaction>> {
        let mut updated_transactions = Vec::with_capacity(transactions.len());
        
        for transaction in transactions {
            let updated_transaction = self.update_transaction_status(transaction).await?;
            updated_transactions.push(updated_transaction);
        }
        
        Ok(updated_transactions)
    }
    
    // Helper method to update a single transaction's status from blockchain
    async fn update_transaction_status(&self, transaction: Transaction) -> AppResult<Transaction> {
        // Skip pending transactions
        if transaction.tx_hash.starts_with("pending_") {
            return Ok(transaction);
        }
        
        // Skip transactions that are already in a final state
        match transaction.status {
            TransactionStatus::Success | TransactionStatus::Failed => return Ok(transaction),
            _ => {}
        }
        
        // In a real implementation, this would query the blockchain for the transaction status
        let blockchain_status = self.blockchain_client.get_transaction_status(&transaction.tx_hash).await?;
        
        // Only update in database if status has changed
        if blockchain_status != transaction.status {
            let updated_transaction = sqlx::query_as!(
                Transaction,
                r#"
                UPDATE transactions
                SET status = $1, updated_at = $2
                WHERE id = $3
                RETURNING 
                    id, user_id, tx_hash, tx_type as "tx_type: TransactionType",
                    amount, token, status as "status: TransactionStatus",
                    protocol_id, position_id, created_at, updated_at
                "#,
                blockchain_status as _,
                Utc::now(),
                transaction.id
            )
            .fetch_one(&self.db_pool)
            .await?;
            
            return Ok(updated_transaction);
        }
        
        Ok(transaction)
    }
    
    // Create a blockchain transaction with proper signing
    async fn create_blockchain_transaction(
        &self,
        sender: &str,
        receiver: &str,
        amount: &str,
        data: Option<String>,
        wallet: &Wallet,
    ) -> AppResult<String> {
        // Get sender account details
        let account = self.blockchain_client.get_account(sender).await?;
        
        // Prepare transaction
        let tx_request = TransactionRequest {
            nonce: account.nonce,
            value: amount.to_string(),
            receiver: receiver.to_string(),
            sender: sender.to_string(),
            gasPrice: self.blockchain_client.get_network_config().min_gas_price,
            gasLimit: self.blockchain_client.get_network_config().min_gas_limit,
            data: data,
            chainID: self.blockchain_client.get_network_config().chain_id.clone(),
            version: 1,
            signature: "".to_string(), // Will be filled below
        };
        
        // Serialize transaction for signing
        let tx_json = serde_json::to_string(&tx_request)?;
        
        // Sign transaction
        let signature = wallet.sign_transaction(tx_json.as_bytes());
        
        // Create final transaction with signature
        let final_tx = TransactionRequest {
            signature,
            ..tx_request
        };
        
        // Send transaction to blockchain
        let tx_hash = self.blockchain_client.send_transaction(final_tx).await?;
        
        Ok(tx_hash)
    }
    
    // Helper method to create a smart contract call
    fn create_contract_call_data(&self, function: &str, args: Vec<&str>) -> String {
        // In a real implementation, this would properly encode the function call for the MultiversX VM
        // For now, we'll just create a simple string representation
        let args_str = args.join(",");
        format!("{}@{}", function, args_str)
    }
}