#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    WhiteSpace,
    // Keywords
    DefKeyword,
    ReturnKeyword,
    IfKeyword,
    ElifKeyword,
    ElseKeyword,
    ForKeyword,
    WhileKeyword,
    BreakKeyword,
    ContinueKeyword,
    PassKeyword,
    ImportKeyword,
    FromKeyword,
    AsKeyword,
    TryKeyword,
    ExceptKeyword,
    FinallyKeyword,
    RaiseKeyword,
    ClassKeyword,
    WithKeyword,
    YieldKeyword,
    GlobalKeyword,
    NonlocalKeyword,
    LambdaKeyword,
    AsyncKeyword,
    AwaitKeyword,

    // Operators
    Plus,               // +
    Minus,              // -
    Star,               // *
    Slash,              // /
    DoubleSlash,        // //
    Percent,            // %
    DoubleStar,         // **
    Equals,             // =
    DoubleEquals,       // ==
    NotEquals,          // !=
    LessThan,           // <
    GreaterThan,        // >
    LessThanOrEqual,    // <=
    GreaterThanOrEqual, // >=
    And,                // and
    Or,                 // or
    Not,                // not
    In,                 // in
    NotIn,              // not in
    Is,                 // is
    IsNot,              // is not

    // Delimiters
    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }
    LBracket,  // [
    RBracket,  // ]
    Comma,     // ,
    Colon,     // :
    Dot,       // .
    Semicolon, // ;
    At,        // @
    Arrow,     // ->
    Ellipsis,  // ...

    // Literals
    Identifier(String),
    Integer(i64),
    Float(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    NoneLiteral, // None

    // Comments and Whitespace
    Comment(String),
    Newline,
    Indent,
    Dedent,

    // End of File
    EOF,
}

use std::iter::Peekable;
use std::str::Chars;

fn clean_tokens(tokens: Vec<Token>) -> Vec<Token> {
    let mut cleaned_tokens = Vec::new();
    let mut prev_token: Option<Token> = None; // Track the previous token
    let mut indent_level = 0; // Track the current indent level

    for token in tokens {
        match (&prev_token, &token) {
            // Skip consecutive newlines
            (Some(Token::Newline), Token::Newline) => continue,

            // Handle unnecessary indent/dedent
            (Some(Token::Newline), Token::Dedent) if indent_level == 0 => continue,
            (Some(Token::Dedent), Token::Dedent) if indent_level == 0 => continue,

            // Track indent level changes
            (Some(Token::Indent), _) => indent_level += 1,
            (Some(Token::Dedent), _) if indent_level > 0 => indent_level -= 1,

            // Push valid tokens
            _ => cleaned_tokens.push(token.clone()),
        }
        prev_token = Some(token); // Update previous token tracker
    }

    cleaned_tokens
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut indent_stack: Vec<usize> = [0].to_vec();
    let mut at_line_start = true;

    while let Some(&ch) = chars.peek() {
        if at_line_start {
            handle_indentation(&mut chars, &mut indent_stack, &mut tokens);
            at_line_start = false;
        }

        match ch {
            ' ' | '\t' => {
                consume_whitespace(&mut chars);
                // tokens.push(Token::WhiteSpace);
            }

            '\n' | '\r' => {
                chars.next();
                tokens.push(Token::Newline);
                at_line_start = true;
            }

            '(' => {
                chars.next();
                tokens.push(Token::LParen);
            }

            ')' => {
                chars.next();
                tokens.push(Token::RParen);
            }

            '{' => {
                chars.next();
                tokens.push(Token::LBrace);
            }

            '}' => {
                chars.next();
                tokens.push(Token::RBrace);
            }

            '[' => {
                chars.next();
                tokens.push(Token::LBracket);
            }

            ']' => {
                chars.next();
                tokens.push(Token::RBracket);
            }
            '+' => {
                chars.next();
                tokens.push(Token::Plus);
            }

            '-' => {
                chars.next();
                if chars.peek() == Some(&'>') {
                    chars.next();
                    tokens.push(Token::Arrow);
                } else {
                    tokens.push(Token::Minus);
                }
            }

            '*' => {
                chars.next();
                if chars.peek() == Some(&'*') {
                    chars.next();
                    tokens.push(Token::DoubleStar);
                } else {
                    tokens.push(Token::Star);
                }
            }

            '/' => {
                chars.next();
                if chars.peek() == Some(&'/') {
                    chars.next();
                    tokens.push(Token::DoubleSlash);
                } else {
                    tokens.push(Token::Slash);
                }
            }

            '=' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::DoubleEquals);
                } else {
                    tokens.push(Token::Equals);
                }
            }

            '!' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::NotEquals);
                }
            }

            '<' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::LessThanOrEqual)
                } else {
                    tokens.push(Token::LessThan);
                }
            }

            '>' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::GreaterThanOrEqual)
                } else {
                    tokens.push(Token::GreaterThan);
                }
            }

            '\'' | '"' => {
                tokens.push(consume_string_literal(&mut chars));
            }

            '#' => {
                tokens.push(consume_comment(&mut chars));
            }

            ';' => {
                chars.next();
                tokens.push(Token::Semicolon);
            }

            ':' => {
                chars.next();
                tokens.push(Token::Colon);
            }

            ',' => {
                chars.next();
                tokens.push(Token::Comma);
            }

            '%' => {
                chars.next();
                tokens.push(Token::Percent);
            }

            '0'..='9' => {
                tokens.push(consume_number(&mut chars));
            }

            _ if ch.is_alphabetic() || ch == '_' => {
                tokens.push(consume_identifier_or_keyword(&mut chars));
            }

            _ => {
                chars.next();
            }
        }
    }
    tokens.push(Token::EOF);
    return tokens;
}

