#[derive(thiserror::Error, Debug)]
#[error("Parser error: {message} at {position}")]
pub struct ParserError {
  pub message: String,
  pub position: usize,
}
