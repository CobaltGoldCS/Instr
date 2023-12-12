use std::iter::Peekable;
pub enum Token {
    Start(String),
    End,
    Text(String),
}

pub fn tokenize_string(string: String) -> Vec<Token> {
    let mut tokens = vec![];
    let mut chars = string.chars().peekable();

    while let Some(token) = next_token(&mut chars) {
        tokens.push(token);
    }

    tokens
}

pub fn next_token(chars: &mut Peekable<impl Iterator<Item = char>>) -> Option<Token> {
    return Some(match chars.peek()? {
        '!' => {
            chars.next();
            Token::Start(consume_to_whitespace(chars)?)
        }
        '$' => {
            chars.next();
            Token::End
        }
        _ => Token::Text(consume_to_special_char(chars)?),
    });
}

pub fn consume_to_special_char(chars: &mut Peekable<impl Iterator<Item = char>>) -> Option<String> {
    let mut return_string = String::new();
    while let Some(char) = chars.peek() {
        match char {
            '!' | '$' => {
                break;
            }
            _ => {
                return_string.push(chars.next()?);
            }
        }
    }

    Some(return_string)
}

pub fn consume_to_whitespace(chars: &mut Peekable<impl Iterator<Item = char>>) -> Option<String> {
    let mut return_string = String::new();
    while let Some(char) = chars.peek() {
        match char {
            ' ' => break,
            '\n' => {
                break;
            }
            _ => return_string.push(chars.next()?),
        }
    }
    Some(return_string)
}
