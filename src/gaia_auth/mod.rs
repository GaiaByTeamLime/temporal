use crate::bearer_token::BearerToken;
use reqwest::{StatusCode, header::AUTHORIZATION};

pub struct GaiaAuth {
    base_url: String,
}

pub enum Error {
    Http(reqwest::Error),
    Invalid(String),
}
impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Http(error)
    }
}

impl GaiaAuth {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    pub async fn verify_token(&self, token: &BearerToken) -> Result<String, Error> {
        let client = reqwest::Client::new();
        let result = client
            .get(format!("{}/verify/token", self.base_url))
            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()))
            .send().await?;

        println!("Verify token request for {}, with result {}", token.as_str(), result.status());

        match result.status() {
            StatusCode::OK => Ok(result.text().await?),
            _ => Err(Error::Invalid("Token does not exist".to_string()))
        }
    }

    pub async fn verify_firebase(&self, token: &BearerToken) -> Result<String, Error> {
        let client = reqwest::Client::new();
        let result = client
            .get(format!("{}/verify/firebase", self.base_url))
            .header(AUTHORIZATION, format!("Bearer {}", token.as_str()))
            .send().await?;

        match result.status() {
            StatusCode::OK => Ok(result.text().await?),
            _ => Err(Error::Invalid("Token does not exist".to_string()))
        }
    }
}