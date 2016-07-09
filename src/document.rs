use tendril::{StrTendril, ByteTendril, ReadExt};

use node::{self, Node};
use predicate::Predicate;
use selection::Selection;

use std::io;

/// An HTML document.
#[derive(Clone, Debug, PartialEq)]
pub struct Document {
    pub nodes: Vec<node::Raw>,
}

impl Document {
    /// Returns a `Selection` containing nodes passing the given predicate `p`.
    pub fn find<'a, P: Predicate>(&'a self, p: P) -> Selection<'a> {
        Selection::new(self,
                       (0..self.nodes.len())
                           .filter(|&index| p.matches(&self.nth(index).unwrap()))
                           .collect())
    }

    /// Returns the `n`th node of the document as a `Some(Node)`, indexed from
    /// 0, or `None` if n is greater than or equal to the number of nodes.
    pub fn nth(&self, n: usize) -> Option<Node> {
        Node::new(self, n)
    }

    pub fn from_read<R: io::Read>(mut readable: R) -> io::Result<Document> {
        let mut byte_tendril = ByteTendril::new();
        try!(readable.read_to_tendril(&mut byte_tendril));

        match byte_tendril.try_reinterpret() {
            Ok(str_tendril) => Ok(Document::from(str_tendril)),
            Err(_) => {
                Err(io::Error::new(io::ErrorKind::InvalidData,
                                   "stream did not contain valid UTF-8"))
            }
        }
    }
}

impl From<StrTendril> for Document {
    /// Parses the given `StrTendril` into a `Document`.
    fn from(tendril: StrTendril) -> Document {
        use html5ever::{parse_document, rcdom};
        use tendril::stream::TendrilSink;

        let mut document = Document { nodes: vec![] };

        let rc_dom = parse_document(rcdom::RcDom::default(), Default::default()).one(tendril);
        recur(&mut document, &rc_dom.document, None, None);
        return document;

        fn recur(document: &mut Document,
                 node: &rcdom::Handle,
                 parent: Option<usize>,
                 prev: Option<usize>)
                 -> Option<usize> {
            match node.borrow().node {
                rcdom::Document => {
                    let mut prev = None;
                    for child in &node.borrow().children {
                        prev = recur(document, &child, None, prev)
                    }
                    None
                }
                rcdom::Doctype(..) => None,
                rcdom::Text(ref text) => {
                    let data = node::Data::Text(text.clone());
                    Some(append(document, data, parent, prev))
                }
                rcdom::Comment(ref comment) => {
                    let data = node::Data::Comment(comment.clone());
                    Some(append(document, data, parent, prev))
                }
                rcdom::Element(ref name, ref _element, ref attrs) => {
                    let name = name.local.clone();
                    let attrs = attrs.iter()
                        .map(|attr| (attr.name.local.clone(), attr.value.clone()))
                        .collect();
                    let data = node::Data::Element(name, attrs, vec![]);
                    let index = append(document, data, parent, prev);
                    let mut prev = None;
                    for child in &node.borrow().children {
                        prev = recur(document, &child, Some(index), prev)
                    }
                    Some(index)
                }
            }
        }

        fn append(document: &mut Document,
                  data: node::Data,
                  parent: Option<usize>,
                  prev: Option<usize>)
                  -> usize {
            let index = document.nodes.len();

            document.nodes.push(node::Raw {
                index: index,
                parent: parent,
                prev: prev,
                next: None,
                data: data,
            });

            if let Some(parent) = parent {
                match document.nodes[parent].data {
                    node::Data::Element(_, _, ref mut children) => {
                        children.push(index);
                    }
                    _ => unreachable!(),
                }
            }

            if let Some(prev) = prev {
                document.nodes[prev].next = Some(index);
            }

            index
        }
    }
}

impl<'a> From<&'a str> for Document {
    /// Parses the given `&str` into a `Document`.
    fn from(str: &str) -> Document {
        Document::from(StrTendril::from(str))
    }
}
