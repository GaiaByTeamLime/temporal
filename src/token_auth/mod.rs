use sqlx::PgPool;
use ormx::Insert;
use crate::bearer_token::BearerToken;

pub struct TokenAuth {
    allowed_tokens: Vec<String>,
}

impl TokenAuth {
    pub fn new(token: &str) -> Self {
        Self {
            allowed_tokens: vec![token.to_string()],
        }
    }

    pub async fn verify(&self, token: &BearerToken) -> Result<(), String> {
        match self.allowed_tokens.iter().any(|i| i == token.as_str()) {
            true  => Ok(()),
            false => Err("Token does not exist".to_string())
        }
    }
}