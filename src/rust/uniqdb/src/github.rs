use super::UniqDb;
use anyhow::Result;
use github_rs::client::{Executor, Github};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum GithubDbError {
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

pub struct GithubDb {
    client: Github,
    organization: String,
}

impl GithubDb {
    pub fn new(token: &str, organization: &str) -> Result<Self, GithubDbError> {
        Ok(Self {
            client: Github::new(token)?,
            organization: organization.to_string(),
        })
    }
}

impl<T> UniqDb<T> for GithubDb
where
    T: DeserializeOwned + Serialize + Clone,
{
    type Error = GithubDbError;

    fn get_all(&self) -> Result<Vec<T>, GithubDbError> {
        let mut recipe_list = vec![];

        let mut page = 1;
        let mut data = list_organization_repositories(&self.client, &self.organization, page)?;
        while data.len() == 100 {
            recipe_list.append(&mut data);
            page += 1;
            data = list_organization_repositories(&self.client, &self.organization, page)?;
        }
        recipe_list.append(&mut data);

        let recipes = recipe_list
            .iter()
            .filter_map(|item| match serde_json::from_value(item.clone()) {
                Ok(item) => Some(item),
                Err(err) => {
                    println!("{}", err);
                    None
                }
            })
            .collect::<Vec<_>>();

        Ok(recipes)
    }

    fn create(&self, id: &str, name: &str, model: &T) -> Result<T, GithubDbError> {
        create_an_organization_repository(&self.client, &self.organization, &id, name)?;
        create_or_update_file_content(
            &self.client,
            &self.organization,
            &id,
            "uniq.json",
            &serde_json::json!({
                "message": "update",
                "content": base64::encode(serde_json::to_string(&model)?),
            }),
        )?;
        Ok(model.clone())
    }

    fn get(&self, id: &str) -> Result<T, GithubDbError> {
        let json = get_file_content(&self.client, &self.organization, &id, "uniq.json")?;
        if let Some(content) = json.get("content") {
            let text = content.to_string().replace("\\n", "").replace("\"", "");
            let bytes = base64::decode(text)?;
            let data = std::str::from_utf8(&bytes)?;
            let recipe = serde_json::from_str(data)?;
            Ok(recipe)
        } else {
            Err(GithubDbError::InvalidDataError(
                "Missing content field".to_string(),
            ))
        }
    }

    fn update(&self, id: &str, model: &T) -> Result<T, GithubDbError> {
        let json = get_file_content(&self.client, &self.organization, &id, "uniq.json")?;
        if let Some(sha) = json.get("sha") {
            create_or_update_file_content(
                &self.client,
                "open-cooking",
                &id,
                "uniq.json",
                &serde_json::json!({
                    "message": "update",
                    "content": base64::encode(serde_json::to_string(&model)?),
                    "sha": sha,
                }),
            )?;
            Ok(model.clone())
        } else {
            Err(GithubDbError::InvalidDataError(
                "Missing content field".to_string(),
            ))
        }
    }

    fn delete(&self, id: &str) -> Result<(), GithubDbError> {
        delete_a_repository(&self.client, &self.organization, &id)
    }
}

fn list_organization_repositories(
    client: &Github,
    org: &str,
    page: u32,
) -> Result<Vec<serde_json::Value>, GithubDbError> {
    let endpoint = format!(
        "orgs/{}/repos?per_page=100&sort=updated&direction=dsc&page={}",
        org, page
    );
    let (_, _, json) = client
        .get()
        .custom_endpoint(&endpoint)
        .execute::<serde_json::Value>()?;
    if let Some(json) = json {
        if let Some(data) = json.as_array() {
            Ok(data.clone())
        } else {
            Err(GithubDbError::InvalidDataError(json.to_string()))
        }
    } else {
        Err(GithubDbError::InvalidDataError(
            "no json body response".to_string(),
        ))
    }
}

fn create_an_organization_repository(
    client: &Github,
    org: &str,
    id: &str,
    name: &str,
) -> Result<serde_json::Value, GithubDbError> {
    let endpoint = format!("orgs/{}/repos", org);
    let (_, _, json) = client
        .post(serde_json::json!({ "name": id, "description": name }))
        .custom_endpoint(&endpoint)
        .execute::<serde_json::Value>()?;
    if let Some(json) = json {
        Ok(json)
    } else {
        Err(GithubDbError::InvalidDataError(
            "no json body response".to_string(),
        ))
    }
}

fn delete_a_repository(client: &Github, owner: &str, repo_name: &str) -> Result<(), GithubDbError> {
    let endpoint = format!("repos/{}/{}", owner, repo_name);
    client
        .delete(serde_json::json!({}))
        .custom_endpoint(&endpoint)
        .execute::<serde_json::Value>()?;
    Ok(())
}

fn get_file_content(
    client: &Github,
    owner: &str,
    repo_name: &str,
    path: &str,
) -> Result<serde_json::Value, GithubDbError> {
    let (_, _, json) = client
        .get()
        .repos()
        .owner(owner)
        .repo(repo_name)
        .contents()
        .path(path)
        .execute()?;
    if let Some(json) = json {
        Ok(json)
    } else {
        Err(GithubDbError::InvalidDataError(
            "no json body response".to_string(),
        ))
    }
}

fn create_or_update_file_content(
    client: &Github,
    owner: &str,
    repo_name: &str,
    path: &str,
    body: &serde_json::Value,
) -> Result<serde_json::Value, GithubDbError> {
    let endpoint = format!("repos/{}/{}/contents/{}", owner, repo_name, path);
    let (_, _, json) = client
        .put(body)
        .custom_endpoint(&endpoint)
        .execute::<serde_json::Value>()?;
    if let Some(json) = json {
        Ok(json)
    } else {
        Err(GithubDbError::InvalidDataError(
            "no json body response".to_string(),
        ))
    }
}
