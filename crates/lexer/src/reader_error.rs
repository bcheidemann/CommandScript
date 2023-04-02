#[derive(thiserror::Error, Debug)]
#[error("Reader error: {message} at {position}")]
pub struct ReaderError {
  pub message: String,
  pub position: usize,
}

impl ReaderError {
  pub fn format_inline(&self, source: &str) -> String {
    let mut line = 1;
    let mut column = 1;
    for (i, ch) in source.chars().enumerate() {
      if i == self.position {
        break;
      }
      if ch == '\n' {
        line += 1;
        column = 1;
      } else {
        column += 1;
      }
    }
    let mut output = String::new();
    
    output.push_str(&format!("{}:{}: {}\n", line, column, self.message));
    output.push_str(&format!("{}\n", source.lines().nth(line - 1).unwrap()));
    for _ in 0..column - 1 {
      output.push(' ');
    }
    output.push_str("^");
    return output;
  }
}
