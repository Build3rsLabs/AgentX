use crate::error::AppResult;
use crate::models::user::{User, UserResponse};
use sqlx::PgPool;
use uuid::Uuid;

pub struct UserService {
    db_pool: PgPool,
}

impl UserService {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
    
    pub async fn get_user_by_id(&self, user_id: Uuid) -> AppResult<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, address, nonce, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_one(&self.db_pool)
        .await?;
        
        Ok(user)
    }
    
    pub async fn get_user_by_address(&self, address: &str) -> AppResult<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, address, nonce, created_at, updated_at
            FROM users
            WHERE address = $1
            "#,
            address
        )
        .fetch_one(&self.db_pool)
        .await?;
        
        Ok(user)
    }
    
    pub async fn get_all_users(&self) -> AppResult<Vec<UserResponse>> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, address, nonce, created_at, updated_at
            FROM users
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.db_pool)
        .await?;
        
        Ok(users.into_iter().map(UserResponse::from).collect())
    }
}