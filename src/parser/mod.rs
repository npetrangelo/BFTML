mod attributes;
mod tags;
mod values;

#[cfg(test)]
mod test {
    use crate::parser::{tags::Tag, values::Value};

    #[test]
    fn test_altogether() {
        let parsed = "<button foo=\"bar\"><foo></foo></button>".parse::<Tag>();
        let mut expected = Tag::new("button");
        expected.with("foo", Value::String("bar".into()));
        expected.child(Tag::new("foo"));
        assert_eq!(expected, parsed.unwrap());
    }
}