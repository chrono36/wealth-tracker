use thiserror::Error;

#[derive(Debug, Error)]
pub enum TrackerError {
    #[error("request http error")]
    HttpError(#[from] reqwest::Error),
}
