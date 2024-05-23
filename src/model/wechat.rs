use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct WebhookEvent<'r> {
    #[serde(rename = "msgtype")]
    pub msg_type: &'r str,
    pub markdown: MarkDown<'r>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct MarkDown<'r> {
    pub content: &'r str,
}