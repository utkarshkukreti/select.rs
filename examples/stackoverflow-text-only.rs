extern crate select;

use select::document::Document;
use select::node::{Node, Data};

///
/// Just the text of a web page without tags or scripts.
///
/// The document.nth(i) lists all the nodes in order with the root node being the first.
///
/// To be able to skip a node's contents one needs to recurse down through the root node's children.
pub fn main() {
    // stackoverflow.html was fetched from
    // http://stackoverflow.com/questions/tagged/rust?sort=votes&pageSize=50 on
    // Aug 10, 2015.
    let document = Document::from(include_str!("stackoverflow.html"));

    let root_node = document.nth(0).unwrap();

    let mut buffer = String::new();
    build_text(&mut buffer, &root_node);
    println!("{}", buffer);
}

fn build_text(buffer: &mut String, node: &Node) {
    for child in node.children() {
        match *child.data() {
            Data::Element(ref name, _) => {
                let tag_name : &str = &name.local.to_string();
                match tag_name {
                    "script" | "noscript" | "noscript-warning" => {},
                    _ => { build_text(buffer, &child); }
                }
            },
            Data::Text(ref text) => {
                buffer.push_str(&text.to_string());
                build_text(buffer, &child);
            },
            _ => { build_text(buffer, &child); }
        }
    }
}