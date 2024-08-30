use reqwest;
use http::{HeaderMap, Method};
use reqwest::{Response, Url, Client};
use reqwest::Body;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/39.0.2171.71 Safari/537.36";


#[derive(Debug)]
pub struct ProxyClient {
    pub client: Client,
}

impl ProxyClient {
    pub fn new() -> ProxyClient {
        let async_client = reqwest::Client::builder()
            .use_native_tls()
            .user_agent(USER_AGENT)
            .build().unwrap();
        ProxyClient {
            client: async_client
        }
    }
}

impl Default for ProxyClient {
    fn default() -> Self {
        let async_client = reqwest::Client::builder()
            .use_native_tls()
            .user_agent(USER_AGENT)
            .build().unwrap();
        ProxyClient {
            client: async_client
        }
    }
}

impl ProxyClient {
    pub async fn fetch_url(&self, url: &String, method: &str, body: Body, header: &HeaderMap) -> Result<Response> {
        let mut req = reqwest::Request::new(
            Method::from_bytes(method.as_bytes())?,
            Url::parse(url)?);
        let _ = req.body_mut().insert(body);
        for (k, v) in header.iter() {
            req.headers_mut().insert(k.clone(), v.clone());
        }
        let res = self.client.execute(req).await?;

        Ok(res)
    }
}


#[cfg(test)]
mod test {
    use http::HeaderMap;
    use reqwest::Body;
    use tokio_test;
    use super::*;

    #[test]
    fn test_fetch_uri() {
        let client = ProxyClient::default();
        let key: &str = "";
        let url: String = format!("https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key={}", key);

        let body = r#"{"msgtype": "markdown", "markdown": {"content": "1"}}"#;
        let body = Body::from(body);
        let mut header = HeaderMap::new();
        header.insert(http::header::ACCEPT, http::header::HeaderValue::from_str("application/json").unwrap());
        let result = client.fetch_url(&url, "POST", body, &header);
        tokio_test::block_on(result).expect("fetch error");
    }
}