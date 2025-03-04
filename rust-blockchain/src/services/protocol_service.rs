use crate::error::{AppError, AppResult};
use crate::models::pool::{Pool, PoolResponse};
use crate::models::protocol::{Protocol, ProtocolResponse, RiskLevel};
use crate::smart_contracts::ProtocolRegistry;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::{info, error};

pub struct ProtocolService {
    db_pool: PgPool,
    protocol_registry: ProtocolRegistry,
}

impl ProtocolService {
    pub fn new(db_pool: PgPool, protocol_registry: ProtocolRegistry) -> Self {
        Self { db_pool, protocol_registry }
    }
    
    pub async fn get_all_protocols(&self) -> AppResult<Vec<ProtocolResponse>> {
        // First, get protocols from database
        let protocols = sqlx::query_as!(
            Protocol,
            r#"
            SELECT 
                id, name, logo_url, description, tvl, apy, 
                risk as "risk: RiskLevel", tokens, website_url, 
                contract_address, metadata, is_active
            FROM protocols
            WHERE is_active = true
            ORDER BY tvl DESC
            "#
        )
        .fetch_all(&self.db_pool)
        .await?;
        
        // Then, update with real-time data from blockchain
        let mut updated_protocols = Vec::with_capacity(protocols.len());
        
        for protocol in protocols {
            // Try to get real-time data from blockchain
            if let Some(blockchain_protocol) = self.protocol_registry.get_protocol(&protocol.id) {
                // Get real-time TVL and APY
                let tvl = blockchain_protocol.get_tvl().await.unwrap_or(protocol.tvl);
                let apy = blockchain_protocol.get_apy().await.unwrap_or(protocol.apy);
                
                // Create updated protocol
                let updated_protocol = Protocol {
                    tvl,
                    apy,
                    ..protocol
                };
                
                updated_protocols.push(ProtocolResponse::from(updated_protocol));
            } else {
                // If not available in blockchain, use database data
                updated_protocols.push(ProtocolResponse::from(protocol));
            }
        }
        
        Ok(updated_protocols)
    }
    
    pub async fn get_protocol_by_id(&self, id: &str) -> AppResult<ProtocolResponse> {
        // First, get protocol from database
        let protocol = sqlx::query_as!(
            Protocol,
            r#"
            SELECT 
                id, name, logo_url, description, tvl, apy, 
                risk as "risk: RiskLevel", tokens, website_url, 
                contract_address, metadata, is_active
            FROM protocols
            WHERE id = $1 AND is_active = true
            "#,
            id
        )
        .fetch_one(&self.db_pool)
        .await?;
        
        // Then, update with real-time data from blockchain
        if let Some(blockchain_protocol) = self.protocol_registry.get_protocol(id) {
            // Get real-time TVL and APY
            let tvl = blockchain_protocol.get_tvl().await.unwrap_or(protocol.tvl);
            let apy = blockchain_protocol.get_apy().await.unwrap_or(protocol.apy);
            
            // Create updated protocol
            let updated_protocol = Protocol {
                tvl,
                apy,
                ..protocol
            };
            
            Ok(ProtocolResponse::from(updated_protocol))
        } else {
            // If not available in blockchain, use database data
            Ok(ProtocolResponse::from(protocol))
        }
    }
    
    pub async fn get_pools_by_protocol(&self, protocol_id: &str) -> AppResult<Vec<PoolResponse>> {
        // First, get protocol from database to get the name
        let protocol = sqlx::query_as!(
            Protocol,
            r#"
            SELECT 
                id, name, logo_url, description, tvl, apy, 
                risk as "risk: RiskLevel", tokens, website_url, 
                contract_address, metadata, is_active
            FROM protocols
            WHERE id = $1 AND is_active = true
            "#,
            protocol_id
        )
        .fetch_one(&self.db_pool)
        .await?;
        
        // Get pools from database
        let pools = sqlx::query_as!(
            Pool,
            r#"
            SELECT 
                id, protocol_id, name, tvl, apy, tokens,
                risk as "risk: RiskLevel", contract_address, metadata, is_active
            FROM pools
            WHERE protocol_id = $1 AND is_active = true
            ORDER BY apy DESC
            "#,
            protocol_id
        )
        .fetch_all(&self.db_pool)
        .await?;
        
        // Update with real-time data from blockchain
        let mut updated_pools = Vec::with_capacity(pools.len());
        
        if let Some(blockchain_protocol) = self.protocol_registry.get_protocol(protocol_id) {
            for pool in pools {
                // Try to get real-time data for this pool
                let pool_id = &pool.id;
                
                // Get real-time TVL and APY if available
                let tvl = blockchain_protocol.get_pool_tvl(pool_id).await.unwrap_or(pool.tvl);
                let apy = blockchain_protocol.get_pool_apy(pool_id).await.unwrap_or(pool.apy);
                
                // Create updated pool
                let updated_pool = Pool {
                    tvl,
                    apy,
                    ..pool
                };
                
                updated_pools.push(updated_pool.to_response(protocol.name.clone()));
            }
        } else {
            // If protocol not available in blockchain, use database data
            for pool in pools {
                updated_pools.push(pool.to_response(protocol.name.clone()));
            }
        }
        
        Ok(updated_pools)
    }
    
