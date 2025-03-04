use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub blockchain: BlockchainConfig,
    pub api: ApiConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BlockchainConfig {
    pub network: String,
    pub gateway_url: String,
    pub chain_id: String,
    pub min_gas_price: u64,
    pub min_gas_limit: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiConfig {
    pub jwt_secret: String,
    pub token_expiration: u64, // in seconds
}

impl AppConfig {
    pub fn development() -> Self {
        Self {
            database: DatabaseConfig {
                url: env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/agentx_dev".to_string()),
                max_connections: 5,
            },
            blockchain: BlockchainConfig {
                network: "devnet".to_string(),
                gateway_url: "https://devnet-gateway.multiversx.com".to_string(),
                chain_id: "D".to_string(),
                min_gas_price: 1000000000,
                min_gas_limit: 50000,
            },
            api: ApiConfig {
                jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| "development_secret_key".to_string()),
                token_expiration: 86400, // 24 hours
            },
        }
    }

    pub fn production() -> Self {
        Self {
            database: DatabaseConfig {
                url: env::var("DATABASE_URL").expect("DATABASE_URL must be set in production"),
                max_connections: 20,
            },
            blockchain: BlockchainConfig {
                network: env::var("BLOCKCHAIN_NETWORK").unwrap_or_else(|_| "mainnet".to_string()),
                gateway_url: env::var("GATEWAY_URL").unwrap_or_else(|_| "https://gateway.multiversx.com".to_string()),
                chain_id: env::var("CHAIN_ID").unwrap_or_else(|_| "1".to_string()),
                min_gas_price: 1000000000,
                min_gas_limit: 50000,
            },
            api: ApiConfig {
                jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set in production"),
                token_expiration: env::var("TOKEN_EXPIRATION")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(3600), // 1 hour
            },
        }
    }
}