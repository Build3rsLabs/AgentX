use crate::error::AppResult;
use crate::models::position::PositionStrategy;
use crate::smart_contracts::ProtocolRegistry;
use std::collections::HashMap;
use tracing::{info, debug};

pub struct YieldOpportunity {
    pub protocol_id: String,
    pub protocol_name: String,
    pub pool_id: String,
    pub pool_name: String,
    pub apy: f64,
    pub tvl: f64,
    pub risk: String,
    pub tokens: Vec<String>,
}

pub struct YieldOptimizerService {
    protocol_registry: ProtocolRegistry,
}

impl YieldOptimizerService {
    pub fn new(protocol_registry: ProtocolRegistry) -> Self {
        Self {
            protocol_registry,
        }
    }
    
    // Find the best yield opportunities across all protocols
    pub async fn find_best_opportunities(&self, 
                                        strategy: &PositionStrategy, 
                                        token: Option<String>,
                                        limit: usize) -> AppResult<Vec<YieldOpportunity>> {
        debug!("Finding best yield opportunities for strategy {:?}, token: {:?}", strategy, token);
        
        let mut opportunities = Vec::new();
        
        // Iterate through all protocols
        for protocol in self.protocol_registry.get_all_protocols() {
            let protocol_id = protocol.get_id().to_string();
            let protocol_name = protocol.get_name().to_string();
            
            // Get pools for this protocol
            let pools = protocol.get_pools().await?;
            
            // Check each pool
            for pool_id in pools {
                // Get pool details
                let apy = protocol.get_pool_apy(&pool_id).await?;
                let tvl = protocol.get_pool_tvl(&pool_id).await?;
                let risk = protocol.get_risk_level().to_string();
                
                // Get supported tokens for this pool
                let tokens = protocol.get_supported_tokens().await?;
                
                // Filter by token if specified
                if let Some(ref token_filter) = token {
                    if !tokens.contains(token_filter) {
                        continue;
                    }
                }
                
                // Create opportunity
                let opportunity = YieldOpportunity {
                    protocol_id: protocol_id.clone(),
                    protocol_name: protocol_name.clone(),
                    pool_id: pool_id.clone(),
                    pool_name: pool_id.clone(), // In a real implementation, we would get the actual pool name
                    apy,
                    tvl,
                    risk: risk.clone(),
                    tokens,
                };
                
                opportunities.push(opportunity);
            }
        }
        
        // Filter opportunities based on strategy
        self.filter_by_strategy(&mut opportunities, strategy);
        
        // Sort by APY (descending)
        opportunities.sort_by(|a, b| b.apy.partial_cmp(&a.apy).unwrap_or(std::cmp::Ordering::Equal));
        
        // Limit results
        if opportunities.len() > limit {
            opportunities.truncate(limit);
        }
        
        Ok(opportunities)
    }
    
    // Filter opportunities based on strategy
    fn filter_by_strategy(&self, opportunities: &mut Vec<YieldOpportunity>, strategy: &PositionStrategy) {
        match strategy {
            PositionStrategy::Conservative => {
                // Keep only low risk opportunities
                opportunities.retain(|o| o.risk == "Low");
            },
            PositionStrategy::Balanced => {
                // Keep low and medium risk opportunities
                opportunities.retain(|o| o.risk == "Low" || o.risk == "Medium");
            },
            PositionStrategy::Aggressive => {
                // Keep all opportunities (no filtering)
            },
        }
    }
    
