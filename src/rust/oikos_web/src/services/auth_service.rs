use super::Error;
use super::Requests;
use oikos_api::components::schemas::*;
use serde::Serialize;
use yew::callback::Callback;
use yew::services::fetch::FetchTask;

#[derive(Default, Debug)]
pub struct AuthService {
    requests: Requests,
}

#[derive(Serialize)]
struct AccessTokenRequest {
    client_id: String,
    client_secret: String,
    code: String,
}

impl AuthService {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    pub fn request_token(
        &mut self,
        code: String,
        callback: Callback<Result<String, Error>>,
    ) -> FetchTask {
        self.requests.post::<AccessTokenRequest, String>(
            "https://github.com/login/oauth/access_token".to_string(),
            AccessTokenRequest {
                client_id: "6243e7d6a656115a9871".to_string(),
                client_secret: "69f43053d4352116deb51ffec675102ec650d7c6".to_string(),
                code,
            },
            callback,
        )
    }
}
