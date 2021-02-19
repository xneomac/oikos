use super::UniqDb;
use anyhow::Result;
use github_rs::client::{Executor, Github};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum MongoDbError {
    #[error("access token not found")]
    AccessTokenNotFound(#[from] std::env::VarError),
    #[error("base64 decode error")]
    DecodeError(#[from] base64::DecodeError),
    #[error("utf8 error")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("serde error")]
    SerdeError(#[from] serde_json::error::Error),
    #[error("invalid data {0}")]
    InvalidDataError(String),
    #[error("github error {0}")]
    GithubError(#[from] github_rs::errors::Error),
}

pub struct MongoDb {
    client: mongodb::Client,
}

impl MongoDb {
    pub fn new() -> Result<Self, MongoDbError> {
        Ok(Self {
            client: mongodb::Client::with_uri_str("mongodb://localhost:27017/")
                .expect("Failed to initialize client."),
        })
    }
}

impl<T> UniqDb<T> for MongoDb
where
    T: DeserializeOwned + Serialize + Clone,
{
    type Error = MongoDbError;

    fn get_all(&self) -> Result<Vec<T>, MongoDbError> {
        unimplemented!()
    }

    fn create(&self, id: &str, name: &str, model: &T) -> Result<T, MongoDbError> {
        unimplemented!()
    }

    fn get(&self, id: &str) -> Result<T, MongoDbError> {
        unimplemented!()
    }

    fn update(&self, id: &str, model: &T) -> Result<T, MongoDbError> {
        unimplemented!()
    }

    fn delete(&self, id: &str) -> Result<(), MongoDbError> {
        unimplemented!()
    }
}
