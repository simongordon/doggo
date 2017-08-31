extern crate xml;

use std::fs;
use std::fs::File;
// use std::io::{self, Write};
use std::io::prelude::*;
use std::fmt;

use xml::writer::{EmitterConfig, EventWriter, Result, XmlEvent};

// fn get_inner(input: &str) -> &str {
//     "test"
// }

enum Node {
    nd(Elem),
    at(Attr),
    tx(String),
}

struct Attr {
    name: String,
    value: String,
}

pub struct Elem {
    name: String,
    subs: Vec<Node>,
}

impl Elem {
    pub fn to_string(&self) -> String {
        let mut output = String::new();
        fmt::write(&mut output, format_args!("<{}>", self.name)).unwrap();
        // let mut counter = 0;
        for sub in self.subs.iter() {
            // fmt::write(&mut output, format_args!("{}", counter)).unwrap();
            // counter += 1;
            // let sub: Node = sub.unwrap;
            fmt::write(
                &mut output,
                format_args!(
                    "{}",
                    match sub {
                        &Node::nd(ref elem) => elem.to_string(),
                        // &Node::at(ref elem) => elem.to_string(),
                        &Node::tx(ref txt) => txt.to_string(),
                        _ => String::from("a"),
                    }
                ),
            ).unwrap();
        }
        fmt::write(&mut output, format_args!("</{}>", self.name)).unwrap();

        output
    }
}

fn get_children(input: &str) -> Vec<String> {
    let out: Vec<String> = Vec::new();
    out
}

pub fn compile_text(input: &str) -> String {
    let mut output: String = String::new();


    let trimmed = input.trim();
    if trimmed.len() > 0 {
        let children = get_children(input);
        let (tagname, body) = if let Some(first_space) = trimmed.find(" ") {
            let pieces = trimmed.split_at(first_space);
            (pieces.0.trim(), Some(pieces.1.trim()))
        } else {
            (trimmed, None)
        };

        output = format!("<{}>{}", tagname, output);
        for child in children {
            output = format!("{}{}", output, compile_text(&child));
        }
        output = format!("{}</{}>", output, tagname);
    }

    // let filename = "output.xml";
    // let mut file = File::create(filename).unwrap();
    // let mut config = EmitterConfig::new();
    // config.write_document_declaration = false;

    // let mut writer = config.perform_indent(false).create_writer(&mut file);

    // let mut indent_level = 0;
    // let mut last_indent = 0;
    // let lines = input.split("\n");
    // for line in lines {
    //     let trimmed = line.trim();
    //     if trimmed.len() > 0 {
    //         let (tagname, body) = if let Some(first_space) = trimmed.find(" ") {
    //             let pieces = trimmed.split_at(first_space);
    //             (pieces.0, Some(pieces.1.trim()))
    //         } else {
    //             (trimmed, None)
    //         };



    //         // let inner: &str = &String::from(format!("{}", last_indent));
    //         // let event: XmlEvent = XmlEvent::characters(&inner).into();
    //         // writer.write(event).unwrap();


    //         if let Some(cur_indent) = line.replace("    ", "\t").rfind("\t") {
    //             if cur_indent <= last_indent && cur_indent != 0 {
    //                 let event: XmlEvent = XmlEvent::end_element().into();
    //                 writer.write(event).unwrap();
    //                 indent_level -= 1;
    //             } else {

    //             }
    //             last_indent = cur_indent;
    //         }

    //         let event: XmlEvent = XmlEvent::start_element(tagname).into();
    //         writer.write(event).unwrap();
    //         indent_level += 1;


    //         if let Some(body) = body {
    //             let event: XmlEvent = XmlEvent::characters(body).into();
    //             writer.write(event).unwrap();
    //         }
    //     }
    // }

    // for _ in 0..indent_level {
    //     let event: XmlEvent = XmlEvent::end_element().into();
    //     writer.write(event).unwrap();
    // }

    // // let event: XmlEvent = XmlEvent::

    // // let mut file = File::open("foo.txt")?;

    // // file.read_to_string(&mut output);

    // let mut file = File::open(filename).unwrap();
    // let mut output = String::new();
    // file.read_to_string(&mut output).unwrap();
    // // let output = String::from(output);
    // // assert_eq!(contents, "Hello, world!");

    // fs::remove_file(filename).unwrap();

    output
}


#[cfg(test)]
const memeage: &str = r#"
html
    head
    body
        p henlo
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_text_simple() {
        // assert_eq!("<html />", compile_text("html"));
        // assert_eq!("<memes />", compile_text("memes"));
        assert_eq!(
            "<html><head /><body><p>henlo</p></body></html>",
            compile_text(memeage)
        );
    }

    #[test]
    fn test_node_struct() {
        let doc = Elem {
            name: String::from("html"),
            subs: vec![Node::tx(String::from("here's some text"))],
        };
        assert_eq!("<html>here's some text</html>", doc.to_string());
    }
}
