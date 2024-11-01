use iced::{widget, Element};

use super::ParseResult;

pub struct Button<'a, Message: Clone + 'a> {
    content: Element<'a, Message>,
}

impl<'a, Message: Clone + 'a> From<Button<'a, Message>> for Element<'a, Message> {
    fn from(button: Button<'a, Message>) -> Self {
        widget::Button::new(button.content).into()
    }
}

impl<'a, Message: Clone + 'a> From<&'a str> for ParseResult<'a, Button<'a, Message>> {
    fn from(value: &'a str) -> Self {
        todo!()
    }
}