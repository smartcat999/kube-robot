use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigValidatorError {
    #[error("trivy cache dir must not be blank")]
    TrivyCacheDirEmpty,
    #[error("trivy reports dir must not be blank")]
    TrivyReportsDirEmpty,
    #[error("TLS certificate file does not exist: `{0}`")]
    TLSCertNotFound(String),
    #[error("TLS private key file does not exist: `{0}`")]
    TLSPrivateKeyNotFound(String),
    #[error("ClientCA file does not exist: `{0}`")]
    ClientCANotFound(String)
}