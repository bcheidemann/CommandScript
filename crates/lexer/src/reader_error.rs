#[derive(thiserror::Error, Debug)]
#[error("Reader error: {message} at {position}")]
pub struct ReaderError {
    pub message: String,
    pub position: usize,
}
