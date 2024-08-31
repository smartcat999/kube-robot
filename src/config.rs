use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum NotifyType {
    #[serde(rename = "wechat")]
    Wechat
}

impl NotifyType {
    #[allow(unused)]
    fn as_str(&self) -> &'static str {
        match self {
            NotifyType::Wechat => "wechat",
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RuntimeConfig {
    pub notification: Notification,
    pub github: GithubConfig,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GithubConfig {
    pub owner: String,
    pub repo: String,
    pub token: String,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Notification {
    #[serde(rename = "type")]
    pub notify_type: NotifyType,
    pub url: String,
}