use std::{fs::File, io::BufReader};

use iced::{widget::{button, column, text, Column}, Element};
use indexmap::IndexMap;
use xml::{reader::XmlEvent, EventReader};
use DOM::{BftmlElement, Tag, TagType};

mod DOM;

struct Counter {
    value: i64,
    tree: BftmlElement
}

impl Default for Counter {
    fn default() -> Self {
        let plus = TagType::Button(Box::new(BftmlElement::Text("+".into())));
        let plus = Tag { id: "+".into(), tag_type: plus, attributes: IndexMap::new() };
        let minus = TagType::Button(Box::new(BftmlElement::Text("-".into())));
        let minus = Tag { id: "-".into(), tag_type: minus, attributes: IndexMap::new() };
        let children = vec![BftmlElement::Tag(plus), BftmlElement::Tag(minus)];
        Self {
            value: Default::default(),
            tree: BftmlElement::Tag(Tag { id: "column".into(), tag_type: TagType::Column(children), attributes: IndexMap::new()})
        }
    }
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
            Message::Set(value) => self.value = value
        }
    }

    fn view(&self) -> Element<Message> {
        self.tree.clone().into()
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
    Set(i64)
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
