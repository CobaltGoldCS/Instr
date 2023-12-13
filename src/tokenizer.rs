use std::iter::Peekable;

#[derive(PartialEq, Debug)]
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
            Some(Token::Start(consume_to_char(chars, &[' ', '\n'])?))
        }
        '$' => {
            chars.next();
            Some(Token::End)
        }
        '\n' | ' ' => Some(Token::Whitespace(chars.next()?)),
        _ => Some(Token::Text(consume_to_char(chars, &['$'])?)),
    };
}

fn consume_to_char(
    chars: &mut Peekable<impl Iterator<Item = char>>,
    consume_to: &[char],
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

#[cfg(test)]
mod tests {
    use super::{tokenize_string, Token};

    #[test]
    fn tokenizer_recognizes_whitespace() {
        let input = " \n \n".to_owned();
        let tokens = tokenize_string(input);

        assert_eq!(
            vec![
                Token::Whitespace(' '),
                Token::Whitespace('\n'),
                Token::Whitespace(' '),
                Token::Whitespace('\n')
            ],
            tokens,
            "Tokens should be properly handled"
        );

        match tokens[0] {
            Token::Whitespace(char) => assert_eq!(
                char, ' ',
                "First token should properly hold correct whitespace character."
            ),
            _ => assert!(false, "Incorrect type"),
        }

        match tokens[1] {
            Token::Whitespace(char) => assert_eq!(
                char, '\n',
                "First token should properly hold correct whitespace character."
            ),
            _ => assert!(false, "Incorrect type"),
        }

        match tokens[2] {
            Token::Whitespace(char) => assert_eq!(
                char, ' ',
                "First token should properly hold correct whitespace character."
            ),
            _ => assert!(false, "Incorrect type"),
        }

        match tokens[3] {
            Token::Whitespace(char) => assert_eq!(
                char, '\n',
                "First token should properly hold correct whitespace character."
            ),
            _ => assert!(false, "Incorrect type"),
        }
    }

    #[test]
    fn tokenizer_handles_simple_text() {
        let input = "!normal This is a test$".to_owned();
        let tokens = tokenize_string(input);

        for token in &tokens {
            match token {
                Token::Start(value) => assert_eq!("normal", value),
                Token::Text(text) => assert_eq!("This is a test", text),
                Token::Whitespace(value) => assert_eq!(&' ', value),
                _ => (),
            }
        }

        assert_eq!(
            vec![
                Token::Start("normal".to_string()),
                Token::Whitespace(' '),
                Token::Text("This is a test".to_string()),
                Token::End
            ],
            tokens
        );
    }
}
