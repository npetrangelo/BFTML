use indexmap::IndexMap;
use winnow::{ascii::{alphanumeric1, space0}, combinator::repeat, PResult, Parser};

use super::values::{value, Value};

pub fn single<'s>(s: &mut &'s str) -> PResult<(&'s str, Option<Value>)> {
    let _ = space0.parse_next(s)?;
    let key = alphanumeric1.parse_next(s)?;
    let equals: PResult<&'s str> = "=".parse_next(s);
    if equals.is_err() {
        return Ok((key, None));
    }
    let value = value.parse_next(s)?;
    Ok((key, Some(value)))
}

pub fn many<'s>(s: &mut &'s str) -> PResult<IndexMap<String, Option<Value>>> {
    repeat(0.., single).fold(IndexMap::new, |mut acc: IndexMap<_, _>, item| {
        acc.insert(item.0.into(), item.1.into());
        acc
    }).parse_next(s)
}

#[cfg(test)]
mod test {
    use indexmap::IndexMap;
    use winnow::Parser;

    use super::{many, Value};

    use super::single;

    #[test]
    fn test_parse_attr() {
        let mut input = "foo=\"bar\"";
        let parsed = single.parse_next(&mut input);
        assert_eq!(input, "");
        assert!(parsed.is_ok());
        let pair = parsed.unwrap();
        assert_eq!(pair.0, "foo");
        assert_eq!(pair.1, Some(Value::String("bar".into())));
    }

    #[test]
    fn test_parse_attrs() {
        let mut input = "foo=\"bar\" bo=\"burnham\"";
        let parsed = many.parse_next(&mut input);
        assert_eq!(input, "");
        assert!(parsed.is_ok());

        let mut expected: IndexMap<String, Option<Value>> = IndexMap::new();
        expected.insert("foo".into(), Some(Value::String("bar".into())));
        expected.insert("bo".into(), Some(Value::String("burnham".into())));

        let actual = parsed.unwrap();
        assert_eq!(expected, actual);
    }
}