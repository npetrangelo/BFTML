use std::ops::Range;

use winnow::{ascii::{digit0, digit1}, combinator::alt, token::take_till, PResult, Parser};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Range(Range<i64>),
}

fn string(s: &mut &str) -> PResult<String> {
    let _ = '"'.parse_next(s)?;
    let result = take_till(0.., ['"']).parse_next(s)?;
    let _ = '"'.parse_next(s)?;
    Ok(result.into())
}

fn int(s: &mut &str) -> PResult<i64> {
    let result = digit1.parse_next(s)?;
    Ok(result.parse::<i64>().expect("Should only be digits"))
}

fn float(s: &mut &str) -> PResult<f64> {
    let left = digit0.parse_next(s)?;
    let _ = ".".parse_next(s)?;
    let right = digit0.parse_next(s)?;
    Ok(format!("{left}.{right}").parse::<f64>().expect("Should be digits"))
}

fn bool(s: &mut &str) -> PResult<bool> {
    let result = alt(["true", "false"]).parse_next(s)?;
    match result {
        "true" => Ok(true),
        "false" => Ok(false),
        &_ => panic!("Result can only be true or false")
    }
}

fn range(s: &mut &str) -> PResult<Range<i64>> {
    let start = int.parse_next(s)?;
    let _ = "..".parse_next(s)?;
    let end = int.parse_next(s)?;
    Ok(start..end)
}

pub fn value(s: &mut &str) -> PResult<Value> {
    alt((
        string.map(|s| Value::String(s)),
        range.map(|r| Value::Range(r)),
        float.map(|f| Value::Float(f)),
        int.map(|i| Value::Int(i)),
        bool.map(|b| Value::Bool(b))))
    .parse_next(s)
}

#[cfg(test)]
mod test {
    use winnow::Parser;
    use super::{Value, value};

    #[test]
    fn test_string() {
        let expected = Value::String("foo".into());
        assert_eq!(expected, value.parse_next(&mut "\"foo\"").unwrap());
    }

    #[test]
    fn test_int() {
        let expected = Value::Int(42);
        assert_eq!(expected, value.parse_next(&mut "42").unwrap());
    }

    #[test]
    fn test_float() {
        let expected = Value::Float(3.14);
        assert_eq!(expected, value.parse_next(&mut "3.14").unwrap());
    }

    #[test]
    fn test_bool() {
        let expected = Value::Bool(true);
        assert_eq!(expected, value.parse_next(&mut "true").unwrap());
    }

    #[test]
    fn test_range() {
        let expected = Value::Range(0..42);
        assert_eq!(expected, value.parse_next(&mut "0..42").unwrap());
    }
}
