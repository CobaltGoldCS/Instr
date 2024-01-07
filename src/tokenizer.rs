use std::iter::Peekable;

use ratatui::layout::Alignment;
use unicode_segmentation::{Graphemes, UnicodeSegmentation};

use self::tokenizing_helpers::{consume_all, consume_to_char};

#[derive(PartialEq, Debug)]
pub enum Token<'a> {
    Start(&'a str),
    End,
    Text(&'a str),
    Whitespace(&'a str),
}

pub struct ExtractionResult<'a, T> {
    pub string_viewer: StringViewer<'a>,
    pub output_values: Vec<T>,
}

pub enum DisplayAttributes<'a> {
    Title(&'a str),
    TextSize(usize),
    Alignment(Alignment),
}

pub(crate) struct StringViewer<'a> {
    iter: Peekable<Graphemes<'a>>,
    string: &'a str,
    index: usize,
}

impl<'a> StringViewer<'a> {
    pub(crate) fn new(string: &'a str) -> Self {
        Self {
            iter: UnicodeSegmentation::graphemes(string, true).peekable(),
            index: 0,
            string,
        }
    }

    pub(crate) fn next(&mut self) -> Option<&'a str> {
        let grapheme = self.iter.next()?;
        self.index += grapheme.len();
        Some(grapheme)
    }

    pub(crate) fn peek(&mut self) -> Option<&&'a str> {
        self.iter.peek()
    }

    pub(super) fn split_string(&self, slice_idx: (usize, usize)) -> &'a str {
        &self.string[slice_idx.0..slice_idx.1]
    }
}

pub fn extract_prelude<'a>(string: &'a str) -> ExtractionResult<'a, DisplayAttributes> {
    let mut string_viewer = StringViewer::new(string);

    let new_lines = &["\r\n", "\n"];

    consume_to_char(&mut string_viewer, &["-"]);
    consume_to_char(&mut string_viewer, new_lines);

    let mut display_attributes = vec![];
    while let Some(attribute) = extract_attribute(string, &mut string_viewer) {
        display_attributes.push(attribute);
    }

    consume_to_char(&mut string_viewer, new_lines);

    ExtractionResult { string_viewer, output_values: display_attributes }
}

fn extract_attribute<'a>(
    string: &'a str,
    chars: &mut StringViewer<'a>,
) -> Option<DisplayAttributes<'a>> {
    match chars.peek()? {
        &"\r\n" | &"\n" | &" " => {
            consume_all(chars, &[" ", "\r\n", "\n"])?;
            extract_attribute(string, chars)
        }
        &"-" => None,
        _ => {
            let name_idx = consume_to_char(chars, &[":"])?;
            let name = chars.split_string(name_idx);

            consume_all(chars, &[" "])?;

            let value_idx = consume_to_char(chars, &["\n", "\r\n", " "])?;
            let value = chars.split_string(value_idx);

            match name {
                "text_size" => Some(DisplayAttributes::TextSize(value.parse::<usize>().ok()?)),
                "text_alignment" => Some(DisplayAttributes::Alignment(match value {
                    "left" => Alignment::Left,
                    "center" => Alignment::Center,
                    "right" => Alignment::Right,
                    _ => {
                        eprintln!("Invalid value for alignment");
                        Alignment::Left
                    }
                })),
                "title" => Some(DisplayAttributes::Title(value)),
                _ => None,
            }
        }
    }
}

pub fn tokenize_string<'a>(mut string_viewer: StringViewer<'a>) -> ExtractionResult<'a, Token<'a>> {
    let mut tokens = vec![];

    while let Some(token) = next_token(&mut string_viewer) {
        tokens.push(token);
    }

    ExtractionResult { string_viewer, output_values: tokens }
}

fn next_token<'a>(chars: &mut StringViewer<'a>) -> Option<Token<'a>> {
    let whitespace_characters = &[" ", "\n", "\r\n"];
    return match chars.peek()? {
        &"!" => {
            chars.next()?;
            let text_idx = consume_to_char(chars, whitespace_characters)?;
            let text = chars.split_string(text_idx);
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
            let text_idx = consume_to_char(chars, &["$"])?;
            let text = chars.split_string(text_idx);
            Some(Token::Text(text))
        }
    };
}

mod tokenizing_helpers {
    use std::iter::Peekable;
    use unicode_segmentation::{Graphemes, UnicodeSegmentation};

    use super::StringViewer;

    pub(super) fn consume_to_char(
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

    pub(super) fn consume_all(
        chars: &mut StringViewer,
        consume_all: &[&str],
    ) -> Option<(usize, usize)> {
        let begin_index = chars.index;

        while let Some(char) = chars.peek() {
            if !consume_all.contains(char) {
                break;
            }

            chars.next()?;
        }
        Some((begin_index, chars.index))
    }

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
        assert!(
            false,
            "There should be a carriage return character present in the whitespcae characters"
        );
    }
}
