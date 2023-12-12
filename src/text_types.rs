use std::fs::File;
use std::io::{self, prelude::*};

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;

use crate::tokenizer::{tokenize_string, Token};

#[derive(Clone)]
pub struct TextType {
    style: String,
    text: String,
}

impl TextType {
    pub fn new(style: String, text: String) -> TextType {
        TextType { style, text }
    }

    pub fn to_lines<'a>(self) -> Vec<Line<'a>> {
        let split_lines = self.text.split('\n');
        let mut return_lines = vec![];
        for line in split_lines {
            return_lines.push(Line::styled(
                line.to_string(),
                Self::return_style(Style::default(), &self.style).unwrap(),
            ));
        }

        return_lines
    }

    fn return_style(default_style: Style, modifier: &str) -> io::Result<Style> {
        return match modifier {
            "title" => Ok(default_style.add_modifier(Modifier::ITALIC)),
            "warning" => Ok(default_style.fg(Color::White).bg(Color::Rgb(255, 100, 0))),
            "normal" => Ok(default_style),
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

    let tokens = tokenize_string(file_string);

    let mut current_text_type: TextType = TextType {
        style: String::new(),
        text: String::new(),
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
            Token::Whitespace(c) => current_text_type.text += &c.to_string(),
        }
    }
    Ok(text_types)
}
