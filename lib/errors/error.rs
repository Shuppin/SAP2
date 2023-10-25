use crate::lexer::token::Token;

use crate::colours::*;

#[derive(Debug)]
pub enum ErrorType {
    NameError,
    SyntaxError,
    Runtime(String), // User defined errors?
}

impl ToString for ErrorType {
    fn to_string(&self) -> String {
        match self {
            ErrorType::NameError => "NameError".to_string(),
            ErrorType::SyntaxError => "SyntaxError".to_string(),
            ErrorType::Runtime(msg) => format!("RuntimeError: {}", msg),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub error_type: ErrorType,
    pub token: Token,
}

// TODO: Add a file buffer system
pub fn handle_error(error: Error, src_buffer: &str, file_name: &str) {
    let (line_num, col_num) = find_line_column(src_buffer, error.token.span.start);
    let lines = get_context_lines(src_buffer, line_num);
    let mut lines_enum = lines.iter().enumerate().peekable();
    let span_len = error.token.span.end - error.token.span.start;

    let left_pad = line_num.to_string().len() + 1;
    let blank_pad = " ".repeat(left_pad);

    println!("");
    println!(
        "{colour_cyan}{style_bold}{}--> {colour_reset}{file_name}:{line_num}:{col_num}",
        blank_pad
    );
    println!("{colour_cyan}{style_bold}{} |", blank_pad);
    while let Some((i, line)) = lines_enum.next() {
        let line_num = line_num.saturating_sub(4) + i + 1;
        if lines_enum.peek().is_none() {
            let before_highlight = &line[0..col_num];
            let highlight = &line[col_num..col_num + span_len + 1];
            let after_highlight = &line[col_num + span_len + 1..];
            println!(
                "{colour_cyan}{style_bold}{line_num:>left_pad$} | {colour_reset}{style_reset}{before_highlight}{colour_green}{style_bold}{highlight}{colour_reset}{after_highlight}",
                line_num = line_num,
                left_pad = left_pad
            );
        } else {
            println!("{colour_cyan}{style_bold}{line_num:>left_pad$} | {colour_reset}{style_reset}{line}",);
        }
    }
    if span_len > 1 {
        println!(
            "{colour_cyan}{style_bold}{} | {colour_green}{}{}",
            blank_pad,
            " ".repeat(col_num),
            "~".repeat(span_len)
        );
    } else {
        println!(
            "{colour_cyan}{} | {colour_green}{style_bold}{}^",
            blank_pad,
            " ".repeat(col_num)
        );
    }
    println!("");
    println!(
        "{colour_red}{style_bold} {} {colour_reset}:: {}{colour_reset}{style_reset}",
        error.error_type.to_string(),
        error.message
    );
}

/// Returns the line and column number of the given index.
fn find_line_column(source: &str, index: usize) -> (usize, usize) {
    let mut line = 1;
    let mut column = 0;

    for (i, c) in source.chars().enumerate() {
        if i == index {
            break;
        }

        if c == '\n' {
            line += 1;
            column = 0;
        } else {
            column += 1;
        }
    }

    (line, column)
}

/// Returns the line of code that contains the given index,
/// along with the 3 lines above it (If they exist).
fn get_context_lines(input: &str, line_num: usize) -> Vec<String> {
    let skip = line_num.saturating_sub(4);
    input
        .lines()
        .skip(skip)
        .take(line_num.min(4))
        .map(|line| line.to_string())
        .collect()
}
