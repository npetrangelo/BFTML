use std::str::FromStr;

use indexmap::{IndexMap, IndexSet};
use winnow::{ascii::{alphanumeric1, multispace0}, combinator::{alt, not, repeat}, error::{ContextError, ErrMode}, token::take_till, Result, Parser};

use super::{traits, attributes, values::Value};

#[derive(Clone, Debug, PartialEq)]
enum Inner {
    None,
    Content(String),
    Children(Vec<Tag>)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tag {
    pub name: String,
    pub traits: IndexSet<String>,
    pub attributes: IndexMap<String, Value>,
    pub inner: Inner
}

impl Tag {
    pub fn new(name: &str) -> Self {
        Tag { name: name.into(), traits: IndexSet::new(), attributes: IndexMap::new(), inner: Inner::None }
    }

    pub fn set(&mut self, key: &str) {
        self.traits.insert(key.into());
    }

    pub fn with(&mut self, key: &str, value: Value) {
        self.attributes.insert(key.into(), value);
    }

    pub fn content(&mut self, content: &str) {
        self.inner = Inner::Content(content.into())
    }

    pub fn children(&mut self, tags: Vec<Tag>) {
        self.inner = Inner::Children(tags);
    }
}

pub fn single(s: &mut &str) -> Result<Tag> {
    let _ = multispace0.parse_next(s)?;
    let _ = "<".parse_next(s)?;
    let name: String = alphanumeric1.parse_next(s)?.into();

    let traits = traits::many.parse_next(s)?;

    let attributes = attributes::many.parse_next(s)?;
    let _ = multispace0.parse_next(s)?;

    let out: Result<&str> = "/>".parse_next(s);
    if out.is_ok() {
        return Ok(Tag { name, traits, attributes, inner: Inner::None })
    }

    let _ = ">".parse_next(s)?;


    let inner = alt((
        many.map(|children| Inner::Children(children)),
        content.map(|content| Inner::Content(content.into())),
    )).parse_next(s)?;
    println!("{:?}", inner);

    let _ = "</".parse_next(s)?;
    let _ = name.as_str().parse_next(s)?;
    let _ = ">".parse_next(s)?;

    Ok(Tag { name, traits, attributes, inner })
}

fn many<'s>(s: &mut &'s str) -> Result<Vec<Tag>> {
    repeat(1.., single).fold(Vec::new, |mut acc: Vec<Tag>, item| {
        acc.push(item);
        acc
    }).parse_next(s)
}

fn content<'s>(s: &mut &'s str) -> Result<&'s str> {
    take_till(0.., '<').parse_next(s)
}

impl<'a> FromStr for Tag {
    type Err = ContextError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        single(&mut s.clone())
    }
}

#[cfg(test)]
mod test {
    use indexmap::IndexMap;
    use indexmap::IndexSet;
    use winnow::Parser;

    use crate::parser::tags::single;

    use super::content;
    use super::Inner::*;
    use super::Tag;

    use super::many;

    #[test]
    fn test_parse_tag() {
        let expected = Tag { name: "button".into(), traits: IndexSet::new(), attributes: IndexMap::new(), inner: Content("".into()) };
        assert_eq!(Ok(expected), "<button></button>".parse());
    }

    #[test]
    fn test_parse_short_tag() {
        let expected = Tag { name: "button".into(), traits: IndexSet::new(), attributes: IndexMap::new(), inner: None };
        assert_eq!(Ok(expected), "<button />".parse());
    }

    #[test]
    fn test_parse_children() {
        let parsed = many.parse_next(&mut "<foo /><bar />");
        let expected = vec![Tag::new("foo"), Tag::new("bar")];
        assert_eq!(expected, parsed.unwrap())
    }

    #[test]
    fn test_content() {
        let parsed = content.parse_next(&mut "Banana<");
        let expected = "Banana";
        assert_eq!(expected, parsed.unwrap())
    }

    #[test]
    fn test_parse_tag_children() {
        let parsed = "<foo><bar /></foo>".parse();
        let mut expected = Tag::new("foo");
        expected.children(vec![Tag::new("bar")]);
        assert_eq!(expected, parsed.unwrap())
    }

    #[test]
    fn test_parse_tag_content() {
        let mut input = "<foo>Bananas</foo>";
        let parsed = single.parse_next(&mut input);
        let mut expected = Tag::new("foo");
        expected.content("Bananas");
        println!("{input}");
        assert_eq!(expected, parsed.unwrap())
    }

    #[test]
    fn test_parse_err() {
        let parsed = "<button".parse::<Tag>();
        assert!(parsed.is_err());
    }
}