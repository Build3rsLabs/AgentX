use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use std::collections::HashMap;

use super::protocol::RiskLevel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pool {
    pub id: String,
    pub protocol_id: String,
    pub name: String,
    pub tvl: f64,
    pub apy: f64,
    pub tokens: Vec<String>,
    pub risk: RiskLevel,
    pub contract_address: Option<String>,
    pub metadata: Json<HashMap<String, serde_json::Value>>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolResponse {
    pub id: String,
    pub protocol_id: String,
    pub protocol_name: String,
    pub name: String,
    pub tvl: f64,
    pub apy: f64,
    pub tokens: Vec<String>,
    pub risk: String,
}

impl Pool {
    pub fn to_response(&self, protocol_name: String) -> PoolResponse {
        PoolResponse {
            id: self.id.clone(),
            protocol_id: self.protocol_id.clone(),
            protocol_name,
            name: self.name.clone(),
            tvl: self.tvl,
            apy: self.apy,
            tokens: self.tokens.clone(),
            risk: format!("{:?}", self.risk),
        }
    }
}