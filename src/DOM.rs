use iced::{widget, Element};
use indexmap::IndexMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TagType {
    Button(Box<BftmlElement>),
    Column(Vec<BftmlElement>),
    Row(Vec<BftmlElement>)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tag {
    pub id: String,
    pub tag_type: TagType,
    pub attributes: IndexMap<String, Option<String>>,
}

impl<'a, Message: Clone + 'a> From<Tag> for Element<'a, Message> {
    fn from(tag: Tag) -> Self {
        match tag.tag_type {
            TagType::Button(child) => widget::Button::new(*child).into(),
            TagType::Column(children) => widget::Column::from_iter(children.into_iter().map(|el| el.into())).into(),
            TagType::Row(children) => widget::Row::from_iter(children.into_iter().map(|el| el.into())).into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BftmlElement {
    Tag(Tag),
    Text(String)
}

impl<'a, Message: Clone + 'a> From<BftmlElement> for Element<'a, Message> {
    fn from(element: BftmlElement) -> Self {
        match element {
            BftmlElement::Text(content ) => widget::text(content).into(),
            BftmlElement::Tag(tag) => tag.into(),
        }
    }
}