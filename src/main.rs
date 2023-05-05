use std::process::exit;

use common::error::format_error_message_inline;
use lexer::default_lexer;
use parser::Parser;

fn main() {
    let source = "1 + 2 * 3 / 4 - some_variable";

    let result = default_lexer().lex(&source);

    if result.errors.len() > 0 {
        result.errors.iter().for_each(move |error| {
            println!("{}", format_error_message_inline(source, &error.message, error.position));
        });

        exit(1);
    }

    println!("tokens = {:#?}", result.tokens);

    let result = Parser::new(&result.tokens).parse();

    if let Err(error) = result {
        println!("{}", format_error_message_inline(source, &error.message, error.position));

        exit(1);
    }

    println!("program = {:#?}", result.unwrap());
}
