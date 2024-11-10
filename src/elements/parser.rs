use std::str::FromStr;

use indexmap::IndexMap;
use winnow::{ascii::{alphanumeric1, multispace0, space0}, combinator::repeat, error::{ContextError, ErrMode}, PResult, Parser};

fn parse_attribute<'s>(s: &mut &'s str) -> PResult<(&'s str, &'s str)> {
    let _ = space0.parse_next(s)?;
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

fn parse_children<'s>(s: &mut &'s str) -> PResult<Vec<Tag>> {
    repeat(0.., parse_tag).fold(Vec::new, |mut acc: Vec<Tag>, item| {
        acc.push(item);
        acc
    }).parse_next(s)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tag {
    pub name: String,
    pub attributes: IndexMap<String, String>,
    pub children: Vec<Tag>
}

impl Tag {
    pub fn new(name: &str) -> Self {
        Tag { name: name.into(), attributes: IndexMap::new(), children: vec![] }
    }

    pub fn with(&mut self, key: &str, value: &str) {
        self.attributes.insert(key.into(), value.into());
    }

    pub fn child(&mut self, tag: Tag) {
        self.children.push(tag);
    }
}

fn parse_tag<'a>(s: &mut &'a str) -> PResult<Tag> {
    let _ = multispace0.parse_next(s)?;
    let _ = "<".parse_next(s)?;
    let name: String = alphanumeric1.parse_next(s)?.into();

    let attributes = parse_attributes.parse_next(s)?;
    let _ = multispace0.parse_next(s)?;

    let out: PResult<&str> = "/>".parse_next(s);
    if out.is_ok() {
        return Ok(Tag { name, attributes, children: vec![] })
    }

    let _ = ">".parse_next(s)?;

    let children = parse_children.parse_next(s)?;

    let _ = "</".parse_next(s)?;
    let _ = name.as_str().parse_next(s)?;
    let _ = ">".parse_next(s)?;

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
    use super::parse_children;
    use super::parse_tag;

    #[test]
    fn test_parse_tag() {
        let expected = Tag { name: "button".into(), attributes: IndexMap::new(), children: vec![] };
        assert_eq!(Ok(expected), "<button></button>".parse());
    }

    #[test]
    fn test_parse_short_tag() {
        let expected = Tag { name: "button".into(), attributes: IndexMap::new(), children: vec![] };
        assert_eq!(Ok(expected), "<button />".parse());
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
        let mut input = "<button foo=bar></button>";
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
    fn test_parse_children() {
        let parsed = parse_children.parse_next(&mut "<foo></foo><bar></bar>");
        let expected = vec![Tag::new("foo"), Tag::new("bar")];
        assert_eq!(expected, parsed.unwrap())
    }

    #[test]
    fn test_parse_tag_children() {
        let parsed = parse_tag.parse_next(&mut "<foo><bar></bar></foo>");
        let mut expected = Tag::new("foo");
        expected.child(Tag::new("bar"));
        assert_eq!(expected, parsed.unwrap())
    }

    #[test]
    fn test_altogether() {
        let parsed = parse_tag.parse_next(&mut "<button foo=bar><foo></foo></button>");
        let mut expected = Tag::new("button");
        expected.with("foo", "bar");
        expected.child(Tag::new("foo"));
        assert_eq!(expected, parsed.unwrap())
    }

    #[test]
    fn test_parse_err() {
        let parsed = parse_tag.parse_next(&mut "<button");
        assert!(parsed.is_err());
    }
}