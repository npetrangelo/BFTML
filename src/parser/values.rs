use winnow::{ascii::{digit0, digit1}, combinator::alt, token::take_till, PResult, Parser};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

fn parse_string(s: &mut &str) -> PResult<Value> {
    let _ = '"'.parse_next(s)?;
    let result = take_till(0.., ['"']).parse_next(s)?;
    let _ = '"'.parse_next(s)?;
    Ok(Value::String(result.into()))
}

fn parse_int(s: &mut &str) -> PResult<Value> {
    let result = digit1.parse_next(s)?;
    Ok(Value::Int(result.parse::<i64>().expect("Should only be digits")))
}

fn parse_float(s: &mut &str) -> PResult<Value> {
    let left = digit0.parse_next(s)?;
    let _ = ".".parse_next(s)?;
    let right = digit0.parse_next(s)?;
    Ok(Value::Float(format!("{left}.{right}").parse::<f64>().expect("Should be digits")))
}

fn parse_bool(s: &mut &str) -> PResult<Value> {
    let result = alt(["true", "false"]).parse_next(s)?;
    match result {
        "true" => Ok(Value::Bool(true)),
        "false" => Ok(Value::Bool(false)),
        &_ => panic!("Result can only be true or false")
    }
}

pub fn parse_value(s: &mut &str) -> PResult<Value> {
    alt([parse_string, parse_float, parse_int, parse_bool]).parse_next(s)
}

#[cfg(test)]
mod test {
    use winnow::Parser;
    use super::{Value, parse_value};

    #[test]
    fn test_string() {
        let expected = Value::String("foo".into());
        assert_eq!(expected, parse_value.parse_next(&mut "\"foo\"").unwrap());
    }

    #[test]
    fn test_int() {
        let expected = Value::Int(42);
        assert_eq!(expected, parse_value.parse_next(&mut "42").unwrap());
    }

    #[test]
    fn test_float() {
        let expected = Value::Float(3.14);
        assert_eq!(expected, parse_value.parse_next(&mut "3.14").unwrap());
    }

    #[test]
    fn test_bool() {
        let expected = Value::Bool(true);
        assert_eq!(expected, parse_value.parse_next(&mut "true").unwrap());
    }
}