use std::str::FromStr;

use indexmap::{IndexMap, IndexSet};
use winnow::{ascii::{alphanumeric1, multispace0}, combinator::repeat, error::{ContextError, ErrMode}, PResult, Parser};

use super::{traits, attributes, values::Value};

#[derive(Clone, Debug, PartialEq)]
pub struct Tag {
    pub name: String,
    pub traits: IndexSet<String>,
    pub attributes: IndexMap<String, Value>,
    pub children: Vec<Tag>
}

impl Tag {
    pub fn new(name: &str) -> Self {
        Tag { name: name.into(), traits: IndexSet::new(), attributes: IndexMap::new(), children: vec![] }
    }

    pub fn set(&mut self, key: &str) {
        self.traits.insert(key.into());
    }

    pub fn with(&mut self, key: &str, value: Value) {
        self.attributes.insert(key.into(), value);
    }

    pub fn child(&mut self, tag: Tag) {
        self.children.push(tag);
    }
}

pub fn single(s: &mut &str) -> PResult<Tag> {
    let _ = multispace0.parse_next(s)?;
    let _ = "<".parse_next(s)?;
    let name: String = alphanumeric1.parse_next(s)?.into();

    let traits = traits::many.parse_next(s)?;

    let attributes = attributes::many.parse_next(s)?;
    let _ = multispace0.parse_next(s)?;

    let out: PResult<&str> = "/>".parse_next(s);
    if out.is_ok() {
        return Ok(Tag { name, traits, attributes, children: vec![] })
    }

    let _ = ">".parse_next(s)?;

    let children = many.parse_next(s)?;

    let _ = "</".parse_next(s)?;
    let _ = name.as_str().parse_next(s)?;
    let _ = ">".parse_next(s)?;

    Ok(Tag { name, traits, attributes, children })
}

fn many<'s>(s: &mut &'s str) -> PResult<Vec<Tag>> {
    repeat(0.., single).fold(Vec::new, |mut acc: Vec<Tag>, item| {
        acc.push(item);
        acc
    }).parse_next(s)
}

impl<'a> FromStr for Tag {
    type Err = ErrMode<ContextError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        single(&mut s.clone())
    }
}

#[cfg(test)]
mod test {
    use indexmap::IndexMap;
    use indexmap::IndexSet;
    use winnow::Parser;

    use super::Tag;

    use super::many;

    #[test]
    fn test_parse_tag() {
        let expected = Tag { name: "button".into(), traits: IndexSet::new(), attributes: IndexMap::new(), children: vec![] };
        assert_eq!(Ok(expected), "<button></button>".parse());
    }

    #[test]
    fn test_parse_short_tag() {
        let expected = Tag { name: "button".into(), traits: IndexSet::new(), attributes: IndexMap::new(), children: vec![] };
        assert_eq!(Ok(expected), "<button />".parse());
    }

    #[test]
    fn test_parse_children() {
        let parsed = many.parse_next(&mut "<foo></foo><bar></bar>");
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