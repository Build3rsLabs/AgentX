use crate::config::AppConfig;
use crate::error::{AppError, AppResult};
use crate::models::user::{AuthRequest, AuthResponse, User, UserResponse};
use crate::wallet::Wallet;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use tracing::{info, error};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
    user_id: String,
}

pub struct AuthService {
    db_pool: PgPool,
    config: Arc<AppConfig>,
}

impl AuthService {
    pub fn new(db_pool: PgPool, config: Arc<AppConfig>) -> Self {
        Self { db_pool, config }
    }
    
    pub async fn register_user(&self, address: &str) -> AppResult<User> {
        // Generate a random nonce
        let nonce = self.generate_nonce();
        
        // Check if user already exists
        let existing_user = sqlx::query_as!(
            User,
            r#"
            SELECT id, address, nonce, created_at, updated_at
            FROM users
            WHERE address = $1
            "#,
            address
        )
        .fetch_optional(&self.db_pool)
        .await?;
        
        if let Some(mut user) = existing_user {
            // Update the nonce
            user.nonce = nonce;
            
            sqlx::query!(
                r#"
                UPDATE users
                SET nonce = $1, updated_at = $2
                WHERE id = $3
                "#,
                user.nonce,
                Utc::now(),
                user.id
            )
            .execute(&self.db_pool)
            .await?;
            
            return Ok(user);
        }
        
        // Create a new user
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, address, nonce, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, address, nonce, created_at, updated_at
            "#,
            Uuid::new_v4(),
            address,
            nonce,
            Utc::now(),
            Utc::now()
        )
        .fetch_one(&self.db_pool)
        .await?;
        
        info!("Registered new user with address: {}", address);
        Ok(user)
    }
    
    pub async fn authenticate(&self, auth_request: AuthRequest) -> AppResult<AuthResponse> {
        // Get the user by address
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, address, nonce, created_at, updated_at
            FROM users
            WHERE address = $1
            "#,
            auth_request.address
        )
        .fetch_optional(&self.db_pool)
        .await?
        .ok_or_else(|| AppError::Auth(format!("User with address {} not found", auth_request.address)))?;
        
        // Verify the signature
        let message = format!("Sign this message to authenticate with AgentX: {}", user.nonce);
        let is_valid = self.verify_signature(&auth_request.address, &message, &auth_request.signature)?;
        
        if !is_valid {
            return Err(AppError::Auth("Invalid signature".to_string()));
        }
        
        // Generate a new nonce for next time
        let new_nonce = self.generate_nonce();
        
        sqlx::query!(
            r#"
            UPDATE users
            SET nonce = $1, updated_at = $2
            WHERE id = $3
            "#,
            new_nonce,
            Utc::now(),
            user.id
        )
        .execute(&self.db_pool)
        .await?;
        
        // Generate JWT token
        let token = self.generate_token(&user)?;
        
        Ok(AuthResponse {
            token,
            user: UserResponse::from(user),
        })
    }
    
    fn generate_nonce(&self) -> String {
        let mut rng = thread_rng();
        let nonce: u64 = rng.gen();
        format!("{:016x}", nonce)
    }
    
    fn verify_signature(&self, address: &str, message: &str, signature: &str) -> AppResult<bool> {
        // In a real implementation, this would verify the signature against the message
        // using the MultiversX cryptographic primitives
        
        // For the hackathon demo, we'll just return true
        // In production, you would use the actual verification logic
        
        Ok(true)
    }
    
    fn generate_token(&self, user: &User) -> AppResult<String> {
        let now = Utc::now();
        let expiry = now + Duration::seconds(self.config.api.token_expiration as i64);
        
        let claims = Claims {
            sub: user.address.clone(),
            exp: expiry.timestamp() as usize,
            iat: now.timestamp() as usize,
            user_id: user.id.to_string(),
        };
        
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.api.jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::Auth(format!("Failed to generate token: {}", e)))?;
        
        Ok(token)
    }
}