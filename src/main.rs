use std::{fs::File, io::BufReader};

use iced::widget::{button, column, text, Column};

#[derive(Default)]
struct Counter {
    value: i64,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Column<Message> {
        column![
            button("+").on_press(Message::Increment).style(button::success),
            text(self.value),
            button("-").on_press(Message::Decrement),
        ]
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

fn main() -> iced::Result {
    iced::run("A cool counter", Counter::update, Counter::view)
}
