#[derive(Debug, Clone)]
pub struct LexerState {
  pub chars: Vec<char>,
  pub length: usize,
  pub position: usize,
}

impl LexerState {
  pub fn at_end(&self) -> bool {
    self.position >= self.length
  }
}