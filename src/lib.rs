
//use std::fmt;
//use std::collections::HashMap;


pub enum Sub {
    Node(Elem),
    Text(String),
    Comment(String),
}

pub struct Elem {
    name: String,
    //attributes: HashMap<String, String>,
    attributes: Vec<(String, String)>,
    children: Vec<Sub>,
}


impl Elem {
    pub fn to_string(&self) -> String {
        format!(
            "<{tag}{attr}>{body}</{tag}>",
            tag = self.name,
            attr = self.attributes.iter().fold(String::from(""), |joined,
             &(ref new_name,
               ref new_val)| {
                format!("{} {}=\"{}\"", joined, new_name, new_val)
            }),
            body = self.children.iter().fold(String::from(""), |joined, new| {
                format!(
                    "{}{}",
                    joined,
                    match new
                    {
                        &Sub::Node(ref elem) => elem.to_string(),
                        &Sub::Text(ref inner_text) => inner_text.to_string(),
                        &Sub::Comment(ref inner_text) => {
                            format!("<!---{}--->", inner_text.to_string())
                        }
                    }
                )
            })
        )
    }

    pub fn from_string(input: String) -> Elem {
        Elem {
            name: String::from(""),
            //attributes: HashMap::new(),
            attributes: Vec::new(),
            children: Vec::new(),
        }
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
const SAMPLE_SMALL: &str = r#"
html
    head
    body
        p henlo
"#;

#[cfg(test)]
mod tests {
    use super::*;
    //use std::collections::HashMap;

    #[test]
    fn test_node_struct() {
        let doc = Elem {
            name: String::from("tag"),
            children: vec![Sub::Text(String::from("heres some Text"))],
            //attributes: HashMap::new(),
            attributes: Vec::new(),
        };
        assert_eq!("<tag>heres some Text</tag>", doc.to_string());

        let doc = Elem {
            name: String::from("html"),
            //attributes: HashMap::new(),
            attributes: Vec::new(),
            children: vec![
                Sub::Node(Elem {
                    name: String::from("head"),
                    //attributes: HashMap::new(),
                    attributes: Vec::new(),
                    children: vec![
                        Sub::Node(Elem {
                            name: String::from("title"),
                            children: vec![Sub::Text(String::from("Stuff"))],
                            //attributes: HashMap::new(),
                            attributes: Vec::new(),
                        }),
                    ],
                }),
                Sub::Node(Elem {
                    name: String::from("body"),
                    //attributes: HashMap::new(),
                    attributes: Vec::new(),
                    children: vec![
                        Sub::Node(Elem {
                            name: String::from("p"),
                            children: vec![Sub::Text(String::from("First paragraph."))],
                            //attributes: HashMap::new(),
                            //attributes: Vec::new(),
                            attributes: vec![(String::from("class"), String::from("memes"))],
                        }),
                        Sub::Node(Elem {
                            name: String::from("p"),
                            children: vec![Sub::Text(String::from("Second paragraph."))],
                            //attributes: HashMap::new(),
                            attributes: Vec::new(),
                        }),
                    ],
                }),
            ],
        };
        assert_eq!(
            "<html><head><title>Stuff</title></head><body><p class=\"memes\">First paragraph.</p><p>Second paragraph.</p></body></html>",
            doc.to_string()
        );
    }

    #[test]
    fn test_from_string() {
        assert_eq!(
            "<html><head><title>Stuff</title></head><body><p>First paragraph.</p><p>Second paragraph.</p></body></html>",
            Elem::from_string(String::from(SAMPLE_SMALL)).to_string()
        )
    }
}
