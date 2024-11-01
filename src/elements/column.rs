use iced::{widget, Element};

use super::ParseError;

pub struct Column<'a, Message: Clone + 'a> {
    children: Vec<Element<'a, Message>>
}

impl<'a, Message: Clone + 'a> From<Column<'a, Message>> for Element<'a, Message> {
    fn from(column: Column<'a, Message>) -> Self {
        widget::Column::from_iter(column.children.into_iter().map(
            |el| el.into())
        ).into()
    }
}

impl<'a, Message: Clone + 'a> TryFrom<&'a str> for Column<'a, Message> {
    type Error = ParseError<'a>;
    
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        todo!()
    }
}