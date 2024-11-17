use std::str::FromStr;

use indexmap::IndexMap;
use winnow::{ascii::{alphanumeric1, multispace0}, combinator::repeat, error::{ContextError, ErrMode}, PResult, Parser};

use super::{attributes, values::Value};

#[derive(Clone, Debug, PartialEq)]
pub struct Tag {
    pub name: String,
    pub attributes: IndexMap<String, Option<Value>>,
    pub children: Vec<Tag>
}

impl Tag {
    pub fn new(name: &str) -> Self {
        Tag { name: name.into(), attributes: IndexMap::new(), children: vec![] }
    }

    pub fn set(&mut self, key: &str) {
        self.attributes.insert(key.into(), None);
    }

    pub fn with(&mut self, key: &str, value: Value) {
        self.attributes.insert(key.into(), Some(value));
    }

    pub fn child(&mut self, tag: Tag) {
        self.children.push(tag);
    }
}

fn tag<'a>(s: &mut &'a str) -> PResult<Tag> {
    let _ = multispace0.parse_next(s)?;
    let _ = "<".parse_next(s)?;
    let name: String = alphanumeric1.parse_next(s)?.into();

    let attributes = attributes::many.parse_next(s)?;
    let _ = multispace0.parse_next(s)?;

    let out: PResult<&str> = "/>".parse_next(s);
    if out.is_ok() {
        return Ok(Tag { name, attributes, children: vec![] })
    }

    let _ = ">".parse_next(s)?;

    let children = children.parse_next(s)?;

    let _ = "</".parse_next(s)?;
    let _ = name.as_str().parse_next(s)?;
    let _ = ">".parse_next(s)?;

    Ok(Tag { name, attributes, children })
}

fn children<'s>(s: &mut &'s str) -> PResult<Vec<Tag>> {
    repeat(0.., tag).fold(Vec::new, |mut acc: Vec<Tag>, item| {
        acc.push(item);
        acc
    }).parse_next(s)
}

impl<'a> FromStr for Tag {
    type Err = ErrMode<ContextError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        tag(&mut s.clone())
    }
}

#[cfg(test)]
mod test {
    use indexmap::IndexMap;
    use winnow::Parser;

    use super::Tag;

    use super::children;

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
    fn test_parse_children() {
        let parsed = children.parse_next(&mut "<foo></foo><bar></bar>");
        let expected = vec![Tag::new("foo"), Tag::new("bar")];
        assert_eq!(expected, parsed.unwrap())
    }

    #[test]
    fn test_parse_tag_children() {
        let parsed = "<foo><bar></bar></foo>".parse::<Tag>();
        let mut expected = Tag::new("foo");
        expected.child(Tag::new("bar"));
        assert_eq!(expected, parsed.unwrap())
    }

    #[test]
    fn test_parse_err() {
        let parsed = "<button".parse::<Tag>();
        assert!(parsed.is_err());
    }
}