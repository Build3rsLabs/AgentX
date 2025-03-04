use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Deposit,
    Withdraw,
    Rebalance,
    Claim,
    Stake,
    Unstake,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Success,
    Failed,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub tx_hash: String,
    pub tx_type: TransactionType,
    pub amount: Option<String>,
    pub token: Option<String>,
    pub status: TransactionStatus,
    pub protocol_id: Option<String>,
    pub position_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransactionRequest {
    pub user_id: Uuid,
    pub tx_type: TransactionType,
    pub amount: Option<String>,
    pub token: Option<String>,
    pub protocol_id: Option<String>,
    pub position_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub id: Uuid,
    pub tx_hash: String,
    pub tx_type: String,
    pub amount: Option<String>,
    pub token: Option<String>,
    pub status: String,
    pub protocol: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<Transaction> for TransactionResponse {
    fn from(tx: Transaction) -> Self {
        Self {
            id: tx.id,
            tx_hash: tx.tx_hash,
            tx_type: format!("{:?}", tx.tx_type),
            amount: tx.amount,
            token: tx.token,
            status: format!("{:?}", tx.status),
            protocol: tx.protocol_id,
            created_at: tx.created_at,
        }
    }
}