    pub async fn get_all_pools(&self) -> AppResult<Vec<PoolResponse>> {
        // Get all protocols to map protocol_id to name
        let protocols = sqlx::query_as!(
            Protocol,
            r#"
            SELECT 
                id, name, logo_url, description, tvl, apy, 
                risk as "risk: RiskLevel", tokens, website_url, 
                contract_address, metadata, is_active
            FROM protocols
            WHERE is_active = true
            "#
        )
        .fetch_all(&self.db_pool)
        .await?;
        
        let protocol_map: HashMap<String, String> = protocols
            .into_iter()
            .map(|p| (p.id, p.name))
            .collect();
        
        // Get all pools from database
        let pools = sqlx::query_as!(
            Pool,
            r#"
            SELECT 
                id, protocol_id, name, tvl, apy, tokens,
                risk as "risk: RiskLevel", contract_address, metadata, is_active
            FROM pools
            WHERE is_active = true
            ORDER BY apy DESC
            "#
        )
        .fetch_all(&self.db_pool)
        .await?;
        
        // Update with real-time data from blockchain
        let mut updated_pools = Vec::with_capacity(pools.len());
        
        for pool in pools {
            let protocol_id = &pool.protocol_id;
            let protocol_name = protocol_map.get(protocol_id).cloned().unwrap_or_default();
            
            // Try to get real-time data for this pool
            if let Some(blockchain_protocol) = self.protocol_registry.get_protocol(protocol_id) {
                let pool_id = &pool.id;
                
                // Get real-time TVL and APY if available
                let tvl = blockchain_protocol.get_pool_tvl(pool_id).await.unwrap_or(pool.tvl);
                let apy = blockchain_protocol.get_pool_apy(pool_id).await.unwrap_or(pool.apy);
                
                // Create updated pool
                let updated_pool = Pool {
                    tvl,
                    apy,
                    ..pool
                };
                
                updated_pools.push(updated_pool.to_response(protocol_name));
            } else {
                // If protocol not available in blockchain, use database data
                updated_pools.push(pool.to_response(protocol_name));
            }
        }
        
        Ok(updated_pools)
    }
    
    pub async fn get_pool_by_id(&self, id: &str) -> AppResult<PoolResponse> {
        // Get pool from database
        let pool = sqlx::query_as!(
            Pool,
            r#"
            SELECT 
                id, protocol_id, name, tvl, a py, tokens,
                risk as "risk: RiskLevel", contract_address, metadata, is_active
            FROM pools
            WHERE id = $1 AND is_active = true
            "#,
            id
        )
        .fetch_one(&self.db_pool)
        .await?;
        
        // Get protocol name
        let protocol = sqlx::query_as!(
            Protocol,
            r#"
            SELECT 
                id, name, logo_url, description, tvl, apy, 
                risk as "risk: RiskLevel", tokens, website_url, 
                contract_address, metadata, is_active
            FROM protocols
            WHERE id = $1 AND is_active = true
            "#,
            pool.protocol_id
        )
        .fetch_one(&self.db_pool)
        .await?;
        
        // Update with real-time data from blockchain
        if let Some(blockchain_protocol) = self.protocol_registry.get_protocol(&pool.protocol_id) {
            let pool_id = &pool.id;
            
            // Get real-time TVL and APY if available
            let tvl = blockchain_protocol.get_pool_tvl(pool_id).await.unwrap_or(pool.tvl);
            let apy = blockchain_protocol.get_pool_apy(pool_id).await.unwrap_or(pool.apy);
            
            // Create updated pool
            let updated_pool = Pool {
                tvl,
                apy,
                ..pool
            };
            
            Ok(updated_pool.to_response(protocol.name))
        } else {
            // If protocol not available in blockchain, use database data
            Ok(pool.to_response(protocol.name))
        }
    }
}