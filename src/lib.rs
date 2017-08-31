extern crate xml;

use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::io::prelude::*;

use xml::writer::{EmitterConfig, EventWriter, Result, XmlEvent};

pub fn compile_text(input: &str) -> String {
    let mut output: &str = "fff";
    let mut file = File::create("output.xml").unwrap();
    let mut config = EmitterConfig::new();
    config.write_document_declaration = false;

    let mut writer = config.perform_indent(false).create_writer(&mut file);

    let event: XmlEvent = XmlEvent::start_element("html").into();
    writer.write(event).unwrap();
    let event: XmlEvent = XmlEvent::end_element().into();
    writer.write(event).unwrap();

    // let mut file = File::open("foo.txt")?;

    // file.read_to_string(&mut output);

    let mut file = File::open("output.xml").unwrap();
    let mut output = String::new();
    file.read_to_string(&mut output).unwrap();
    // let output = String::from(output);
    // assert_eq!(contents, "Hello, world!");

    fs::remove_file("output.xml").unwrap();

    output
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_text_simple() {
        assert_eq!("<html />", compile_text("html"));
        assert_eq!("<memes />", compile_text("memes"));
    }
}
