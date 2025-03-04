use crate::config::AppConfig;
use crate::error::{AppError, AppResult};
use crate::models::transaction::{Transaction, TransactionStatus};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{debug, error, info};

#[derive(Debug, Clone)]
pub struct MultiversXClient {
    client: Client,
    gateway_url: String,
    chain_id: String,
    min_gas_price: u64,
    min_gas_limit: u64,
}

#[derive(Debug, Deserialize)]
struct AccountResponse {
    data: AccountData,
}

#[derive(Debug, Deserialize)]
struct AccountData {
    account: Account,
}

#[derive(Debug, Deserialize)]
pub struct Account {
    pub address: String,
    pub balance: String,
    pub nonce: u64,
}

#[derive(Debug, Serialize)]
pub struct TransactionRequest {
    pub nonce: u64,
    pub value: String,
    pub receiver: String,
    pub sender: String,
    pub gasPrice: u64,
    pub gasLimit: u64,
    pub data: Option<String>,
    pub chainID: String,
    pub version: u32,
    pub signature: String,
}

#[derive(Debug, Deserialize)]
struct TransactionResponse {
    data: TransactionData,
}

#[derive(Debug, Deserialize)]
struct TransactionData {
    txHash: String,
}

#[derive(Debug, Deserialize)]
struct NetworkStatusResponse {
    data: NetworkStatusData,
}

#[derive(Debug, Deserialize)]
struct NetworkStatusData {
    status: NetworkStatus,
}

#[derive(Debug, Deserialize)]
struct NetworkStatus {
    erd_current_round: u64,
    erd_epoch_number: u64,
    erd_highest_final_nonce: u64,
    erd_nonce: u64,
}

#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub chain_id: String,
    pub min_gas_price: u64,
    pub min_gas_limit: u64,
}

#[async_trait]
pub trait BlockchainProvider {
    async fn get_account(&self, address: &str) -> AppResult<Account>;
    async fn get_balance(&self, address: &str) -> AppResult<String>;
    async fn get_nonce(&self, address: &str) -> AppResult<u64>;
    async fn send_transaction(&self, tx: TransactionRequest) -> AppResult<String>;
    async fn get_transaction_status(&self, tx_hash: &str) -> AppResult<TransactionStatus>;
    async fn get_network_status(&self) -> AppResult<NetworkStatus>;
    fn get_network_config(&self) -> NetworkConfig;
}

impl MultiversXClient {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            client: Client::new(),
            gateway_url: config.blockchain.gateway_url.clone(),
            chain_id: config.blockchain.chain_id.clone(),
            min_gas_price: config.blockchain.min_gas_price,
            min_gas_limit: config.blockchain.min_gas_limit,
        }
    }
    
    pub fn get_network_config(&self) -> NetworkConfig {
        NetworkConfig {
            chain_id: self.chain_id.clone(),
            min_gas_price: self.min_gas_price,
            min_gas_limit: self.min_gas_limit,
        }
    }
}

#[async_trait]
impl BlockchainProvider for MultiversXClient {
    async fn get_account(&self, address: &str) -> AppResult<Account> {
        let url = format!("{}/address/{}", self.gateway_url, address);
        debug!("Fetching account data from {}", url);

        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("Failed to get account: {}", error_text);
            return Err(AppError::Blockchain(format!("Failed to get account: {}", error_text)));
        }

        let account_response: AccountResponse = response.json().await?;
        Ok(account_response.data.account)
    }

    async fn get_balance(&self, address: &str) -> AppResult<String> {
        let account = self.get_account(address).await?;
        Ok(account.balance)
    }

    async fn get_nonce(&self, address: &str) -> AppResult<u64> {
        let account = self.get_account(address).await?;
        Ok(account.nonce)
    }

    async fn send_transaction(&self, tx: TransactionRequest) -> AppResult<String> {
        let url = format!("{}/transaction/send", self.gateway_url);
        debug!("Sending transaction to {}", url);

        let response = self.client.post(&url).json(&tx).send().await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("Failed to send transaction: {}", error_text);
            return Err(AppError::Transaction(format!("Failed to send transaction: {}", error_text)));
        }

        let tx_response: TransactionResponse = response.json().await?;
        info!("Transaction sent with hash: {}", tx_response.data.txHash);
        Ok(tx_response.data.txHash)
    }

    async fn get_transaction_status(&self, tx_hash: &str) -> AppResult<TransactionStatus> {
        let url = format!("{}/transaction/{}", self.gateway_url, tx_hash);
        debug!("Checking transaction status from {}", url);

        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("Failed to get transaction status: {}", error_text);
            return Err(AppError::Transaction(format!("Failed to get transaction status: {}", error_text)));
        }

        #[derive(Deserialize)]
        struct TxStatusResponse {
            data: TxStatusData,
        }

        #[derive(Deserialize)]
        struct TxStatusData {
            transaction: TxStatus,
        }

        #[derive(Deserialize)]
        struct TxStatus {
            status: String,
        }

        let status_response: TxStatusResponse = response.json().await?;
        let status_str = status_response.data.transaction.status;
        
        match status_str.as_str() {
            "pending" => Ok(TransactionStatus::Pending),
            "success" | "executed" => Ok(TransactionStatus::Success),
            "fail" | "invalid" => Ok(TransactionStatus::Failed),
            _ => Ok(TransactionStatus::Unknown),
        }
    }
    
    async fn get_network_status(&self) -> AppResult<NetworkStatus> {
        let url = format!("{}/network/status", self.gateway_url);
        debug!("Fetching network status from {}", url);

        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("Failed to get network status: {}", error_text);
            return Err(AppError::Blockchain(format!("Failed to get network status: {}", error_text)));
        }

        let status_response: NetworkStatusResponse = response.json().await?;
        Ok(status_response.data.status)
    }
    
    fn get_network_config(&self) -> NetworkConfig {
        NetworkConfig {
            chain_id: self.chain_id.clone(),
            min_gas_price: self.min_gas_price,
            min_gas_limit: self.min_gas_limit,
        }
    }
}

// Helper function for getting account balance
pub async fn get_account_balance(client: &MultiversXClient, address: &str) -> AppResult<f64> {
    let balance_str = client.get_balance(address).await?;
    
    // Convert from denominated units (10^18) to EGLD
    let balance_value = balance_str.parse::<u128>().map_err(|e| {
        AppError::Blockchain(format!("Failed to parse balance: {}", e))
    })?;
    
    // Convert to EGLD (1 EGLD = 10^18 smallest units)
    let egld_balance = balance_value as f64 / 1_000_000_000_000_000_000.0;
    
    Ok(egld_balance)
}