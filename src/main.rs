use std::{fs::File, io::BufReader};

use iced::widget::{button, column, text, Column};
use xml::{reader::XmlEvent, EventReader};

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

fn xml_file() -> std::io::Result<()> {
    let file = File::open("file.xml")?;
    let file = BufReader::new(file); // Buffering is important for performance

    let parser = EventReader::new(file);
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                println!("{:spaces$}+{name}", "", spaces = depth * 2);
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{:spaces$}-{name}", "", spaces = depth * 2);
            }
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
            // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
            _ => {}
        }
    }
    Ok(())
}

fn main() -> iced::Result {
    xml_file().unwrap();
    iced::run("A cool counter", Counter::update, Counter::view)
}
