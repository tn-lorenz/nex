#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Pub,
    Fn,
    Ident(String),
    Colon,
    Comma,
    Arrow,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Number(u64),
    Type(String),
    Plus,
    Minus,
    Mul,
    Div,
    Assign,
    Semicolon,
    Let,
    Print,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            c if c.is_whitespace() => {
                chars.next();
            }
            '0'..='9' => {
                let mut num = 0u64;
                while let Some(&d) = chars.peek() {
                    if let Some(digit) = d.to_digit(10) {
                        num = num * 10 + digit as u64;
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(num));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        ident.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                // Keywords erkennen
                let tok = match ident.as_str() {
                    "let" => Token::Let,
                    "print" => Token::Print,
                    _ => Token::Ident(ident),
                };
                tokens.push(tok);
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Mul);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Div);
                chars.next();
            }
            '=' => {
                tokens.push(Token::Assign);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            ';' => {
                tokens.push(Token::Semicolon);
                chars.next();
            }
            _ => {
                chars.next();
            }
        }
    }

    tokens
}
