use std::fmt::Debug;
use rocket::serde::{Deserialize, Serialize};
use chrono::prelude::*;

const SCHEMA_VERSION: i64 = 2;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ScanReport {
    schema_version: i64,
    #[serde(rename = "Results")]
    result: Vec<ScanResult>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ScanResult {
    #[serde(rename = "Target")]
    target: String,
    #[serde(rename = "Vulnerabilities")]
    vulnerabilities: Vec<Vulnerability>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Metadata {
    #[serde(rename = "NextUpdate")]
    next_update: DateTime<Local>,
    #[serde(rename = "UpdatedAt")]
    updated_at: DateTime<Local>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct VersionInfo {
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(rename = "Version")]
    version: String,
    #[serde(rename = "VulnerabilityDB")]
    vulnerability_db: Metadata,
    #[serde(rename = "JavaDB")]
    java_db:        Metadata
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Layer {
    #[serde(rename = "Digest")]
    digest: String,
    #[serde(rename = "DiffID")]
    diff_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct CVSSInfo {
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(rename = "V2Vector")]
    v2vector: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(rename = "V3Vector")]
    v3vector: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "V2Score")]
    v2score: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "V3Score")]
    v3score: Option<f32>,
}

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Report<T: Debug + Serialize> {
    sbom: T,
    vulnerabilities: Vec<Vulnerability>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Vulnerability {
    #[serde(rename = "VulnerabilityID")]
    vulnerability_id: String,
    #[serde(rename = "PkgName")]
    pkg_name: String,
    #[serde(rename = "InstalledVersion")]
    installed_version: String,
    #[serde(rename = "FixedVersion")]
    fixed_version: String,
    #[serde(rename = "Title")]
    title: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Severity")]
    severity: String,
    #[serde(rename = "References")]
    references: String,
    #[serde(rename = "PrimaryURL")]
    primary_url: String,
    #[serde(rename = "Layer")]
    layer: String,
    #[serde(rename = "CVSS")]
    cvss: String,
    #[serde(rename = "CweIDs")]
    cwe_ids: String,
}