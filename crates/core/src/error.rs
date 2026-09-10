use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("YAML parse error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Markdown format error: {0}")]
    MarkdownFormat(String),

    #[error("Entry not found: {0}")]
    NotFound(String),

    #[error("Index error: {0}")]
    IndexError(String),
}

pub type Result<T> = std::result::Result<T, CoreError>;
