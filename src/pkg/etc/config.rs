use std::time;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct BuildInfo {
    pub version: String,
    pub commit: String,
    pub date: String,
}

pub struct Config {
    pub api: API,
    pub trivy: Trivy,
    pub redis_store: RedisStore,
    pub job_queue: JobQueue,
    pub redis_pool: RedisPool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            api: API::default(),
            trivy: Trivy::default(),
            redis_store: RedisStore::default(),
            job_queue: JobQueue::default(),
            redis_pool: RedisPool::default(),
        }
    }
}

pub struct Trivy {
    pub cache_dir: String,
    pub reports_dir: String,
    pub debug_mode: bool,
    pub vuln_type: String,
    pub scanners: String,
    pub severity: String,
    pub ignore_unfixed: bool,
    pub ignore_policy: String,
    pub skip_db_update: bool,
    pub skip_java_db_update: bool,
    pub offline_scan: bool,
    pub git_hub_token: String,
    pub insecure: bool,
    pub timeout: time::Duration,
}

impl Default for Trivy {
    fn default() -> Self {
        Trivy {
            cache_dir: "".to_string(),
            reports_dir: "".to_string(),
            debug_mode: false,
            vuln_type: "".to_string(),
            scanners: "".to_string(),
            severity: "".to_string(),
            ignore_unfixed: false,
            ignore_policy: "".to_string(),
            skip_db_update: false,
            skip_java_db_update: false,
            offline_scan: false,
            git_hub_token: "".to_string(),
            insecure: false,
            timeout: Default::default(),
        }
    }
}

pub struct API {
    pub addr: String,
    pub tls_certificate: String,
    pub tls_key: String,
    pub client_cas: Vec<String>,
    pub read_timeout: time::Duration,
    pub write_timeout: time::Duration,
    pub idle_timeout: time::Duration,
    pub metrics_enabled: bool,
}

impl API {
    pub fn is_tls_enabled(&self) -> bool {
        return self.tls_certificate != "" && self.tls_key != "";
    }
}

impl Default for API {
    fn default() -> Self {
        API {
            addr: "".to_string(),
            tls_certificate: "".to_string(),
            tls_key: "".to_string(),
            client_cas: vec![],
            read_timeout: Default::default(),
            write_timeout: Default::default(),
            idle_timeout: Default::default(),
            metrics_enabled: false,
        }
    }
}

pub struct RedisStore {
    pub namespace: String,
    pub scan_job_ttl: time::Duration,
}

impl Default for RedisStore {
    fn default() -> Self {
        RedisStore {
            namespace: "".to_string(),
            scan_job_ttl: Default::default(),
        }
    }
}

pub struct JobQueue {
    pub namespace: String,
    pub worker_concurrency: i64,
}

impl Default for JobQueue {
    fn default() -> Self {
        JobQueue {
            namespace: "".to_string(),
            worker_concurrency: 0,
        }
    }
}

pub struct RedisPool {
    pub url: String,
    pub max_active: i64,
    pub max_idle: i64,
    pub idle_timeout: time::Duration,
    pub connection_timeout: time::Duration,
    pub read_timeout: time::Duration,
    pub write_timeout: time::Duration,
}

impl Default for RedisPool {
    fn default() -> Self {
        RedisPool {
            url: "".to_string(),
            max_active: 0,
            max_idle: 0,
            idle_timeout: Default::default(),
            connection_timeout: Default::default(),
            read_timeout: Default::default(),
            write_timeout: Default::default(),
        }
    }
}