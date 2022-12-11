//! Extracting bearer tokens from request headers
use std::convert::TryFrom;
use rocket::{http::Status, outcome, outcome::IntoOutcome, request, Request};

/// The bearer token included in request headers
///
/// Valid requests with the `Authorization` header included should look
/// like the following:
/// ```ignore
/// Authorization: Bearer <some_bearer_token>
/// ```
#[derive(Debug)]
pub struct BearerToken(String);

impl BearerToken {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Errors around invalid request headers and encoded tokens
#[derive(Debug)]
pub enum InvalidAuthHeader {
    /// Too many Authorization headers. Expects only 1
    BadCount,
    /// Authorization header is missing altogether in request
    MissingAuthHeader,
    /// `Bearer` keyword is missing
    MissingBearer,
    /// Bearer token is missing
    MissingBearerValue,
    /// Invalid Bearer token format. Couldn't parse to BearerToken type
    InvalidFormat(String),
}

/// Try to convert an Authorization headers to a valid bearer token
impl TryFrom<&str> for BearerToken {
    type Error = InvalidAuthHeader;

    fn try_from(header: &str) -> Result<Self, Self::Error> {
        match header.trim().split(' ').collect::<Vec<&str>>() {
            parts if parts[0].to_lowercase() != "bearer" =>
                Err(InvalidAuthHeader::MissingBearer),
            parts if parts.len() != 2 =>
                Err(InvalidAuthHeader::InvalidFormat(
                    "Authorization Header should have 2 arguments formatted as \
                    `Bearer <token>`. Number of arguments did not match the \
                    number of arguments expected.".to_string()
                )),
            parts if parts[1].is_empty() =>
                Err(InvalidAuthHeader::MissingBearerValue),
            parts  =>
                Ok(BearerToken(parts[1].to_string()))
        }
    }
}

/// Allows for direct access to bearer tokens as function parameters in rocket
#[rocket::async_trait]
impl<'r> request::FromRequest<'r> for BearerToken {
    type Error = InvalidAuthHeader;

    async fn from_request(
        request: &'r Request<'_>,
    ) -> request::Outcome<Self, Self::Error> {
        match request
            .headers()
            .get("Authorization")
            .collect::<Vec<&str>>()
        {
            auth_field if auth_field.is_empty() => outcome::Outcome::Failure((
                Status::BadRequest,
                InvalidAuthHeader::MissingAuthHeader,
                
            )),
            auth_field if auth_field.len() == 1 => {
                auth_field[0].try_into().into_outcome(Status::BadRequest)
            }
            _ => outcome::Outcome::Failure((
                Status::BadRequest,
                InvalidAuthHeader::BadCount,
            )),
        }
    }
}