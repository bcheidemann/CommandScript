use crate::{ reader_result::ReaderResult, reader_state::ReaderState };

pub trait Reader {
  fn name(&self) -> String;

  fn read(
    &self,
    _: &mut ReaderState,
  ) -> ReaderResult {
    ReaderResult::None
  }
}
