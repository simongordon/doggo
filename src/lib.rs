extern crate xml;

// use std::fs;
// use std::fs::File;
// use std::io::{self, Write};
// use std::io::prelude::*;
use std::fmt;

// use xml::writer::{EmitterConfig, EventWriter, Result, XmlEvent};

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
        // let mut counter = 0;

        let mut body = String::new();
        let mut attr = String::new();
        for sub in self.subs.iter() {
            match sub {
                &Node::nd(ref elem) => {
                    fmt::write(&mut body, format_args!("{}", elem.to_string())).unwrap()
                }
                &Node::tx(ref elem) => {
                    fmt::write(&mut body, format_args!("{}", elem.to_string())).unwrap()
                }
                &Node::at(ref elem) => {
                    fmt::write(&mut attr, format_args!("{}=\"{}\"", elem.name, elem.value)).unwrap()
                }
            }
        }
        format!(
            "<{tag}{attr}>{body}</{tag}>",
            tag = self.name,
            attr = attr,
            body = body
        )
    }
}

// fn get_children(input: &str) -> Vec<String> {
//     let out: Vec<String> = Vec::new();
//     out
// }

// pub fn compile_text(input: &str) -> String {
//     let mut output: String = String::new();


//     let trimmed = input.trim();
//     if trimmed.len() > 0 {
//         let children = get_children(input);
//         let (tagname, body) = if let Some(first_space) = trimmed.find(" ") {
//             let pieces = trimmed.split_at(first_space);
//             (pieces.0.trim(), Some(pieces.1.trim()))
//         } else {
//             (trimmed, None)
//         };

//         output = format!("<{}>{}", tagname, output);
//         for child in children {
//             output = format!("{}{}", output, compile_text(&child));
//         }
//         output = format!("{}</{}>", output, tagname);
//     }

//     // let filename = "output.xml";
//     // let mut file = File::create(filename).unwrap();
//     // let mut config = EmitterConfig::new();
//     // config.write_document_declaration = false;

//     // let mut writer = config.perform_indent(false).create_writer(&mut file);

//     // let mut indent_level = 0;
//     // let mut last_indent = 0;
//     // let lines = input.split("\n");
//     // for line in lines {
//     //     let trimmed = line.trim();
//     //     if trimmed.len() > 0 {
//     //         let (tagname, body) = if let Some(first_space) = trimmed.find(" ") {
//     //             let pieces = trimmed.split_at(first_space);
//     //             (pieces.0, Some(pieces.1.trim()))
//     //         } else {
//     //             (trimmed, None)
//     //         };



//     //         // let inner: &str = &String::from(format!("{}", last_indent));
//     //         // let event: XmlEvent = XmlEvent::characters(&inner).into();
//     //         // writer.write(event).unwrap();


//     //         if let Some(cur_indent) = line.replace("    ", "\t").rfind("\t") {
//     //             if cur_indent <= last_indent && cur_indent != 0 {
//     //                 let event: XmlEvent = XmlEvent::end_element().into();
//     //                 writer.write(event).unwrap();
//     //                 indent_level -= 1;
//     //             } else {

//     //             }
//     //             last_indent = cur_indent;
//     //         }

//     //         let event: XmlEvent = XmlEvent::start_element(tagname).into();
//     //         writer.write(event).unwrap();
//     //         indent_level += 1;


//     //         if let Some(body) = body {
//     //             let event: XmlEvent = XmlEvent::characters(body).into();
//     //             writer.write(event).unwrap();
//     //         }
//     //     }
//     // }

//     // for _ in 0..indent_level {
//     //     let event: XmlEvent = XmlEvent::end_element().into();
//     //     writer.write(event).unwrap();
//     // }

//     // // let event: XmlEvent = XmlEvent::

//     // // let mut file = File::open("foo.txt")?;

//     // // file.read_to_string(&mut output);

//     // let mut file = File::open(filename).unwrap();
//     // let mut output = String::new();
//     // file.read_to_string(&mut output).unwrap();
//     // // let output = String::from(output);
//     // // assert_eq!(contents, "Hello, world!");

//     // fs::remove_file(filename).unwrap();

//     output
// }


#[cfg(test)]
const sample_small: &str = r#"
html
    head
    body
        p henlo
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_struct() {
        let doc = Elem {
            name: String::from("tag"),
            subs: vec![Node::tx(String::from("heres some text"))],
        };
        assert_eq!("<tag>heres some text</tag>", doc.to_string());

        let doc = Elem {
            name: String::from("html"),
            subs: vec![
                Node::nd(Elem {
                    name: String::from("head"),
                    subs: vec![
                        Node::nd(Elem {
                            name: String::from("title"),
                            subs: vec![Node::tx(String::from("Stuff"))],
                        }),
                    ],
                }),
                Node::nd(Elem {
                    name: String::from("body"),
                    subs: vec![
                        Node::nd(Elem {
                            name: String::from("p"),
                            subs: vec![Node::tx(String::from("First paragraph."))],
                        }),
                        Node::nd(Elem {
                            name: String::from("p"),
                            subs: vec![Node::tx(String::from("Second paragraph."))],
                        }),
                    ],
                }),
            ],
        };
        assert_eq!(
            "<html><head><title>Stuff</title></head><body><p>First paragraph.</p><p>Second paragraph.</p></body></html>",
            doc.to_string()
        );
    }
}
