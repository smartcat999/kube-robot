use tokio_test;
use serde_json;
use http::HeaderMap;
use tracing::{info, warn};
use reqwest::Body;
use rocket::futures::future::err;
use common::pkg::github::issues::{GithubHttpClient, IssueHelper, IssueReq};
mod setup;

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

#[test]
fn test_list_repo_issues() {
    setup::setup();
    let issue_helper = IssueHelper::new(GithubHttpClient::default());
    let result = issue_helper.list_repo_issues(TOKEN, OWNER, REPO);
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