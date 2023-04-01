use crate::{lexer_result, lexer_state::LexerState, reader::Reader, reader_result::ReaderResult};

pub struct Lexer {
    pub readers: Vec<Box<dyn Reader>>,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            readers: Vec::new(),
        }
    }

    pub fn add_reader<R>(mut self, reader: R) -> Self
    where
        R: 'static + Reader,
    {
        self.readers.push(Box::new(reader));

        self
    }

    pub fn lex(&mut self, source: &str) -> lexer_result::LexerResult {
        let mut state = LexerState {
            chars: source.chars().collect(),
            length: source.chars().count(),
            position: 0,
        };
        let mut result = lexer_result::LexerResult {
            tokens: Vec::new(),
            errors: Vec::new(),
        };

        for reader in &self.readers {
            println!("Reader: {}", reader.name());
        }

        loop {
            println!("=====================================");
            for reader in &mut self.readers {
                println!("Trying reader: {}", reader.name());

                let mut reader_state = (&state).into();

                let reader_result = reader.read(&mut reader_state);

                match reader_result {
                    ReaderResult::Token(token) => {
                        println!("Found token: {:#?}", token);

                        result.tokens.push(token);
                        state.position = reader_state.get_position();

                        break;
                    }
                    ReaderResult::None => {
                        println!("Reader did not find a token.");

                        // Continue to the next reader.
                        continue;
                    }
                    ReaderResult::Err(error) => {
                        println!("Reader found an error: {:#?}", error);

                        result.errors.push(error);
                        state.position = reader_state.get_position();
                        break;
                    }
                }
            }

            if state.at_end() {
                break;
            }
        }

        result
    }
}
