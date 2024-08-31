use std::path::Path;
use std::fs;
use crate::pkg::etc::config::Config;
use crate::pkg::etc::errors;
use anyhow::{Result, bail, anyhow};
use tracing::{debug, warn};
use tracing_subscriber::fmt::format;

pub fn check(config: Config) -> Result<()> {
    if config.trivy.cache_dir.is_empty() {
        bail!(errors::ConfigValidatorError::TrivyCacheDirEmpty);
    }
    if config.trivy.reports_dir.is_empty() {
        bail!(errors::ConfigValidatorError::TrivyReportsDirEmpty);
    }
    ensure_dir_exists(&config.trivy.cache_dir, "trivy cache dir")?;
    ensure_dir_exists(&config.trivy.reports_dir, "trivy reports dir")?;

    if config.api.is_tls_enabled() {
        if !file_exists(&config.api.tls_certificate) {
            bail!(errors::ConfigValidatorError::TLSCertNotFound(config.api.tls_certificate.to_string()))
        }
        if !file_exists(&config.api.tls_key) {
            bail!(errors::ConfigValidatorError::TLSPrivateKeyNotFound(config.api.tls_key.to_string()))
        }
        for path in config.api.client_cas.iter() {
            if !file_exists(path) {
                bail!(errors::ConfigValidatorError::ClientCANotFound(path.to_string()))
            }
        }
    }
    return Ok(());
}

fn ensure_dir_exists(path: &str, description: &str) -> Result<()> {
    if !dir_exists(path) {
        warn!(description, "does not exist");
        debug!("Creating {}", description);
        fs::create_dir_all(path)?;
    }
    return Ok(());
}

fn dir_exists(name: &str) -> bool {
    let p = Path::new(name);
    if !p.exists() {
        return false;
    }
    return p.is_dir();
}

fn file_exists(name: &str) -> bool {
    let p = Path::new(name);
    if !p.exists() {
        return false;
    }
    return !p.is_dir();
}