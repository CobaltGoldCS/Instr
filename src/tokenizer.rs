use std::iter::Peekable;
use unicode_segmentation::{Graphemes, UnicodeSegmentation};

#[derive(PartialEq, Debug)]
pub enum Token<'a> {
    Start(&'a str),
    End,
    Text(&'a str),
    Whitespace(&'a str),
}

struct StringViewer<'a> {
    iter: Peekable<Graphemes<'a>>,
    index: usize,
}

impl<'a> StringViewer<'a> {
    fn new(string: &'a str) -> Self {
        Self {
            iter: UnicodeSegmentation::graphemes(string, true).peekable(),
            index: 0
        }
    }

    fn next(&mut self) -> Option<&'a str> {
        let grapheme = self.iter.next()?;
        self.index += grapheme.len();
        Some(grapheme)
    }

    fn peek(&mut self) -> Option<&&'a str> {
        self.iter.peek()
    }
}

pub fn tokenize_string<'a>(string: &'a str) -> Vec<Token<'a>> {
    let mut tokens = vec![];
    let mut chars = StringViewer::new(string);

    while let Some(token) = next_token(string, &mut chars) {
        tokens.push(token);
    }

    tokens
}

fn next_token<'a>(string: &'a str, chars: &mut StringViewer<'a>) -> Option<Token<'a>> {
    let whitespace_characters = &[" ", "\n", "\r\n"];
    return match chars.peek()? {
        &"!" => {
            chars.next()?;
            let (begin, end) = consume_to_char(chars, whitespace_characters)?;
            let text = &string[begin..end];
            Some(Token::Start(text))
        }
        &"$" => {
            chars.next()?;
            Some(Token::End)
        }
        &"\r" | &"\r\n" | &"\n" | &" " => {
            let character = chars.next()?;
            Some(Token::Whitespace(character))
        }
        _ => {
            let (begin, end) = consume_to_char(chars, &["$"])?; 
            let text = &string[begin..end];
            Some(Token::Text(text))
        }
    };
}

fn consume_to_char(
    chars: &mut StringViewer,
    consume_to: &[&str],
) -> Option<(usize, usize)> {
    let begin_index = chars.index;
    while let Some(char) = chars.peek() {
        if consume_to.contains(char) {
            break;
        }

        chars.next()?;
    }
    Some((begin_index, chars.index))
}

#[cfg(test)]
mod tests {
    use super::{tokenize_string, Token};

    #[test]
    fn tokenizer_recognizes_whitespace() {
        let input = " \n \r\n";
        let tokens = tokenize_string(input);

        assert_eq!(
            vec![
                Token::Whitespace(" "),
                Token::Whitespace("\n"),
                Token::Whitespace(" "),
                Token::Whitespace("\r\n"),
            ],
            tokens,
            "Tokens should be properly handled"
        );

        match tokens[0] {
            Token::Whitespace(char) => assert_eq!(
                char, " ",
                "First token should properly hold correct whitespace character."
            ),
            _ => assert!(false, "Incorrect type"),
        }

        match tokens[1] {
            Token::Whitespace(char) => assert_eq!(
                char, "\n",
                "First token should properly hold correct whitespace character."
            ),
            _ => assert!(false, "Incorrect type"),
        }

        match tokens[2] {
            Token::Whitespace(char) => assert_eq!(
                char, " ",
                "First token should properly hold correct whitespace character."
            ),
            _ => assert!(false, "Incorrect type"),
        }

        match tokens[3] {
            Token::Whitespace(char) => assert_eq!(
                char, "\r\n",
                "This token should hold a windows style return"
            ),
            _ => assert!(false, "Incorrect type"),
        }
    }

    #[test]
    fn tokenizer_handles_simple_text() {
        let input = "!normal This is a test$";
        let tokens = tokenize_string(input);

        for token in &tokens {
            match token {
                Token::Start(value) => assert_eq!(&"normal", value),
                Token::Text(text) => assert_eq!(&"This is a test", text),
                Token::Whitespace(value) => assert_eq!(&" ", value),
                _ => (),
            }
        }

        assert_eq!(
            vec![
                Token::Start("normal"),
                Token::Whitespace(" "),
                Token::Text("This is a test"),
                Token::End
            ],
            tokens
        );
    }

    #[test]
    fn tokenizer_handles_carriage_return() {
        let input = "!normal \r\nthis is a test$ \r\n";

        let tokens = tokenize_string(input);

        for token in &tokens {
            match token {
                Token::Start(value) => assert_eq!(&"normal", value),
                Token::Text(text) => assert_eq!(&"this is a test", text),
                Token::Whitespace(character) => println!("Whitespace hit!: {:?}", character),
                _ => (),
            }
        }

        let whitespace_tokens = tokens.iter().skip(1).take(4);

        for token in whitespace_tokens {
            if let Token::Whitespace(c) = token {
                if c == &"\r\n" {
                    return;
                }
            }
        }
        assert!(false, "There should be a carriage return character present in the whitespcae characters");
         
    }
}
