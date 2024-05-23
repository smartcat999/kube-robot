#![allow(dead_code)]
use std::collections::HashMap;
use reqwest::Body;
use serde::{Deserialize, Serialize};
use serde_json;
use reqwest;
use http::{HeaderMap, Method};
use reqwest::{Response, Url, Client};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/39.0.2171.71 Safari/537.36";


#[derive(Debug)]
pub struct GithubHttpClient {
    pub client: Client,
}


impl GithubHttpClient {
    pub fn new() -> GithubHttpClient {
        let async_client = reqwest::Client::builder()
            .use_native_tls()
            .user_agent(USER_AGENT)
            .build().unwrap();
        GithubHttpClient {
            client: async_client
        }
    }
}

impl Default for GithubHttpClient {
    fn default() -> Self {
        let async_client = reqwest::Client::builder()
            .use_native_tls()
            .user_agent(USER_AGENT)
            .build().unwrap();
        GithubHttpClient {
            client: async_client
        }
    }
}

impl GithubHttpClient {
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


#[derive(Serialize, Deserialize)]
#[derive(Debug, Default, Clone)]
pub struct Issue {
    pub id: u64,
    pub number: u64,
    pub title: String,
    pub state: String,
    pub url: String,
}


#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
pub struct IssueReq {
    pub title: String,
    pub body: String,
    pub assignees: Vec<String>,
    pub milestone: Option<String>,
    pub labels: Vec<String>,
}


impl IssueReq {
    pub fn body(&self) -> Body {
        Body::from(serde_json::to_string(&self).unwrap())
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
pub struct IssueResponse {
    pub html_url: String,
}


#[derive(Debug)]
pub struct IssueHelper {
    pub client: GithubHttpClient,
    pub issue_store: HashMap<String, Issue>,
}

impl IssueHelper {
    pub fn new(client: GithubHttpClient) -> IssueHelper {
        let issue_store: HashMap<String, Issue> = HashMap::new();
        IssueHelper {
            client,
            issue_store,
        }
    }

    pub async fn create_issue_unique(&mut self, token: &str, owner: &str, repo: &str, req: &IssueReq) -> Result<()> {
        if self.issue_store.is_empty() {
            let issues = self.list_repo_issues(token, owner, repo).await?;
            for issue in issues.iter() {
                self.issue_store.insert(issue.title.to_string(), issue.clone());
            }
        }
        if self.issue_store.contains_key(&req.title) {
            return Ok(());
        }

        let issue = self.create_issue(token, owner, repo, req.body()).await?;
        self.issue_store.insert(issue.title.to_string(), issue);
        Ok(())
    }

    pub async fn create_issue(&self, token: &str, owner: &str, repo: &str, body: Body) -> Result<Issue> {
        let url: String = format!("https://api.github.com/repos/{}/{}/issues", owner, repo);
        let mut header = HeaderMap::new();
        header.insert(http::header::ACCEPT, http::header::HeaderValue::from_str("application/vnd.github+json")?);
        header.insert(http::header::AUTHORIZATION, http::header::HeaderValue::from_str(format!("Bearer {}", token).as_str())?);
        header.insert("X-GitHub-Api-Version", http::header::HeaderValue::from_str("2022-11-28")?);
        let resp = self.client.fetch_url(&url, "POST", body, &header).await?;
        let issue = resp.json::<Issue>().await?;
        info!("[Info] create_issue {:?} #{:?}", &issue.title, &issue.number);
        Ok(issue)
    }

    pub async fn list_repo_issues(&self, token: &str, owner: &str, repo: &str) -> Result<Vec<Issue>> {
        let per_page = 50;
        let mut page = 1;
        let mut header = HeaderMap::new();
        header.insert(http::header::ACCEPT, http::header::HeaderValue::from_str("application/vnd.github+json")?);
        header.insert(http::header::AUTHORIZATION, http::header::HeaderValue::from_str(format!("Bearer {}", token).as_str())?);
        header.insert("X-GitHub-Api-Version", http::header::HeaderValue::from_str("2022-11-28")?);
        let mut issues: Vec<Issue> = Vec::new();

        loop {
            let url: String = format!("https://api.github.com/repos/{}/{}/issues?per_page={}&page={}", owner, repo, per_page, page);
            let resp = self.client.fetch_url(&url, "GET", Body::from(""), &header).await?;
            let ret = resp.json::<Vec<Issue>>().await?;
            // println!("{:?}", &ret);
            if ret.len() <= 0 {
                break;
            }
            issues.extend(ret);
            page += 1;
        }

        // println!("{:?}", &issues);
        Ok(issues)
    }
}


#[cfg(test)]
mod test {
    use tokio_test;
    use serde_json;
    use http::HeaderMap;
    use reqwest::Body;
    use super::*;

    const TOKEN: &str = "";
    const OWNER: &str = "smartcat999";
    const REPO: &str = "issue_auto_report";

    #[test]
    fn test_fetch_uri() {
        let client = GithubHttpClient::default();
        let token: &str = "";
        let owner: &str = "smartcat999";
        let repo: &str = "issue_auto_report";
        let url: String = format!("https://api.github.com/repos/{}/{}/issues", owner, repo);

        let body = r#"{"title":"CVE-2023-25173","body":"","assignees":["smartcat999"],"milestone": null,"labels":["bug"]}"#;
        let body = Body::from(body);
        let mut header = HeaderMap::new();
        header.insert(http::header::ACCEPT, http::header::HeaderValue::from_str("application/vnd.github+json").unwrap());
        header.insert(http::header::AUTHORIZATION, http::header::HeaderValue::from_str(format!("Bearer {}", token).as_str()).unwrap());
        header.insert("X-GitHub-Api-Version", http::header::HeaderValue::from_str("2022-11-28").unwrap());
        let result = client.fetch_url(&url, "POST", body, &header);
        tokio_test::block_on(result).expect("fetch error");
    }

    #[test]
    fn test_create_issue() {
        let issue_helper = IssueHelper::new(GithubHttpClient::default());
        let issue: IssueReq = serde_json::from_str(r#"{"title":"CVE-2023-25173","body":"","assignees":["smartcat999"],"milestone": null,"labels":["bug"]}"#).unwrap();
        let result = issue_helper.create_issue(TOKEN, OWNER, REPO, issue.body());
        tokio_test::block_on(result).expect("fetch error");
    }

    #[test]
    fn test_list_repo_issues() {
        let issue_helper = IssueHelper::new(GithubHttpClient::default());
        let result = issue_helper.list_repo_issues(TOKEN, OWNER, REPO);
        tokio_test::block_on(result).expect("fetch error");
    }
}