    // Calculate optimal portfolio allocation based on strategy
    pub async fn calculate_optimal_allocation(&self, 
                                             strategy: &PositionStrategy,
                                             investment_amount: f64) -> AppResult<HashMap<String, f64>> {
        debug!("Calculating optimal allocation for strategy {:?}, amount: {}", strategy, investment_amount);
        
        let mut allocation = HashMap::new();
        
        // Get best opportunities
        let opportunities = self.find_best_opportunities(strategy, None, 5).await?;
        
        match strategy {
            PositionStrategy::Conservative => {
                // Conservative allocation: 70% in lowest risk, 30% in second lowest risk
                if opportunities.len() >= 2 {
                    allocation.insert(format!("{}:{}", opportunities[0].protocol_id, opportunities[0].pool_id), 
                                     investment_amount * 0.7);
                    allocation.insert(format!("{}:{}", opportunities[1].protocol_id, opportunities[1].pool_id), 
                                     investment_amount * 0.3);
                } else if opportunities.len() == 1 {
                    allocation.insert(format!("{}:{}", opportunities[0].protocol_id, opportunities[0].pool_id), 
                                     investment_amount);
                }
            },
            PositionStrategy::Balanced => {
                // Balanced allocation: 50% in highest APY, 30% in second, 20% in third
                if opportunities.len() >= 3 {
                    allocation.insert(format!("{}:{}", opportunities[0].protocol_id, opportunities[0].pool_id), 
                                     investment_amount * 0.5);
                    allocation.insert(format!("{}:{}", opportunities[1].protocol_id, opportunities[1].pool_id), 
                                     investment_amount * 0.3);
                    allocation.insert(format!("{}:{}", opportunities[2].protocol_id, opportunities[2].pool_id), 
                                     investment_amount * 0.2);
                } else if opportunities.len() == 2 {
                    allocation.insert(format!("{}:{}", opportunities[0].protocol_id, opportunities[0].pool_id), 
                                     investment_amount * 0.7);
                    allocation.insert(format!("{}:{}", opportunities[1].protocol_id, opportunities[1].pool_id), 
                                     investment_amount * 0.3);
                } else if opportunities.len() == 1 {
                    allocation.insert(format!("{}:{}", opportunities[0].protocol_id, opportunities[0].pool_id), 
                                     investment_amount);
                }
            },
            PositionStrategy::Aggressive => {
                // Aggressive allocation: 80% in highest APY, 20% in second
                if opportunities.len() >= 2 {
                    allocation.insert(format!("{}:{}", opportunities[0].protocol_id, opportunities[0].pool_id), 
                                     investment_amount * 0.8);
                    allocation.insert(format!("{}:{}", opportunities[1].protocol_id, opportunities[1].pool_id), 
                                     investment_amount * 0.2);
                } else if opportunities.len() == 1 {
                    allocation.insert(format!("{}:{}", opportunities[0].protocol_id, opportunities[0].pool_id), 
                                     investment_amount);
                }
            },
        }
        
        Ok(allocation)
    }
    
    // Check if rebalancing is needed for a position
    pub async fn should_rebalance(&self, 
                                 current_allocation: &HashMap<String, f64>,
                                 strategy: &PositionStrategy,
                                 total_value: f64) -> AppResult<bool> {
        // Calculate optimal allocation
        let optimal_allocation = self.calculate_optimal_allocation(strategy, total_value).await?;
        
        // Compare current allocation with optimal allocation
        let mut total_deviation = 0.0;
        
        for (key, optimal_amount) in &optimal_allocation {
            let current_amount = current_allocation.get(key).cloned().unwrap_or(0.0);
            let deviation = (current_amount - optimal_amount).abs() / total_value;
            total_deviation += deviation;
        }
        
        // If total deviation is more than 10%, rebalancing is needed
        Ok(total_deviation > 0.1)
    }
    
    // Calculate expected APY for a given allocation
    pub async fn calculate_expected_apy(&self, 
                                       allocation: &HashMap<String, f64>) -> AppResult<f64> {
        let mut total_amount = 0.0;
        let mut weighted_apy = 0.0;
        
        for (key, amount) in allocation {
            let parts: Vec<&str> = key.split(':').collect();
            if parts.len() != 2 {
                continue;
            }
            
            let protocol_id = parts[0];
            let pool_id = parts[1];
            
            if let Some(protocol) = self.protocol_registry.get_protocol(protocol_id) {
                let apy = protocol.get_pool_apy(pool_id).await?;
                weighted_apy += apy * amount;
                total_amount += amount;
            }
        }
        
        if total_amount > 0.0 {
            Ok(weighted_apy / total_amount)
        } else {
            Ok(0.0)
        }
    }
}