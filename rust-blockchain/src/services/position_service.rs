use crate::error::{AppError, AppResult};
use crate::models::position::{
    CreatePositionRequest, Position, PositionResponse, PositionStrategy,
    RebalanceFrequency, TokenAllocation, UpdatePositionRequest,
};
use crate::blockchain::{BlockchainProvider, MultiversXClient};
use crate::utils::{egld_to_denomination, denomination_to_egld};
use chrono::Utc;
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::{info, error, debug};
use uuid::Uuid;

pub struct PositionService {
    db_pool: PgPool,
    blockchain_client: MultiversXClient,
}

impl PositionService {
    pub fn new(db_pool: PgPool, blockchain_client: MultiversXClient) -> Self {
        Self { db_pool, blockchain_client }
    }
    
    pub async fn create_position(
        &self,
        user_id: Uuid,
        request: CreatePositionRequest,
    ) -> AppResult<PositionResponse> {
        // Validate the allocation percentages sum to 100%
        let total_percentage: f64 = request.allocation.iter().map(|a| a.percentage).sum();
        if (total_percentage - 100.0).abs() > 0.01 {
            return Err(AppError::Validation(
                "Token allocation percentages must sum to 100%".to_string(),
            ));
        }
        
        // Get user wallet address for blockchain operations
        let user = sqlx::query!(
            r#"
            SELECT address FROM users WHERE id = $1
            "#,
            user_id
        )
        .fetch_one(&self.db_pool)
        .await?;
        
        // Verify user has sufficient balance on blockchain
        let balance = self.blockchain_client.get_balance(&user.address).await?;
        let egld_balance = denomination_to_egld(&balance)?;
        
        if egld_balance < request.amount {
            return Err(AppError::Validation(
                format!("Insufficient balance. Required: {} EGLD, Available: {} EGLD", 
                    request.amount, egld_balance)
            ));
        }
        
        // Begin database transaction
        let mut tx = self.db_pool.begin().await?;
        
        let now = Utc::now();
        let position_id = Uuid::new_v4();
        
        let metadata = serde_json::json!({
            "created_by": "agentx_blockchain",
            "version": "1.0.0",
            "blockchain_tx": null,
            "last_rebalance_tx": null
        });
        
        // Create position record
        let position = sqlx::query_as!(
            Position,
            r#"
            INSERT INTO positions (
                id, user_id, protocol_id, name, position_type, tokens,
                deposited, current_value, apy, strategy, entry_date,
                last_rebalance, rebalance_frequency, allocation, metadata,
                created_at, updated_at
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17
            )
            RETURNING 
                id, user_id, protocol_id, name, position_type, tokens,
                deposited, current_value, apy, strategy as "strategy: PositionStrategy",
                entry_date, last_rebalance, 
                rebalance_frequency as "rebalance_frequency: RebalanceFrequency",
                allocation as "allocation: Vec<TokenAllocation>", metadata,
                created_at, updated_at
            "#,
            position_id,
            user_id,
            request.protocol_id,
            request.name,
            request.position_type,
            &request.tokens as _,
            request.amount,
            request.amount, // Initial current_value equals deposited amount
            0.0, // Initial APY is 0
            request.strategy as _,
            now,
            now,
            request.rebalance_frequency as _,
            &request.allocation as _,
            serde_json::to_value(metadata)? as _,
            now,
            now
        )
        .fetch_one(&mut *tx)
        .await?;
        
        // Get protocol contract address
        let protocol = sqlx::query!(
            r#"
            SELECT contract_address FROM protocols WHERE id = $1
            "#,
            request.protocol_id
        )
        .fetch_one(&mut *tx)
        .await?;
        
        let contract_address = protocol.contract_address.ok_or_else(|| 
            AppError::Validation(format!("Protocol {} has no contract address", request.protocol_id))
        )?;
        
        // Create transaction record for deposit
        let tx_id = Uuid::new_v4();
        let tx_hash = format!("pending_{}", Uuid::new_v4().to_string().replace("-", ""));
        
        sqlx::query!(
            r#"
            INSERT INTO transactions (
                id, user_id, tx_hash, tx_type, amount, token,
                status, protocol_id, position_id, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
            tx_id,
            user_id,
            tx_hash,
            "Deposit",
            Some(request.amount.to_string()),
            Some("EGLD"),
            "Pending",
            Some(request.protocol_id),
            Some(position_id),
            now,
            now
        )
        .execute(&mut *tx)
        .await?;
        
        // Commit database transaction
        tx.commit().await?;
        
        // Initiate blockchain transaction asynchronously
        tokio::spawn({
            let db_pool = self.db_pool.clone();
            let blockchain_client = self.blockchain_client.clone();
            let user_address = user.address.clone();
            let contract_address = contract_address.clone();
            let amount = request.amount;
            let position_id = position_id;
            let tx_id = tx_id;
            
            async move {
                // In a real implementation, this would create and submit a blockchain transaction
                // to the protocol's smart contract
                debug!("Initiating blockchain transaction for position {}", position_id);
                
                // Simulate blockchain transaction processing
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                
                // Generate a realistic transaction hash
                let tx_hash = format!("tx_{}", Uuid::new_v4().to_string().replace("-", ""));
                
                // Update transaction record with hash
                let result = sqlx::query!(
                    r#"
                    UPDATE transactions
                    SET tx_hash = $1, status = $2, updated_at = $3
                    WHERE id = $4
                    "#,
                    tx_hash,
                    "Success",
                    Utc::now(),
                    tx_id
                )
                .execute(&db_pool)
                .await;
                
                if let Err(e) = result {
                    error!("Failed to update transaction status: {}", e);
                }
                
                // Update position metadata with transaction hash
                let metadata = serde_json::json!({
                    "created_by": "agentx_blockchain",
                    "version": "1.0.0",
                    "blockchain_tx": tx_hash,
                    "last_rebalance_tx": null
                });
                
                let result = sqlx::query!(
                    r#"
                    UPDATE positions
                    SET metadata = $1, updated_at = $2
                    WHERE id = $3
                    "#,
                    serde_json::to_value(metadata).unwrap() as _,
                    Utc::now(),
                    position_id
                )
                .execute(&db_pool)
                .await;
                
                if let Err(e) = result {
                    error!("Failed to update position metadata: {}", e);
                }
                
                info!("Position {} created successfully with transaction {}", position_id, tx_hash);
            }
        });
        
        info!("Created new position: {} for user: {}", position_id, user_id);
        Ok(PositionResponse::from(position))
    }
    
    pub async fn get_positions_by_user(
        &self,
        user_id: Uuid,
        protocol_id: Option<String>,
    ) -> AppResult<Vec<PositionResponse>> {
        let positions = match protocol_id {
            Some(protocol_id) => {
                sqlx::query_as!(
                    Position,
                    r#"
                    SELECT 
                        id, user_id, protocol_id, name, position_type, tokens,
                        deposited, current_value, apy, strategy as "strategy: PositionStrategy",
                        entry_date, last_rebalance, 
                        rebalance_frequency as "rebalance_frequency: RebalanceFrequency",
                        allocation as "allocation: Vec<TokenAllocation>", metadata,
                        created_at, updated_at
                    FROM positions
                    WHERE user_id = $1 AND protocol_id = $2
                    ORDER BY created_at DESC
                    "#,
                    user_id,
                    protocol_id
                )
                .fetch_all(&self.db_pool)
                .await?
            }
            None => {
                sqlx::query_as!(
                    Position,
                    r#"
                    SELECT 
                        id, user_id, protocol_id, name, position_type, tokens,
                        deposited, current_value, apy, strategy as "strategy: PositionStrategy",
                        entry_date, last_rebalance, 
                        rebalance_frequency as "rebalance_frequency: RebalanceFrequency",
                        allocation as "allocation: Vec<TokenAllocation>", metadata,
                        created_at, updated_at
                    FROM positions
                    WHERE user_id = $1
                    ORDER BY created_at DESC
                    "#,
                    user_id
                )
                .fetch_all(&self.db_pool)
                .await?
            }
        };
        
        // Update current values from blockchain if needed
        let updated_positions = self.update_position_values(positions).await?;
        
        Ok(updated_positions.into_iter().map(PositionResponse::from).collect())
    }
    
    pub async fn get_position_by_id(&self, id: Uuid, user_id: Uuid) -> AppResult<PositionResponse> {
        let position = sqlx::query_as!(
            Position,
            r#"
            SELECT 
                id, user_id, protocol_id, name, position_type, tokens,
                deposited, current_value, apy, strategy as "strategy: PositionStrategy",
                entry_date, last_rebalance, 
                rebalance_frequency as "rebalance_frequency: RebalanceFrequency",
                allocation as "allocation: Vec<TokenAllocation>", metadata,
                created_at, updated_at
            FROM positions
            WHERE id = $1 AND user_id = $2
            "#,
            id,
            user_id
        )
        .fetch_one(&self.db_pool)
        .await?;
        
        // Get latest position value from blockchain
        let updated_position = self.update_position_value(position).await?;
        
        Ok(PositionResponse::from(updated_position))
    }
    
    pub async fn update_position(
        &self,
        id: Uuid,
        user_id: Uuid,
        request: UpdatePositionRequest,
    ) -> AppResult<PositionResponse> {
        // Validate allocation if provided
        if let Some(ref allocation) = request.allocation {
            let total_percentage: f64 = allocation.iter().map(|a| a.percentage).sum();
            if (total_percentage - 100.0).abs() > 0.01 {
                return Err(AppError::Validation(
                    "Token allocation percentages must sum to 100%".to_string(),
                ));
            }
        }
        
        // Get the current position
        let position = sqlx::query_as!(
            Position,
            r#"
            SELECT 
                id, user_id, protocol_id, name, position_type, tokens,
                deposited, current_value, apy, strategy as "strategy: PositionStrategy",
                entry_date, last_rebalance, 
                rebalance_frequency as "rebalance_frequency: RebalanceFrequency",
                allocation as "allocation: Vec<TokenAllocation>", metadata,
                created_at, updated_at
            FROM positions
            WHERE id = $1 AND user_id = $2
            "#,
            id,
            user_id
        )
        .fetch_one(&self.db_pool)
        .await?;
        
        // Check if strategy change requires rebalancing
        let strategy_changed = request.strategy.is_some() && request.strategy.as_ref() != Some(&position.strategy);
        let allocation_changed = request.allocation.is_some();
        
        // Begin database transaction
        let mut tx = self.db_pool.begin().await?;
        
        // Update the position
        let now = Utc::now();
        let updated_position = sqlx::query_as!(
            Position,
            r#"
            UPDATE positions
            SET 
                current_value = COALESCE($3, current_value),
                apy = COALESCE($4, apy),
                strategy = COALESCE($5, strategy),
                rebalance_frequency = COALESCE($6, rebalance_frequency),
                allocation = COALESCE($7, allocation),
                updated_at = $8
            WHERE id = $1 AND user_id = $2
            RETURNING 
                id, user_id, protocol_id, name, position_type, tokens,
                deposited, current_value, apy, strategy as "strategy: PositionStrategy",
                entry_date, last_rebalance, 
                rebalance_frequency as "rebalance_frequency: RebalanceFrequency",
                allocation as "allocation: Vec<TokenAllocation>", metadata,
                created_at, updated_at
            "#,
            id,
            user_id,
            request.current_value,
            request.apy,
            request.strategy as _,
            request.rebalance_frequency as _,
            request.allocation as _,
            now
        )
        .fetch_one(&mut *tx)
        .await?;
        
        // If strategy or allocation changed, create a rebalance transaction
        if strategy_changed || allocation_changed {
            // Get protocol contract address
            let protocol = sqlx::query!(
                r#"
                SELECT contract_address FROM protocols WHERE id = $1
                "#,
                position.protocol_id
            )
            .fetch_one(&mut *tx)
            .await?;
            
            let contract_address = protocol.contract_address.ok_or_else(|| 
                AppError::Validation(format!("Protocol {} has no contract address", position.protocol_id))
            )?;
            
            // Create transaction record for rebalance
            let tx_id = Uuid::new_v4();
            let tx_hash = format!("pending_{}", Uuid::new_v4().to_string().replace("-", ""));
            
            sqlx::query!(
                r#"
                INSERT INTO transactions (
                    id, user_id, tx_hash, tx_type, amount, token,
                    status, protocol_id, position_id, created_at, updated_at
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                "#,
                tx_id,
                user_id,
                tx_hash,
                "Rebalance",
                None::<String>,
                None::<String>,
                "Pending",
                Some(position.protocol_id),
                Some(id),
                now,
                now
            )
            .execute(&mut *tx)
            .await?;
            
            // Commit database transaction
            tx.commit().await?;
            
            // Initiate blockchain transaction asynchronously
            tokio::spawn({
                let db_pool = self.db_pool.clone();
                let blockchain_client = self.blockchain_client.clone();
                let position_id = id;
                let tx_id = tx_id;
                let metadata = position.metadata.clone();
                
                async move {
                    // In a real implementation, this would create and submit a blockchain transaction
                    // to rebalance the position according to the new strategy/allocation
                    debug!("Initiating blockchain rebalance transaction for position {}", position_id);
                    
                    // Simulate blockchain transaction processing
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    
                    // Generate a realistic transaction hash
                    let tx_hash = format!("tx_{}", Uuid::new_v4().to_string().replace("-", ""));
                    
                    // Update transaction record with hash
                    let result = sqlx::query!(
                        r#"
                        UPDATE transactions
                        SET tx_hash = $1, status = $2, updated_at = $3
                        WHERE id = $4
                        "#,
                        tx_hash,
                        "Success",
                        Utc::now(),
                        tx_id
                    )
                    .execute(&db_pool)
                    .await;
                    
                    if let Err(e) = result {
                        error!("Failed to update transaction status: {}", e);
                    }
                    
                    // Update position metadata with rebalance transaction hash
                    let mut metadata_value: HashMap<String, serde_json::Value> = 
                        serde_json::from_value(metadata.clone()).unwrap_or_default();
                    
                    metadata_value.insert("last_rebalance_tx".to_string(), serde_json::Value::String(tx_hash.clone()));
                    
                    let result = sqlx::query!(
                        r#"
                        UPDATE positions
                        SET metadata = $1, last_rebalance = $2, updated_at = $3
                        WHERE id = $4
                        "#,
                        serde_json::to_value(metadata_value).unwrap() as _,
                        Utc::now(),
                        Utc::now(),
                        position_id
                    )
                    .execute(&db_pool)
                    .await;
                    
                    if let Err(e) = result {
                        error!("Failed to update position metadata: {}", e);
                    }
                    
                    info!("Position {} rebalanced successfully with transaction {}", position_id, tx_hash);
                }
            });
        } else {
            // Commit database transaction
            tx.commit().await?;
        }
        
        info!("Updated position: {} for user: {}", id, user_id);
        Ok(PositionResponse::from(updated_position))
    }
    
    pub async fn delete_position(&self, id: Uuid, user_id: Uuid) -> AppResult<()> {
        // Get the position to check if it exists and get its details
        let position = sqlx::query_as!(
            Position,
            r#"
            SELECT 
                id, user_id, protocol_id, name, position_type, tokens,
                deposited, current_value, apy, strategy as "strategy: PositionStrategy",
                entry_date, last_rebalance, 
                rebalance_frequency as "rebalance_frequency: RebalanceFrequency",
                allocation as "allocation: Vec<TokenAllocation>", metadata,
                created_at, updated_at
            FROM positions
            WHERE id = $1 AND user_id = $2
            "#,
            id,
            user_id
        )
        .fetch_one(&self.db_pool)
        .await?;
        
        // Begin database transaction
        let mut tx = self.db_pool.begin().await?;
        
        // Get protocol contract address
        let protocol = sqlx::query!(
            r#"
            SELECT contract_address FROM protocols WHERE id = $1
            "#,
            position.protocol_id
        )
        .fetch_one(&mut *tx)
        .await?;
        
        let contract_address = protocol.contract_address.ok_or_else(|| 
            AppError::Validation(format!("Protocol {} has no contract address", position.protocol_id))
        )?;
        
        // Create transaction record for withdrawal
        let tx_id = Uuid::new_v4();
        let tx_hash = format!("pending_{}", Uuid::new_v4().to_string().replace("-", ""));
        let now = Utc::now();
        
        sqlx::query!(
            r#"
            INSERT INTO transactions (
                id, user_id, tx_hash, tx_type, amount, token,
                status, protocol_id, position_id, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
            tx_id,
            user_id,
            tx_hash,
            "Withdraw",
            Some(position.current_value.to_string()),
            Some("EGLD"),
            "Pending",
            Some(position.protocol_id),
            Some(id),
            now,
            now
        )
        .execute(&mut *tx)
        .await?;
        
        // Mark position as deleted (in a real system, you might want to keep the record but mark it as closed)
        let result = sqlx::query!(
            r#"
            DELETE FROM positions
            WHERE id = $1 AND user_id = $2
            "#,
            id,
            user_id
        )
        .execute(&mut *tx)
        .await?;
        
        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Position with ID {} not found", id)));
        }
        
        // Commit database transaction
        tx.commit().await?;
        
        // Initiate blockchain transaction asynchronously
        tokio::spawn({
            let db_pool = self.db_pool.clone();
            let blockchain_client = self.blockchain_client.clone();
            let position_id = id;
            let tx_id = tx_id;
            
            async move {
                // In a real implementation, this would create and submit a blockchain transaction
                // to withdraw funds from the protocol's smart contract
                debug!("Initiating blockchain withdrawal transaction for position {}", position_id);
                
                // Simulate blockchain transaction processing
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                
                // Generate a realistic transaction hash
                let tx_hash = format!("tx_{}", Uuid::new_v4().to_string().replace("-", ""));
                
                // Update transaction record with hash
                let result = sqlx::query!(
                    r#"
                    UPDATE transactions
                    SET tx_hash = $1, status = $2, updated_at = $3
                    WHERE id = $4
                    "#,
                    tx_hash,
                    "Success",
                    Utc::now(),
                    tx_id
                )
                .execute(&db_pool)
                .await;
                
                if let Err(e) = result {
                    error!("Failed to update transaction status: {}", e);
                }
                
                info!("Position {} withdrawn successfully with transaction {}", position_id, tx_hash);
            }
        });
        
        info!("Deleted position: {} for user: {}", id, user_id);
        Ok(())
    }
    
    pub async fn rebalance_position(&self, id: Uuid, user_id: Uuid) -> AppResult<PositionResponse> {
        // Get the position
        let position = sqlx::query_as!(
            Position,
            r#"
            SELECT 
                id, user_id, protocol_id, name, position_type, tokens,
                deposited, current_value, apy, strategy as "strategy: PositionStrategy",
                entry_date, last_rebalance, 
                rebalance_frequency as "rebalance_frequency: RebalanceFrequency",
                allocation as "allocation: Vec<TokenAllocation>", metadata,
                created_at, updated_at
            FROM positions
            WHERE id = $1 AND user_id = $2
            "#,
            id,
            user_id
        )
        .fetch_one(&self.db_pool)
        .await?;
        
        // Begin database transaction
        let mut tx = self.db_pool.begin().await?;
        
        // Get protocol contract address
        let protocol = sqlx::query!(
            r#"
            SELECT contract_address FROM protocols WHERE id = $1
            "#,
            position.protocol_id
        )
        .fetch_one(&mut *tx)
        .await?;
        
        let contract_address = protocol.contract_address.ok_or_else(|| 
            AppError::Validation(format!("Protocol {} has no contract address", position.protocol_id))
        )?;
        
        // Create transaction record for rebalance
        let tx_id = Uuid::new_v4();
        let tx_hash = format!("pending_{}", Uuid::new_v4().to_string().replace("-", ""));
        let now = Utc::now();
        
        sqlx::query!(
            r#"
            INSERT INTO transactions (
                id, user_id, tx_hash, tx_type, amount, token,
                status, protocol_id, position_id, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
            tx_id,
            user_id,
            tx_hash,
            "Rebalance",
            None::<String>,
            None::<String>,
            "Pending",
            Some(position.protocol_id),
            Some(id),
            now,
            now
        )
        .execute(&mut *tx)
        .await?;
        
        // Update position last_rebalance timestamp
        let updated_position = sqlx::query_as!(
            Position,
            r#"
            UPDATE positions
            SET last_rebalance = $3, updated_at = $4
            WHERE id = $1 AND user_id = $2
            RETURNING 
                id, user_id, protocol_id, name, position_type, tokens,
                deposited, current_value, apy, strategy as "strategy: PositionStrategy",
                entry_date, last_rebalance, 
                rebalance_frequency as "rebalance_frequency: RebalanceFrequency",
                allocation as "allocation: Vec<TokenAllocation>", metadata,
                created_at, updated_at
            "#,
            id,
            user_id,
            now,
            now
        )
        .fetch_one(&mut *tx)
        .await?;
        
        // Commit database transaction
        tx.commit().await?;
        
        // Initiate blockchain transaction asynchronously
        tokio::spawn({
            let db_pool = self.db_pool.clone();
            let blockchain_client = self.blockchain_client.clone();
            let position_id = id;
            let tx_id = tx_id;
            let metadata = position.metadata.clone();
            
            async move {
                // In a real implementation, this would create and submit a blockchain transaction
                // to rebalance the position according to the current strategy/allocation
                debug!("Initiating blockchain rebalance transaction for position {}", position_id);
                
                // Simulate blockchain transaction processing
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                
                // Generate a realistic transaction hash
                let tx_hash = format!("tx_{}", Uuid::new_v4().to_string().replace("-", ""));
                
                // Update transaction record with hash
                let result = sqlx::query!(
                    r#"
                    UPDATE transactions
                    SET tx_hash = $1, status = $2, updated_at = $3
                    WHERE id = $4
                    "#,
                    tx_hash,
                    "Success",
                    Utc::now(),
                    tx_id
                )
                .execute(&db_pool)
                .await;
                
                if let Err(e) = result {
                    error!("Failed to update transaction status: {}", e);
                }
                
                // Update position metadata with rebalance transaction hash
                let mut metadata_value: HashMap<String, serde_json::Value> = 
                    serde_json::from_value(metadata.clone()).unwrap_or_default();
                
                metadata_value.insert("last_rebalance_tx".to_string(), serde_json::Value::String(tx_hash.clone()));
                
                let result = sqlx::query!(
                    r#"
                    UPDATE positions
                    SET metadata = $1, updated_at = $2
                    WHERE id = $3
                    "#,
                    serde_json::to_value(metadata_value).unwrap() as _,
                    Utc::now(),
                    position_id
                )
                .execute(&db_pool)
                .await;
                
                if let Err(e) = result {
                    error!("Failed to update position metadata: {}", e);
                }
                
                info!("Position {} rebalanced successfully with transaction {}", position_id, tx_hash);
            }
        });
        
        info!("Rebalanced position: {} for user: {}", id, user_id);
        Ok(PositionResponse::from(updated_position))
    }
    
    // Helper method to update position values from blockchain
    async fn update_position_values(&self, positions: Vec<Position>) -> AppResult<Vec<Position>> {
        let mut updated_positions = Vec::with_capacity(positions.len());
        
        for position in positions {
            let updated_position = self.update_position_value(position).await?;
            updated_positions.push(updated_position);
        }
        
        Ok(updated_positions)
    }
    
    // Helper method to update a single position's value from blockchain
    async fn update_position_value(&self, position: Position) -> AppResult<Position> {
        // In a real implementation, this would query the blockchain for the current value
        // of the position in the protocol's smart contract
        
        // For now, we'll simulate a value update with a small increase
        let time_since_creation = (Utc::now() - position.created_at).num_days() as f64;
        let daily_increase_factor = 1.0 + (position.apy / 365.0 / 100.0);
        let new_value = position.deposited * daily_increase_factor.powf(time_since_creation);
        
        // Only update in database if value has changed significantly
        if (new_value - position.current_value).abs() > 0.001 {
            let updated_position = sqlx::query_as!(
                Position,
                r#"
                UPDATE positions
                SET current_value = $2, updated_at = $3
                WHERE id = $1
                RETURNING 
                    id, user_id, protocol_id, name, position_type, tokens,
                    deposited, current_value, apy, strategy as "strategy: PositionStrategy",
                    entry_date, last_rebalance, 
                    rebalance_frequency as "rebalance_frequency: RebalanceFrequency",
                    allocation as "allocation: Vec<TokenAllocation>", metadata,
                    created_at, updated_at
                "#,
                position.id,
                new_value,
                Utc::now()
            )
            .fetch_one(&self.db_pool)
            .await?;
            
            return Ok(updated_position);
        }
        
        Ok(position)
    }
}