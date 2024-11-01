use std::{fs::File, io::BufReader};

use iced::{Element, Task};
use indexmap::IndexMap;
use xml::{reader::XmlEvent, EventReader};
use DOM::{BftmlElement, Tag, TagType};

mod DOM;
mod elements;

struct Dom {
    tree: BftmlElement
}

impl Default for Dom {
    fn default() -> Self {
        let plus = TagType::Button(Box::new(BftmlElement::Text("+".into())));
        let plus = Tag { id: "+".into(), tag_type: plus, attributes: IndexMap::new() };
        let minus = TagType::Button(Box::new(BftmlElement::Text("-".into())));
        let minus = Tag { id: "-".into(), tag_type: minus, attributes: IndexMap::new() };
        let children = vec![BftmlElement::Tag(plus), BftmlElement::Tag(minus)];
        Self {
            tree: BftmlElement::Tag(Tag { id: "column".into(), tag_type: TagType::Column(children), attributes: IndexMap::new()})
        }
    }
}

impl Dom {
    fn update(&mut self, message: Message) {

    }

    fn view(&self) -> Element<Message> {
        self.tree.clone().into()
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {

}

#[derive(Debug)]
enum ParseError {

}

fn xml_file(file: File) -> Result<Dom, ParseError> {
    let file = BufReader::new(file); // Buffering is important for performance

    let parser = EventReader::new(file);
    let mut depth = 0;
    let mut dom = Dom { tree: BftmlElement::Text("Placeholder".into()) };
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
    Ok(dom)
}

fn init_dom() -> (Dom, Task<Message>) {
    let mut dom = Dom { tree: BftmlElement::Text("Placeholder".into()) };
    (dom, Task::none())
}

fn main() -> iced::Result {
    let file = File::open("file.xml").expect("File exists");
    xml_file(file).expect("Parse correctly");
    iced::application("A cool counter", Dom::update, Dom::view).run_with(init_dom)
}
