use iced::{widget, Element};

use super::ParseError;

pub struct Button<'a, Message: Clone + 'a> {
    content: Element<'a, Message>,
}

impl<'a, Message: Clone + 'a> From<Button<'a, Message>> for Element<'a, Message> {
    fn from(button: Button<'a, Message>) -> Self {
        widget::Button::new(button.content).into()
    }
}

impl<'a, Message: Clone + 'a> TryFrom<&'a str> for Button<'a, Message> {
    type Error = ParseError<'a>;
    
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        todo!()
    }
}