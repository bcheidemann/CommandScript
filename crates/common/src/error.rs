pub fn format_error_message_inline(source: &str, message: &str, position: usize) -> String {
    let mut line = 1;
    let mut column = 1;
    for (i, ch) in source.chars().enumerate() {
        if i == position {
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

    output.push_str(&format!("{}:{}: {}\n", line, column, message));
    output.push_str(&format!("{}\n", source.lines().nth(line - 1).unwrap()));
    for _ in 0..column - 1 {
        output.push(' ');
    }
    output.push_str("^");
    return output;
}
