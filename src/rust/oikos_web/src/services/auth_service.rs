use super::Error;
use super::Requests;
use oikos_api::components::schemas::{AccessToken, AccessTokenRequest};
use yew::callback::Callback;
use yew::services::fetch::FetchTask;

#[derive(Default, Debug)]
pub struct AuthService {
    requests: Requests,
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
        callback: Callback<Result<AccessToken, Error>>,
    ) -> FetchTask {
        self.requests.post::<AccessTokenRequest, AccessToken>(
            "/api/access_token".to_string(),
            AccessTokenRequest { code },
            callback,
        )
    }
}
