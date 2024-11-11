use indexmap::IndexMap;
use winnow::{ascii::{alphanumeric1, space0}, combinator::repeat, PResult, Parser};

use super::values::{parse_value, Value};

fn parse_attribute<'s>(s: &mut &'s str) -> PResult<(&'s str, Value)> {
    let _ = space0.parse_next(s)?;
    let key = alphanumeric1.parse_next(s)?;
    let _ = "=".parse_next(s)?;
    let value = parse_value.parse_next(s)?;
    Ok((key, value))
}

pub fn parse_attributes<'s>(s: &mut &'s str) -> PResult<IndexMap<String, Value>> {
    repeat(0.., parse_attribute).fold(IndexMap::new, |mut acc: IndexMap<_, _>, item| {
        acc.insert(item.0.into(), item.1.into());
        acc
    }).parse_next(s)
}

#[cfg(test)]
mod test {
    use indexmap::IndexMap;
    use winnow::Parser;

    use super::{parse_attributes, Value};

    use super::parse_attribute;

    #[test]
    fn test_parse_attr() {
        let mut input = "foo=\"bar\"";
        let parsed = parse_attribute.parse_next(&mut input);
        assert_eq!(input, "");
        assert!(parsed.is_ok());
        let pair = parsed.unwrap();
        assert_eq!(pair.0, "foo");
        assert_eq!(pair.1, Value::String("bar".into()));
    }

    #[test]
    fn test_parse_attrs() {
        let mut input = "foo=\"bar\" bo=\"burnham\"";
        let parsed = parse_attributes.parse_next(&mut input);
        assert_eq!(input, "");
        assert!(parsed.is_ok());

        let mut expected: IndexMap<String, Value> = IndexMap::new();
        expected.insert("foo".into(), Value::String("bar".into()));
        expected.insert("bo".into(), Value::String("burnham".into()));

        let actual = parsed.unwrap();
        assert_eq!(expected, actual);
    }
}