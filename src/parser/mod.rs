pub mod attributes;
pub mod traits;
pub mod tags;
pub mod values;

#[cfg(test)]
mod test {
    use winnow::Parser;

    use crate::parser::{tags::{single, Tag}, values::Value};

    #[test]
    fn test_altogether() {
        let mut text = "<button default foo=\"bar\" answer=42 based=true pi=3.14 range=0..1><foo /></button>";
        let parsed = single.parse_next(&mut text);
        let mut expected = Tag::new("button");
        expected.set("default")
            .with("foo", Value::String("bar".into()))
            .with("answer", Value::Int(42))
            .with("based", Value::Bool(true))
            .with("pi", Value::Float(3.14))
            .with("range", Value::Range(0..1))
            .children(vec![Tag::new("foo")]);
        println!("{text}");
        assert_eq!(expected, parsed.unwrap());
    }
}
