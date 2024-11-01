mod button;
mod column;

use std::fmt::{self, format};

pub use button::Button;
pub use column::Column;

pub enum Tag<'a, Message: Clone + 'a> {
    Button(Button<'a, Message>),
    Column(Column<'a, Message>),
    Error(ParseError<'a>)
}

pub enum ParseError<'a> {
    InvalidTag { tag: &'a str },
    InvalidAttributeKey { key: &'a str },
    InvalidAttributeValue { value: &'a str },
    InvalidChildren(String)
}

pub type ParseResult<'a, T> = Result<T, ParseError<'a>>;

impl<'a> From<ParseError<'a>> for String {
    fn from(value: ParseError) -> Self {
        match value {
            ParseError::InvalidTag {tag} => format!("{tag} is not a tag"),
            ParseError::InvalidAttributeKey{key} => format!("{key} is not a key"),
            ParseError::InvalidAttributeValue{value} => format!("{value} is not a value"),
            ParseError::InvalidChildren(s) => s,
        }
    }
}

impl<'a, Message: Clone + 'a> From<ParseError<'a>> for iced::Element<'a, Message> {
    fn from(value: ParseError) -> Self {
        todo!("ParseErrors don't yet render")
    }
}

impl<'a, Message: Clone + 'a> From<Tag<'a, Message>> for iced::Element<'a, Message> {
    fn from(tag: Tag<'a, Message>) -> Self {
        match tag {
            Tag::Button(button) => button.into(),
            Tag::Column(column) => column.into(),
            Tag::Error(error) => error.into(),
        }
    }
}