use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RuntimeConfig {
    pub wechat_api: String,
    pub github: GithubConfig,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GithubConfig {
    pub owner: String,
    pub repo: String,
    pub token: String,
}