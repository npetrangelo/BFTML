use iced::{widget, Element};

use super::ParseResult;

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

impl<'a, Message: Clone + 'a> From<&'a str> for ParseResult<'a, Column<'a, Message>> {
    fn from(value: &'a str) -> Self {
        todo!()
    }
}