fn handle_indentation(
    chars: &mut Peekable<Chars>,
    indent_stack: &mut Vec<usize>,
    tokens: &mut Vec<Token>,
) {
    let mut indent_level = 0;

    // Count spaces or tabs for indentation level
    while let Some(&ch) = chars.peek() {
        if ch == ' ' {
            indent_level += 1;
        } else if ch == '\t' {
            indent_level += 4; // Assume 1 tab = 4 spaces
        } else {
            break;
        }
        chars.next();
    }

    // Check the change in indentation
    let current_level = *indent_stack.last().unwrap();
    if indent_level > current_level {
        indent_stack.push(indent_level);
        tokens.push(Token::Indent);
    } else if indent_level < current_level {
        while indent_stack.last().unwrap() > &indent_level {
            indent_stack.pop();
            tokens.push(Token::Dedent);
        }
    }
}

fn consume_string_literal(chars: &mut Peekable<Chars>) -> Token {
    let quote = chars.next().unwrap(); // Consume the opening quote
    let mut literal = String::new();

    while let Some(&ch) = chars.peek() {
        chars.next();
        if ch == quote {
            break; // Closing quote found
        }
        literal.push(ch);
    }

    Token::StringLiteral(literal)
}

fn consume_comment(chars: &mut Peekable<Chars>) -> Token {
    chars.next(); // Consume the '#'
    let mut comment = String::new();

    while let Some(&ch) = chars.peek() {
        if ch == '\n' {
            break; // End of comment
        }
        chars.next();
        comment.push(ch);
    }

    Token::Comment(comment)
}

fn consume_number(chars: &mut Peekable<Chars>) -> Token {
    let mut number = String::new();

    while let Some(&ch) = chars.peek() {
        if !ch.is_numeric() && ch != '.' {
            break;
        }
        chars.next();
        number.push(ch);
    }

    if number.contains('.') {
        Token::Float(number.parse().unwrap())
    } else {
        Token::Integer(number.parse().unwrap())
    }
}

fn consume_whitespace(chars: &mut Peekable<Chars>) {
    while let Some(&ch) = chars.peek() {
        if ch != ' ' && ch != '\t' {
            break;
        }
        chars.next();
    }
}

fn consume_identifier_or_keyword(chars: &mut Peekable<Chars>) -> Token {
    let mut identifier = String::new();

    while let Some(&ch) = chars.peek() {
        if !ch.is_alphanumeric() && ch != '_' {
            break;
        }
        chars.next();
        identifier.push(ch);
    }

    match identifier.as_str() {
        "def" => Token::DefKeyword,
        "return" => Token::ReturnKeyword,
        "if" => Token::IfKeyword,
        "else" => Token::ElseKeyword,
        "None" => Token::NoneLiteral,
        "True" => Token::BooleanLiteral(true),
        "False" => Token::BooleanLiteral(false),
        "elif" => Token::ElifKeyword,
        "for" => Token::ForKeyword,
        "while" => Token::WhileKeyword,
        "break" => Token::BreakKeyword,
        "continue" => Token::ContinueKeyword,
        "pass" => Token::PassKeyword,
        "import" => Token::ImportKeyword,
        "from" => Token::FromKeyword,
        "as" => Token::AsKeyword,
        "try" => Token::TryKeyword,
        "except" => Token::ExceptKeyword,
        "finally" => Token::FinallyKeyword,
        "raise" => Token::RaiseKeyword,
        "class" => Token::ClassKeyword,
        "with" => Token::WithKeyword,
        "yield" => Token::YieldKeyword,
        "global" => Token::GlobalKeyword,
        "lambda" => Token::LambdaKeyword,
        "async" => Token::AsyncKeyword,
        "await" => Token::AwaitKeyword,
        "and" => Token::And,
        "or" => Token::Or,
        "not" => Token::Not,
        "in" => Token::In,
        "is" => Token::Is,
        "is not" => Token::IsNot,
        _ => Token::Identifier(identifier),
    }
}
