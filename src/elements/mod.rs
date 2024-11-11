use std::str::FromStr;

use iced::{widget::{button, Button, Column}, Element};
use regex::Regex;
use winnow::{token::{literal, take_till}, IResult, Parser};

pub enum Tag<'a, Message: Clone + 'a> {
    Button(Button<'a, Message>),
    Column(Column<'a, Message>),
    Error(ParseError<'a>)
}

pub enum ParseError<'a> {
    InvalidTag { tag: &'a str },
    AttributeNotFound { attribute: &'a str },
    InvalidAttribute { attribute: &'a str },
    InvalidKey { key: &'a str },
    InvalidValue { value: &'a str },
    InvalidChildren(String)
}

pub type ParseResult<'a, T> = Result<T, ParseError<'a>>;

impl<'a> From<ParseError<'a>> for String {
    fn from(value: ParseError) -> Self {
        use ParseError::*;
        match value {
            InvalidTag {tag} => format!("{tag} is not a tag"),
            AttributeNotFound { attribute } => format!("{attribute} was not found"),
            InvalidAttribute { attribute } => format!("{attribute} is not an attribute"),
            InvalidKey{key} => format!("{key} is not a key"),
            InvalidValue{value} => format!("{value} is not a value"),
            InvalidChildren(s) => s,
        }
    }
}

impl<'a, Message: Clone + 'a> From<ParseError<'a>> for Element<'a, Message> {
    fn from(value: ParseError) -> Self {
        todo!("ParseErrors don't yet render")
    }
}

impl<'a, Message: Clone + 'a> From<Tag<'a, Message>> for Element<'a, Message> {
    fn from(tag: Tag<'a, Message>) -> Self {
        use Tag::*;
        match tag {
            Button(button) => button.into(),
            Column(column) => column.into(),
            Error(error) => error.into(),
        }
    }
}

impl<'a, M: Clone + 'a> FromStr for Tag<'a, M> {
    type Err = ParseError<'a>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

/// Looks for attribute with the given key in the given string.
fn parse_attribute<'a, T: From<&'a str>>(string: &'a str, key: &str) -> ParseResult<'a, T> {
    let re = format!(r".*{key}=([a-zA-Z0-9]+).*");
    let re = Regex::new(&re).expect("Valid regex string");
    match re.find(string) {
        Some(_) => todo!(),
        None => todo!(),
    }
    Ok("".into())
}

fn parse_tag<'a, M: Clone + 'a>(s: &str) -> IResult<&str, Tag<'a, M>> {
    let (s, parsed) = literal("<").parse_peek(s)?;
    println!("{parsed} out of {s}");
    let (s, parsed) = take_till(0.., [' ', '>']).parse_peek(s)?;
    println!("{parsed} out of {s}");
    match parsed {
        "button" => Ok(("", Tag::Button::<M>(button("")))),
        &_ => todo!()
    }
}

#[cfg(test)]
mod test {
    use winnow::{token::literal, IResult, Parser};

    use super::{parse_tag, Tag};

    #[test]
    fn test_parse() {
        let out: IResult<&str, &str> = literal("<").and_then(literal("button")).parse_peek("<button");
        println!("{:?}", out);
    }

    #[test]
    fn test_parse_tag<'a>() {
        let (s, parsed) = parse_tag::<()>("<button>").expect("should work");
    }
}