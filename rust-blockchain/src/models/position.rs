use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::{Uuid, Json};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PositionStrategy {
    Conservative,
    Balanced,
    Aggressive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RebalanceFrequency {
    Daily,
    Weekly,
    Monthly,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenAllocation {
    pub token: String,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub id: Uuid,
    pub user_id: Uuid,
    pub protocol_id: String,
    pub name: String,
    pub position_type: String,
    pub tokens: Vec<String>,
    pub deposited: f64,
    pub current_value: f64,
    pub apy: f64,
    pub strategy: PositionStrategy,
    pub entry_date: DateTime<Utc>,
    pub last_rebalance: DateTime<Utc>,
    pub rebalance_frequency: RebalanceFrequency,
    pub allocation: Vec<TokenAllocation>,
    pub metadata: Json<HashMap<String, serde_json::Value>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePositionRequest {
    pub protocol_id: String,
    pub name: String,
    pub position_type: String,
    pub tokens: Vec<String>,
    pub amount: f64,
    pub strategy: PositionStrategy,
    pub rebalance_frequency: RebalanceFrequency,
    pub allocation: Vec<TokenAllocation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePositionRequest {
    pub current_value: Option<f64>,
    pub apy: Option<f64>,
    pub strategy: Option<PositionStrategy>,
    pub rebalance_frequency: Option<RebalanceFrequency>,
    pub allocation: Option<Vec<TokenAllocation>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositionResponse {
    pub id: Uuid,
    pub protocol_id: String,
    pub name: String,
    pub position_type: String,
    pub tokens: Vec<String>,
    pub deposited: f64,
    pub current_value: f64,
    pub apy: f64,
    pub strategy: String,
    pub entry_date: DateTime<Utc>,
    pub last_rebalance: DateTime<Utc>,
    pub rebalance_frequency: String,
    pub allocation: Vec<TokenAllocation>,
    pub created_at: DateTime<Utc>,
}

impl From<Position> for PositionResponse {
    fn from(position: Position) -> Self {
        Self {
            id: position.id,
            protocol_id: position.protocol_id,
            name: position.name,
            position_type: position.position_type,
            tokens: position.tokens,
            deposited: position.deposited,
            current_value: position.current_value,
            apy: position.apy,
            strategy: format!("{:?}", position.strategy),
            entry_date: position.entry_date,
            last_rebalance: position.last_rebalance,
            rebalance_frequency: format!("{:?}", position.rebalance_frequency),
            allocation: position.allocation,
            created_at: position.created_at,
        }
    }
}