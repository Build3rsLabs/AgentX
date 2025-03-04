use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Protocol {
    pub id: String,
    pub name: String,
    pub logo_url: String,
    pub description: String,
    pub tvl: f64,
    pub apy: f64,
    pub risk: RiskLevel,
    pub tokens: Vec<String>,
    pub website_url: String,
    pub contract_address: Option<String>,
    pub metadata: Json<HashMap<String, serde_json::Value>>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProtocolResponse {
    pub id: String,
    pub name: String,
    pub logo_url: String,
    pub description: String,
    pub tvl: f64,
    pub apy: f64,
    pub risk: String,
    pub tokens: Vec<String>,
    pub website_url: String,
}

impl From<Protocol> for ProtocolResponse {
    fn from(protocol: Protocol) -> Self {
        Self {
            id: protocol.id,
            name: protocol.name,
            logo_url: protocol.logo_url,
            description: protocol.description,
            tvl: protocol.tvl,
            apy: protocol.apy,
            risk: format!("{:?}", protocol.risk),
            tokens: protocol.tokens,
            website_url: protocol.website_url,
        }
    }
}