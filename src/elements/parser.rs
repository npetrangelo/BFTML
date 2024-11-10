use std::str::FromStr;

use indexmap::IndexMap;
use winnow::{ascii::{alphanumeric1, multispace0}, combinator::repeat, error::{ContextError, ErrMode}, PResult, Parser};

#[derive(Debug, PartialEq)]
enum Token {
    TagOpen,
    TagClose,
    TagElement(String),
    Attribute((String, String))
}

fn parse_attribute<'s>(s: &mut &'s str) -> PResult<(&'s str, &'s str)> {
    let _ = multispace0.parse_next(s)?;
    let key = alphanumeric1.parse_next(s)?;
    let _ = "=".parse_next(s)?;
    let value = alphanumeric1.parse_next(s)?;
    Ok((key, value))
}

fn parse_attributes<'s>(s: &mut &'s str) -> PResult<IndexMap<String, String>> {
    repeat(0.., parse_attribute).fold(IndexMap::new, |mut acc: IndexMap<_, _>, item| {
        acc.insert(item.0.into(), item.1.into());
        acc
    }).parse_next(s)
}

#[derive(Debug, PartialEq)]
pub struct Tag {
    pub name: String,
    pub attributes: IndexMap<String, String>,
    pub children: Vec<Tag>
}

fn parse_tag<'a>(s: &mut &'a str) -> PResult<Tag> {
    let _ = "<".parse_next(s)?;
    let name = alphanumeric1.parse_next(s)?.into();

    let attributes = parse_attributes.parse_next(s)?;

    let _ = ">".parse_next(s)?;
    let children = vec![];
    Ok(Tag { name, attributes, children })
}

impl<'a> FromStr for Tag {
    type Err = ErrMode<ContextError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_tag(&mut s.clone())
    }
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
        let expected = Tag { name: "button".into(), attributes: IndexMap::new(), children: vec![] };
        assert_eq!(Ok(expected), "<button>".parse());
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

        let mut expected: IndexMap<String, String> = IndexMap::new();
        expected.insert("foo".into(), "bar".into());
        expected.insert("bo".into(), "burnham".into());

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

        let mut attributes: IndexMap<String, String> = IndexMap::new();
        attributes.insert("foo".into(), "bar".into());
        let expected = Tag { name: "button".into(), attributes, children: vec![] };
        assert_eq!(tag, expected);
    }

    #[test]
    fn test_parse_err() {
        let parsed = parse_tag.parse_next(&mut "<button");
        assert!(parsed.is_err());
    }
}