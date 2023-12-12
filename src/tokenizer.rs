use std::iter::Peekable;

pub enum Token {
    Start(String),
    End,
    Text(String),
    Whitespace(char),
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
    return match chars.peek()? {
        '!' => {
            chars.next();
            Some(Token::Start(consume_to_char(chars, vec![' ', '\n'])?))
        }
        '$' => {
            chars.next();
            Some(Token::End)
        }
        '\n' | ' ' => Some(Token::Whitespace(chars.next()?)),
        _ => Some(Token::Text(consume_to_char(chars, vec!['$'])?)),
    };
}

fn consume_to_char(
    chars: &mut Peekable<impl Iterator<Item = char>>,
    consume_to: Vec<char>,
) -> Option<String> {
    let mut return_string = String::new();
    while let Some(char) = chars.peek() {
        if consume_to.contains(char) {
            break;
        }

        return_string.push(chars.next()?);
    }
    Some(return_string)
}
