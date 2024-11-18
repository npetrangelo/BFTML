use indexmap::IndexSet;
use winnow::{ascii::{alphanumeric1, space0}, combinator::{not, repeat}, PResult, Parser};

pub fn single<'s>(s: &mut &'s str) -> PResult<&'s str> {
    let _ = space0.parse_next(s)?;
    let key = alphanumeric1.parse_next(s)?;
    // Do not greedily consume attribute key
    let _ = not("=").parse_next(s)?;
    Ok(key)
}

pub fn many<'s>(s: &mut &'s str) -> PResult<IndexSet<String>> {
    repeat(0.., single).fold(IndexSet::new, |mut acc: IndexSet<_>, item| {
        acc.insert(item.into());
        acc
    }).parse_next(s)
}

#[cfg(test)]
mod test {
    use indexmap::IndexSet;
    use winnow::Parser;

    use super::many;

    use super::single;

    #[test]
    fn test_parse_trait() {
        let mut input = "foo";
        let parsed = single.parse_next(&mut input);
        assert_eq!(input, "");
        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap(), "foo");
    }

    #[test]
    fn test_parse_traits() {
        let mut input = "foo bo";
        let parsed = many.parse_next(&mut input);
        assert_eq!(input, "");
        assert!(parsed.is_ok());

        let mut expected: IndexSet<String> = IndexSet::new();
        expected.insert("foo".into());
        expected.insert("bo".into());

        assert_eq!(expected, parsed.unwrap());
    }
}