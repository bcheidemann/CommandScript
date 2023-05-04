#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn start_from(start: usize) -> Self {
        Self::new(start, start)
    }

    pub fn extend(mut self, end: usize) -> Self {
        self.end = end;
        self
    }
}
