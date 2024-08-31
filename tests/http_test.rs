use http::HeaderMap;
use reqwest::Body;
use tokio_test;
use tracing::{info, warn};
use common::pkg::http::proxy::ProxyClient;

mod setup;

#[test]
fn test_fetch_uri() {
    setup::setup();
    let client = ProxyClient::default();
    let key: &str = "";
    let url: String = format!("https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key={}", key);

    let body = r#"{"msgtype": "markdown", "markdown": {"content": "1"}}"#;
    let body = Body::from(body);
    let mut header = HeaderMap::new();
    header.insert(http::header::ACCEPT, http::header::HeaderValue::from_str("application/json").unwrap());
    let result = client.fetch_url(&url, "POST", body, &header);
    tokio_test::block_on(async {
        match result.await {
            Ok(v) => {
                info!("{:?}", v);
            }
            Err(e) => {
                warn!("{}", e);
            }
        }
    });
}