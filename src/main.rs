mod libs;
mod model;
mod config;

#[macro_use]
extern crate rocket;

use model::harbor;
use model::wechat;
use rocket::serde::json::{Value, json};
use rocket::serde::{json};
use std::sync::{Arc};
use tokio::sync::Mutex;
use lazy_static::lazy_static;
use reqwest::Body;
use rocket::data::{Data, ToByteUnit};
use http::{HeaderMap, header};
use crate::libs::http::ProxyClient;
use config::RuntimeConfig;
use rocket::fairing::AdHoc;
use rocket::{State};
use crate::libs::issues::{GithubHttpClient, IssueHelper, IssueReq, IssueResponse};



lazy_static! {
    static ref PROXY_CLIENT: Arc<Mutex<ProxyClient>> = {
        let client = ProxyClient::new();
        Arc::new(Mutex::new(client))
    };
    static ref ISSUE_CLIENT: Arc<Mutex<IssueHelper>> = {
        let client = IssueHelper::new(GithubHttpClient::default());
        Arc::new(Mutex::new(client))
    };
}

pub const HTTP_RESPONSE_OK: Value = json!({ "status": "ok"});
pub const HTTP_RESPONSE_FAILED: Value = json!({ "status": "failed"});


#[get("/healthz")]
fn index() -> &'static str {
    "ok!"
}

#[post("/push/<channel>", format = "json", data = "<data>")]
async fn push(channel: &str, data: Data<'_>, runtime_config: &State<RuntimeConfig>) -> Option<Value> {
    debug!("{:#?}", channel);
    debug!("{:#?}", runtime_config);
    match channel {
        model::CHANNEL_HARBOR => {
            let bytes = match data.open(1024.kibibytes()).into_bytes().await {
                Ok(v) => v,
                Err(e) => {
                    warn!("{}", e);
                    return Some(HTTP_RESPONSE_FAILED);
                }
            };
            if !bytes.is_complete() {
                warn!("there are bytes remaining in the stream");
                return Some(HTTP_RESPONSE_FAILED);
            }

            let bytes_data = bytes.into_inner();
            let mut webhook_event: harbor::WebhookEvent = match json::from_slice(bytes_data.as_slice()) {
                Ok(v) => v,
                Err(e) => {
                    warn!("{}", e);
                    return Some(HTTP_RESPONSE_FAILED);
                }
            };
            info!("{:#?}", webhook_event);

            if webhook_event.event_type != harbor::HARBOR_EVENT_TYPE_SCANNING_COMPLETED {
                info!("{:#?}", webhook_event.event_type);
                return Some(HTTP_RESPONSE_OK)
            }

            // create event message
            let mut resource = &mut harbor::Resource::default();
            if webhook_event.event_data.resources.len() <= 1 {
                resource = webhook_event.event_data.resources.first_mut().unwrap();
            }
            let mut content = format!("<font color=\\\"info\\\">harbor</font>\n\
            事件：<font color=\\\"info\\\">{}</font>\n\
            仓库：<font color=\\\"info\\\">{}</font>\n\
            镜像：{}\n\
            扫描结果: <font color=\\\"info\\\">{}总计-{}可修复</font>\n\
            操作者：<font color=\\\"info\\\">{}</font>\n",
                                  webhook_event.event_type,
                                  webhook_event.event_data.repository.repo_full_name,
                                  format!("[{}]({})", resource.resource_url, resource.resource_url),
                                  resource.scan_overview.report_v1.summary.total,
                                  resource.scan_overview.report_v1.summary.fixable,
                                  webhook_event.operator);

            // create GitHub issue
            if webhook_event.event_type == "SCANNING_COMPLETED" && resource.scan_overview.report_v1.summary.total > 0 {
                let mut issue: IssueReq = IssueReq::default();
                issue.title = format!("{}扫描出CVE漏洞{}总计-{}可修复",
                                      webhook_event.event_data.repository.name,
                                      resource.scan_overview.report_v1.summary.total,
                                      resource.scan_overview.report_v1.summary.fixable);
                issue.body = format!("镜像: {}\n\n扫描结果: {}总计-{}可修复\n\nHigh: {}\nMedium: {}\nLow: {}",
                                     resource.resource_url,
                                     resource.scan_overview.report_v1.summary.total,
                                     resource.scan_overview.report_v1.summary.fixable,
                                     resource.scan_overview.report_v1.summary.summary.high,
                                     resource.scan_overview.report_v1.summary.summary.medium,
                                     resource.scan_overview.report_v1.summary.summary.low);
                issue.assignees.push(runtime_config.github.owner.clone());
                issue.labels.push(String::from("bug"));

                let issue_client = ISSUE_CLIENT.lock().await;
                let issue = match issue_client.create_issue(&runtime_config.github.token, &runtime_config.github.owner, &runtime_config.github.repo, issue.body()).await {
                    Ok(v) => v,
                    Err(e) => {
                        warn!("{}", e);
                        return Some(HTTP_RESPONSE_FAILED);
                    }
                };

                let body = Body::from("");
                let header = HeaderMap::new();
                let issue_response = match issue_client.client.fetch_url(&issue.url, "GET", body, &header).await {
                    Ok(v) => v,
                    Err(e) => {
                        warn!("{}", e);
                        return Some(HTTP_RESPONSE_FAILED);
                    }
                };
                let issue_meta = match issue_response.json::<IssueResponse>().await {
                    Ok(v) => v,
                    Err(e) => {
                        warn!("{}", e);
                        return Some(HTTP_RESPONSE_FAILED);
                    }
                };
                content = format!("{}GitHub: [issue #{}]({})\n", content, issue.number, issue_meta.html_url)
            }

            // push webhook message
            let mut wechat_webhook: wechat::WebhookEvent = wechat::WebhookEvent::default();
            wechat_webhook.msg_type = "markdown";
            wechat_webhook.markdown.content = content.as_str();

            let event_body = match json::to_string(&wechat_webhook) {
                Ok(v) => v,
                Err(e) => {
                    warn!("{}", e);
                    return Some(HTTP_RESPONSE_FAILED);
                }
            };
            let body = Body::from(event_body);
            let mut header = HeaderMap::new();
            header.insert(header::ACCEPT, header::HeaderValue::from_str("application/json").unwrap());

            let url: String = runtime_config.wechat_api.clone();
            let client = PROXY_CLIENT.lock().await;
            let response = match client.fetch_url(&url, "POST", body, &header).await {
                Ok(v) => v,
                Err(e) => {
                    warn!("{}", e);
                    return Some(HTTP_RESPONSE_FAILED);
                }
            };
            let status = response.status();
            info!("{}", status);
        }
        _ => {
            return Some(HTTP_RESPONSE_FAILED);
        }
    }
    Some(HTTP_RESPONSE_OK)
}

#[launch]
fn rocket() -> _ {
    let config = rocket::Config::default();
    if config.tls_enabled() {
        info!("TLS is enabled!");
    } else {
        info!("TLS is disabled.");
    }
    rocket::build()
        .mount("/", routes![index, push])
        .attach(AdHoc::config::<RuntimeConfig>())
}