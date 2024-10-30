use std::{fs::File, io::BufReader};

use iced::widget::{button, column, text, Column};
use xml::{reader::XmlEvent, EventReader};

#[derive(Default)]
struct Counter {
    value: i64,
}

impl Counter {
    fn update(&mut self, message: Set) {
        self.value = message.value
    }

    fn view(&self) -> Column<Set> {
        column![
            button("+").on_press(Set { value: 10 }).style(button::success),
            text(self.value),
            button("-").on_press(Set { value: -10 }),
        ]
    }
}

#[derive(Debug, Clone, Copy)]
struct Set {
    value: i64
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
