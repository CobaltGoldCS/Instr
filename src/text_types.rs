use std::io;

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;

use crate::tokenizer::Token;

#[derive(Clone, Debug)]
pub struct TextType<'a> {
    style: &'a str,
    text: &'a str,
}

impl<'a> TextType<'a> {
    pub fn new(style: &'a str, text: &'a str) -> TextType<'a> {
        TextType {
            style,
            text,
        }
    }

    pub fn to_lines(self) -> Vec<Line<'a>> {
        let split_lines = self.text.split('\n');
        let mut return_lines = vec![];

        let default_style = Style::default();
        for line in split_lines {
            return_lines.push(Line::styled(
                line,
                Self::return_style(default_style, self.style).unwrap(),
            ));
        }

        return_lines
    }

    fn return_style(default_style: Style, modifier: &str) -> io::Result<Style> {
        match modifier {
            "title" => Ok(default_style.add_modifier(Modifier::BOLD | Modifier::ITALIC | Modifier::SLOW_BLINK)),
            "warning" => Ok(default_style.fg(Color::White).bg(Color::Rgb(255, 100, 0))),
            "blinking" => Ok(default_style.add_modifier(Modifier::RAPID_BLINK)),
            "crossed-out" => Ok(default_style.add_modifier(Modifier::CROSSED_OUT)),
            "hidden" => Ok(default_style.add_modifier(Modifier::HIDDEN)),
            "normal" => Ok(default_style),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unsupported Text modifier {}", modifier),
            )),
        }
    }
}

pub fn from_tokens<'a, T>(default_string: &'a str, tokens: &mut T) -> io::Result<Vec<Line<'a>>>
where
    T: Iterator<Item = Token<'a>>,
{
    let mut text_types: Vec<Line> = vec![];

    let mut current_text_type: TextType = TextType {
        style: default_string,
        text: default_string,
    };

    for token in tokens {
        match token {
            Token::Start(text_type) => {
                current_text_type.style = text_type;
            }
            Token::Text(text) => {
                current_text_type.text = text;
            }
            Token::End => {
                text_types.extend(current_text_type.clone().to_lines());
            }
            Token::Whitespace(character) =>  match character {
                "\r\n" | "\n" => text_types.push(Line::from("\n")),
                _ => (),
            }
        }
    }
    Ok(text_types)
}

#[cfg(test)]
mod tests {
    use crate::text_types::TextType;

    use super::{from_tokens, Token};

    #[test]
    fn from_tokens_handles_proper_tokens() {
        let tokens = vec![Token::Start("title"), Token::Text("test"), Token::End];
        let mut binding = tokens.into_iter();
        let line_types = from_tokens("", &mut binding);

        assert_eq!(line_types.expect("Valid Line Type"), TextType::new("title", "test").to_lines())

    }

}
