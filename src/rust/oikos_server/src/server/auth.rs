use oikos_api::components::schemas::AccessToken;

use serde::Serialize;
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("missing env var")]
    MissingEnvVar(#[from] std::env::VarError),
    #[error("reqwest error")]
    ReqwestError(#[from] reqwest::Error),
    #[error("unknown response")]
    UnknownResponse,
}

#[derive(Serialize)]
pub struct AccessTokenRequest {
    client_id: String,
    client_secret: String,
    code: String,
}

impl crate::server::Server {
    pub async fn get_oauth_access_token(&self, code: &str) -> Result<AccessToken, AuthError> {
        let client = reqwest::Client::new();
        let client_id = std::env::var("GITHUB_CLIENT_ID")?;
        let client_secret = std::env::var("GITHUB_CLIENT_SECRET")?;

        let response = client
            .post("https://github.com/login/oauth/access_token")
            .header("Accept", "application/json")
            .json(&AccessTokenRequest {
                client_id: client_id.to_string(),
                client_secret: client_secret.to_string(),
                code: code.to_string(),
            })
            .send()
            .await?;

        match response.status().as_str() {
            "200" => Ok(response.json().await?),
            _ => Err(AuthError::UnknownResponse),
        }
    }
}
