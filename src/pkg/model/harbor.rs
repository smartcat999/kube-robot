use rocket::serde::{Serialize, Deserialize};

pub const HARBOR_EVENT_TYPE_SCANNING_COMPLETED: &'static str = "SCANNING_COMPLETED";
pub const HARBOR_EVENT_TYPE_PUSH_ARTIFACT: &'static str = "PUSH_ARTIFACT";
pub const HARBOR_EVENT_TYPE_PULL_ARTIFACT: &'static str = "PULL_ARTIFACT";

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct WebhookEvent<'r> {
    #[serde(rename = "type")]
    pub event_type: &'r str,
    pub occur_at: u64,
    pub operator: &'r str,
    pub event_data: EventData<'r>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct EventData<'r> {
    #[serde(borrow)]
    #[serde(default)]
    pub resources: Vec<Resource<'r>>,
    pub repository: Repository<'r>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct Resource<'r> {
    #[serde(default)]
    pub digest: &'r str,
    #[serde(default)]
    pub tag: &'r str,
    #[serde(default)]
    pub resource_url: &'r str,
    #[serde(default)]
    pub scan_overview: ScanOverview<'r>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct ScanOverview<'r> {
    #[serde(borrow)]
    #[serde(rename = "application/vnd.security.vulnerability.report; version=1.1")]
    pub report_v1: ScanReport<'r>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct ScanReport<'r> {
    pub report_id: &'r str,
    pub scan_status: &'r str,
    pub severity: &'r str,
    pub duration: u64,
    #[serde(default)]
    pub summary: ScanSummary,
    pub start_time: &'r str,
    pub end_time: &'r str,
    pub scanner: Scanner<'r>,
    pub complete_percent: u64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct ScanSummary {
    pub total: u64,
    pub fixable: u64,
    pub summary: CVESummary,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct CVESummary {
    #[serde(default)]
    #[serde(rename = "High")]
    pub high: u64,
    #[serde(default)]
    #[serde(rename = "Medium")]
    pub medium: u64,
    #[serde(default)]
    #[serde(rename = "Low")]
    pub low: u64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub struct Scanner<'r> {
    pub name: &'r str,
    pub vendor: &'r str,
    pub version: &'r str,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Repository<'r> {
    #[serde(default)]
    pub date_created: u64,
    pub name: &'r str,
    pub namespace: &'r str,
    pub repo_full_name: &'r str,
    pub repo_type: &'r str,
}