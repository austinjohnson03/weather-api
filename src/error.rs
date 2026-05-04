#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Failed to build HTTP client: {0}")]
    BuilderError(#[from] reqwest::Error),

    #[error("Connection timeout")]
    ConnectionTimeout,

    #[error("Request time out")]
    RequestTimeout,

    #[error("HTTP error ({status}): {body}")]
    HttpError { status: u16, body: String },

    #[error("Too many redirects")]
    TooManyRedirects,

    #[error("Failed to parse JSON reponse: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("TLS Error: {0}")]
    TlsError(String),

    #[error("Request failed: {0}")]
    RequestFailed(#[from] reqwest_middleware::Error),

    #[error("Unexpected Error: {0}")]
    Unknown(String),
}
