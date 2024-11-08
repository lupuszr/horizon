use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Authentication failed")]
    AuthenticationFailed,
    #[error("Invalid token")]
    InvalidToken,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub role: UserRole,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    Admin,
    Creator,
    Consumer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    pub token: String,
    pub user_id: Uuid,
}

#[async_trait]
pub trait Auth: Send + Sync + 'static {
    async fn authenticate(&self, username: &str, password: &str) -> Result<AuthToken, AuthError>;
    async fn verify_token(&self, token: &str) -> Result<User, AuthError>;
}

// Simple in-memory auth implementation
pub struct SimpleAuth {
    users: Vec<User>,
    tokens: Vec<AuthToken>,
}

impl SimpleAuth {
    pub fn new() -> Self {
        let admin = User {
            id: Uuid::new_v4(),
            username: "admin".into(),
            role: UserRole::Admin,
        };
        
        Self {
            users: vec![admin],
            tokens: vec![],
        }
    }
}

#[async_trait]
impl Auth for SimpleAuth {
    async fn authenticate(&self, username: &str, _password: &str) -> Result<AuthToken, AuthError> {
        let user = self.users
            .iter()
            .find(|u| u.username == username)
            .ok_or(AuthError::AuthenticationFailed)?;
            
        let token = AuthToken {
            token: Uuid::new_v4().to_string(),
            user_id: user.id,
        };
        
        Ok(token)
    }

    async fn verify_token(&self, token: &str) -> Result<User, AuthError> {
        let token = self.tokens
            .iter()
            .find(|t| t.token == token)
            .ok_or(AuthError::InvalidToken)?;
            
        let user = self.users
            .iter()
            .find(|u| u.id == token.user_id)
            .ok_or(AuthError::InvalidToken)?;
            
        Ok(user.clone())
    }
}
