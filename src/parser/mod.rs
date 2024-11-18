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
        let mut text = "<button default foo=\"bar\" answer=42 based=true pi=3.14 range=0..1><foo></foo></button>";
        let parsed = single.parse_next(&mut text);
        let mut expected = Tag::new("button");
        expected.set("default");
        expected.with("foo", Value::String("bar".into()));
        expected.with("answer", Value::Int(42));
        expected.with("based", Value::Bool(true));
        expected.with("pi", Value::Float(3.14));
        expected.with("range", Value::Range(0..1));
        expected.child(Tag::new("foo"));
        println!("{text}");
        assert_eq!(expected, parsed.unwrap());
    }
}
