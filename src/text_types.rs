use std::fs::File;
use std::io::{self, prelude::*};

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;

use crate::tokenizer::{tokenize_string, Token};

#[derive(Clone)]
pub struct TextType {
    style: Style,
    text: String,
}

impl TextType {
    pub fn new(style: Style, text: String) -> TextType {
        TextType { style, text }
    }

    pub fn to_lines<'a>(self) -> Vec<Line<'a>> {
        let split_lines = self.text.split('\n');
        let mut return_lines = vec![];
        for line in split_lines {
            return_lines.push(Line::styled(line.to_string(), self.style));
        }

        return_lines
    }

    pub fn from_modifier(default_style: Style, modifier: &str) -> io::Result<TextType> {
        return match modifier {
            "title" => Ok(TextType::new(
                default_style.add_modifier(Modifier::ITALIC),
                String::new(),
            )),
            "warning" => Ok(TextType::new(
                default_style.fg(Color::White).bg(Color::Rgb(255, 100, 0)),
                String::new(),
            )),
            "normal" => Ok(TextType::new(default_style, String::new())),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unsupported Text modifier {}", modifier),
            )),
        };
    }
}

pub fn convert_text_types(path: &str) -> io::Result<Vec<Line>> {
    let mut text_types: Vec<Line> = vec![];
    let mut file = File::open(path).expect(&format!("{} does not exist", path));

    let mut file_string = String::new();
    file.read_to_string(&mut file_string)?;

    let mut tokens = tokenize_string(file_string).into_iter();

    let mut current_text_type: TextType = TextType {
        style: Style::default(),
        text: String::new(),
    };

    while let Some(token) = tokens.next() {
        match token {
            Token::Start(text_type) => {
                current_text_type = TextType::from_modifier(Style::default(), &text_type)?;
            }
            Token::Text(text) => {
                current_text_type.text = text;
            }

            Token::End => {
                text_types.extend(current_text_type.clone().to_lines());
            }
        }
    }
    Ok(text_types)
}
