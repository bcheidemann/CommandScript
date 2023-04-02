use crate::lexer_state::LexerState;

#[derive(Debug, Clone)]
pub struct ReaderState {
  chars: Vec<char>,
  position_start: usize,
  position_current: usize,
}

impl ReaderState {
  pub fn read(&mut self) -> Option<&char> {
    let ch = self.chars.get(self.position_current)?;
    self.position_current += 1;
    Some(ch)
  }

  pub fn peek(&mut self) -> Option<&char> {
    self.chars.get(self.position_current)
  }

  pub fn get_start(&self) -> usize {
    self.position_start
  }

  pub fn get_position(&self) -> usize {
    self.position_current
  }

  pub fn did_advance(&self) -> bool {
    self.position_start != self.position_current
  }

  pub fn consume_whitespace(&mut self) {
    while let Some(ch) = self.peek() {
      if !ch.is_whitespace() {
        break;
      }
      self.read();
    }
  }

  pub fn read_str<'a>(&mut self, str: &'a str) -> Option<&'a str> {
    let start = self.position_current;
    for ch in str.chars() {
      if self.read() != Some(&ch) {
        self.position_current = start;
        return None;
      }
    }
    Some(str)
  }
}

impl From<&LexerState> for ReaderState {
  fn from(lexer_state: &LexerState) -> Self {
    Self {
      chars: lexer_state.chars.clone(),
      position_start: lexer_state.position.clone(),
      position_current: lexer_state.position.clone(),
    }
  }
}
