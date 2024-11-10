use iced::widget::button;
use indexmap::IndexMap;
use winnow::{ascii::{alphanumeric1, multispace0}, combinator::{alt, repeat}, error::{ErrorKind, InputError}, stream::ContainsToken, token::{literal, take_till, take_until, take_while}, IResult, PResult, Parser};

#[derive(Debug, PartialEq)]
enum Token {
    TagOpen,
    TagClose,
    TagElement(String),
    Attribute((String, String))
}

impl ContainsToken<Token> for Token {
    #[inline(always)]
    fn contains_token(&self, token: Token) -> bool {
        *self == token
    }
}

impl ContainsToken<Token> for &[Token] {
    #[inline]
    fn contains_token(&self, token: Token) -> bool {
        self.iter().any(|t| *t == token)
    }
}

impl<const LEN: usize> ContainsToken<Token> for &[Token; LEN] {
    #[inline]
    fn contains_token(&self, token: Token) -> bool {
        self.iter().any(|t| *t == token)
    }
}

impl<const LEN: usize> ContainsToken<Token> for [Token; LEN] {
    #[inline]
    fn contains_token(&self, token: Token) -> bool {
        self.iter().any(|t| *t == token)
    }
}

fn parse_attribute<'s>(s: &mut &'s str) -> PResult<(&'s str, &'s str)> {
    let _ = multispace0.parse_next(s)?;
    let key = alphanumeric1.parse_next(s)?;
    let _ = "=".parse_next(s)?;
    let value = alphanumeric1.parse_next(s)?;
    Ok((key, value))
}

fn parse_attributes<'s>(s: &mut &'s str) -> PResult<IndexMap<&'s str, &'s str>> {
    repeat(0.., parse_attribute).fold(IndexMap::new, |mut acc: IndexMap<_, _>, item| {
        acc.insert(item.0, item.1);
        acc
    }).parse_next(s)
}

#[derive(Debug, PartialEq)]
struct Tag<'a> {
    name: &'a str,
    attributes: IndexMap<&'a str, &'a str>,
    children: Vec<Tag<'a>>
}

fn parse_tag<'a>(s: &mut &'a str) -> PResult<Tag<'a>> {
    let _ = "<".parse_next(s)?;
    let name = alphanumeric1.parse_next(s)?;

    let attributes = parse_attributes.parse_next(s)?;

    let _ = ">".parse_next(s)?;
    let children = vec![];
    Ok(Tag { name, attributes, children })
}

#[cfg(test)]
mod test {
    use indexmap::IndexMap;
    use winnow::Parser;

    use crate::elements::parser::parse_attributes;
    use crate::elements::parser::Tag;

    use super::parse_attribute;
    use super::parse_tag;

    #[test]
    fn test_parse_tag() {
        let mut input = "<button>";
        let parsed = parse_tag.parse_next(&mut input);
        assert_eq!(input, "");
        assert!(parsed.is_ok());
        let tag = parsed.unwrap();

        let expected = Tag { name: "button", attributes: IndexMap::new(), children: vec![] };
        assert_eq!(tag, expected);
    }

    #[test]
    fn test_parse_attr() {
        let mut input = "foo=bar";
        let parsed = parse_attribute.parse_next(&mut input);
        assert_eq!(input, "");
        assert!(parsed.is_ok());
        let pair = parsed.unwrap();
        assert_eq!(pair.0, "foo");
        assert_eq!(pair.1, "bar");
    }

    #[test]
    fn test_parse_attrs() {
        let mut input = "foo=bar bo=burnham";
        let parsed = parse_attributes.parse_next(&mut input);
        assert_eq!(input, "");
        assert!(parsed.is_ok());

        let mut expected = IndexMap::new();
        expected.insert("foo", "bar");
        expected.insert("bo", "burnham");

        let actual = parsed.unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_tag_attr() {
        let mut input = "<button foo=bar>";
        let parsed = parse_tag.parse_next(&mut input);
        assert_eq!(input, "");
        assert!(parsed.is_ok());
        let tag = parsed.unwrap();

        let mut attributes = IndexMap::new();
        attributes.insert("foo", "bar");
        let expected = Tag { name: "button", attributes, children: vec![] };
        assert_eq!(tag, expected);
    }

    #[test]
    fn test_parse_err() {
        let parsed = parse_tag.parse_next(&mut "<button");
        assert!(parsed.is_err());
    }
}