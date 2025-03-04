let (score, opp) = scored_opps[i];
                    let allocation_amount = investment_amount * (score / total_score);
                    let key = format!("{}:{}", opp.protocol_id, opp.pool_id);
                    allocation.insert(key, allocation_amount);
                }
            },
            PositionStrategy::Aggressive => {
                // Aggressive allocation prioritizes yield over safety
                // Allocate primarily to highest APY opportunities
                
                // Sort by APY
                let mut sorted_opps = opportunities.to_vec();
                sorted_opps.sort_by(|a, b| b.apy.partial_cmp(&a.apy).unwrap_or(std::cmp::Ordering::Equal));
                
                // Allocate 80% to highest APY, 20% to second highest
                if sorted_opps.len() >= 2 {
                    let key1 = format!("{}:{}", sorted_opps[0].protocol_id, sorted_opps[0].pool_id);
                    let key2 = format!("{}:{}", sorted_opps[1].protocol_id, sorted_opps[1].pool_id);
                    allocation.insert(key1, investment_amount * 0.8);
                    allocation.insert(key2, investment_amount * 0.2);
                } else if sorted_opps.len() == 1 {
                    let key = format!("{}:{}", sorted_opps[0].protocol_id, sorted_opps[0].pool_id);
                    allocation.insert(key, investment_amount);
                }
            },
        }
        
        // Validate allocation against risk profile
        self.validate_allocation(&mut allocation, &risk_profile, opportunities);
        
        Ok(allocation)
    }
    
    /// Validate and adjust allocation to meet risk profile constraints
    fn validate_allocation(
        &self,
        allocation: &mut HashMap<String, f64>,
        risk_profile: &RiskProfile,
        opportunities: &[YieldOpportunity],
    ) {
        // Check if we need to adjust allocation
        let total_amount: f64 = allocation.values().sum();
        if total_amount <= 0.0 {
            return;
        }
        
        // Check protocol allocation limits
        let mut protocol_allocations: HashMap<String, f64> = HashMap::new();
        for (key, amount) in allocation.iter() {
            let protocol_id = key.split(':').next().unwrap_or("").to_string();
            *protocol_allocations.entry(protocol_id).or_insert(0.0) += amount;
        }
        
        let mut needs_rebalance = false;
        
        for (protocol_id, amount) in protocol_allocations.iter() {
            if amount / total_amount > risk_profile.max_protocol_allocation {
                needs_rebalance = true;
                break;
            }
        }
        
        // Check token allocation limits
        let mut token_allocations: HashMap<String, f64> = HashMap::new();
        for (key, amount) in allocation.iter() {
            let parts: Vec<&str> = key.split(':').collect();
            if parts.len() != 2 {
                continue;
            }
            
            let protocol_id = parts[0];
            let pool_id = parts[1];
            
            // Find the opportunity
            if let Some(opp) = opportunities.iter().find(|o| o.protocol_id == protocol_id && o.pool_id == pool_id) {
                for token in &opp.tokens {
                    *token_allocations.entry(token.clone()).or_insert(0.0) += amount;
                }
            }
        }
        
        for (_, amount) in token_allocations.iter() {
            if amount / total_amount > risk_profile.max_token_allocation {
                needs_rebalance = true;
                break;
            }
        }
        
        // If we need to rebalance, do a simple equal allocation across all opportunities
        if needs_rebalance {
            allocation.clear();
            
            let num_opps = opportunities.len().min(risk_profile.min_protocol_count.max(3));
            let allocation_per_opp = total_amount / num_opps as f64;
            
            for i in 0..num_opps {
                let opp = &opportunities[i];
                let key = format!("{}:{}", opp.protocol_id, opp.pool_id);
                allocation.insert(key, allocation_per_opp);
            }
        }
    }
    
    /// Calculate expected APY for a given allocation
    pub async fn calculate_expected_apy(
        &self,
        allocation: &HashMap<String, f64>,
    ) -> AppResult<f64> {
        let mut total_amount = 0.0;
        let mut weighted_apy = 0.0;
        
        for (key, amount) in allocation {
            let parts: Vec<&str> = key.split(':').collect();
            if parts.len() != 2 {
                continue;
            }
            
            let protocol_id = parts[0];
            let pool_id = parts[1];
            
            // Find the protocol adapter
            if let Some(adapter) = self.protocol_adapters.iter().find(|a| a.get_id() == protocol_id) {
                let apy = adapter.get_pool_apy(pool_id).await?;
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
    
    /// Check if rebalancing is needed for a position
    pub async fn should_rebalance(
        &self,
        current_allocation: &HashMap<String, f64>,
        strategy: &PositionStrategy,
        total_value: f64,
    ) -> AppResult<bool> {
        // Get best opportunities
        let opportunities = self.find_best_opportunities(strategy, None, 10).await?;
        
        // Calculate optimal allocation
        let optimal_allocation = self.calculate_optimal_allocation(
            strategy,
            total_value,
            &opportunities,
        ).await?;
        
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
    
    /// Calculate risk metrics for a given allocation
    pub async fn calculate_risk_metrics(
        &self,
        allocation: &HashMap<String, f64>,
        opportunities: &[YieldOpportunity],
    ) -> AppResult<RiskMetrics> {
        let mut total_amount = 0.0;
        let mut weighted_volatility = 0.0;
        let mut weighted_liquidity = 0.0;
        let mut weighted_security = 0.0;
        
        let mut protocol_allocations: HashMap<String, f64> = HashMap::new();
        let mut token_allocations: HashMap<String, f64> = HashMap::new();
        
        for (key, amount) in allocation {
            let parts: Vec<&str> = key.split(':').collect();
            if parts.len() != 2 {
                continue;
            }
            
            let protocol_id = parts[0];
            let pool_id = parts[1];
            
            // Update protocol allocations
            *protocol_allocations.entry(protocol_id.to_string()).or_insert(0.0) += amount;
            
            // Find the opportunity
            if let Some(opp) = opportunities.iter().find(|o| o.protocol_id == protocol_id && o.pool_id == pool_id) {
                weighted_volatility += opp.volatility as f64 * amount;
                weighted_liquidity += opp.liquidity as f64 * amount;
                weighted_security += opp.security as f64 * amount;
                
                // Update token allocations
                for token in &opp.tokens {
                    *token_allocations.entry(token.clone()).or_insert(0.0) += amount;
                }
            }
            
            total_amount += amount;
        }
        
        if total_amount <= 0.0 {
            return Ok(RiskMetrics::default());
        }
        
        // Calculate average metrics
        let avg_volatility = weighted_volatility / total_amount;
        let avg_liquidity = weighted_liquidity / total_amount;
        let avg_security = weighted_security / total_amount;
        
        // Calculate concentration metrics
        let max_protocol_allocation = protocol_allocations.values()
            .fold(0.0, |max, &val| max.max(val)) / total_amount;
        
        let max_token_allocation = token_allocations.values()
            .fold(0.0, |max, &val| max.max(val)) / total_amount;
        
        let protocol_count = protocol_allocations.len();
        
        Ok(RiskMetrics {
            volatility: avg_volatility as u8,
            liquidity: avg_liquidity as u8,
            security: avg_security as u8,
            max_protocol_allocation,
            max_token_allocation,
            protocol_count,
        })
    }
}

/// Risk metrics for a portfolio allocation
#[derive(Debug, Clone)]
pub struct RiskMetrics {
    /// Average volatility score (0-100)
    pub volatility: u8,
    /// Average liquidity score (0-100)
    pub liquidity: u8,
    /// Average security score (0-100)
    pub security: u8,
    /// Maximum allocation to a single protocol (0.0-1.0)
    pub max_protocol_allocation: f64,
    /// Maximum allocation to a single token (0.0-1.0)
    pub max_token_allocation: f64,
    /// Number of protocols in the allocation
    pub protocol_count: usize,
}

impl Default for RiskMetrics {
    fn default() -> Self {
        Self {
            volatility: 50,
            liquidity: 50,
            security: 50,
            max_protocol_allocation: 1.0,
            max_token_allocation: 1.0,
            protocol_count: 0,
        }
    }
}