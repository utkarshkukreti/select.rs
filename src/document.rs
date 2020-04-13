use html5ever::tendril::{ByteTendril, ReadExt, StrTendril};

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
    pub fn find<P: Predicate>(&self, predicate: P) -> Find<P> {
        Find {
            document: self,
            next: 0,
            predicate,
        }
    }

    /// Returns the `n`th node of the document as a `Some(Node)`, indexed from
    /// 0, or `None` if n is greater than or equal to the number of nodes.
    pub fn nth(&self, n: usize) -> Option<Node> {
        Node::new(self, n)
    }

    pub fn from_read<R: io::Read>(mut readable: R) -> io::Result<Document> {
        let mut byte_tendril = ByteTendril::new();
        readable.read_to_tendril(&mut byte_tendril)?;

        match byte_tendril.try_reinterpret() {
            Ok(str_tendril) => Ok(Document::from(str_tendril)),
            Err(_) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "stream did not contain valid UTF-8",
            )),
        }
    }
}

impl From<StrTendril> for Document {
    /// Parses the given `StrTendril` into a `Document`.
    fn from(tendril: StrTendril) -> Document {
        use html5ever::parse_document;
        use html5ever::tendril::stream::TendrilSink;
        use markup5ever_rcdom::{Handle, NodeData, RcDom};

        let mut document = Document { nodes: vec![] };

        let rc_dom = parse_document(RcDom::default(), Default::default()).one(tendril);
        recur(&mut document, &rc_dom.document, None, None);
        return document;

        fn recur(
            document: &mut Document,
            node: &Handle,
            parent: Option<usize>,
            prev: Option<usize>,
        ) -> Option<usize> {
            match node.data {
                NodeData::Document => {
                    let mut prev = None;
                    for child in node.children.borrow().iter() {
                        prev = recur(document, &child, None, prev)
                    }
                    None
                }
                NodeData::Text { ref contents } => {
                    let data = node::Data::Text(contents.borrow().clone());
                    Some(append(document, data, parent, prev))
                }
                NodeData::Comment { ref contents } => {
                    let data = node::Data::Comment(contents.clone());
                    Some(append(document, data, parent, prev))
                }
                NodeData::Element {
                    ref name,
                    ref attrs,
                    ..
                } => {
                    let name = name.clone();
                    let attrs = attrs
                        .borrow()
                        .iter()
                        .map(|attr| (attr.name.clone(), attr.value.clone()))
                        .collect();
                    let data = node::Data::Element(name, attrs);
                    let index = append(document, data, parent, prev);
                    let mut prev = None;
                    for child in node.children.borrow().iter() {
                        prev = recur(document, &child, Some(index), prev)
                    }
                    Some(index)
                }
                _ => None,
            }
        }

        fn append(
            document: &mut Document,
            data: node::Data,
            parent: Option<usize>,
            prev: Option<usize>,
        ) -> usize {
            let index = document.nodes.len();

            document.nodes.push(node::Raw {
                index,
                parent,
                prev,
                next: None,
                first_child: None,
                last_child: None,
                data,
            });

            if let Some(parent) = parent {
                let parent = &mut document.nodes[parent];
                if parent.first_child.is_none() {
                    parent.first_child = Some(index);
                }
                parent.last_child = Some(index);
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

pub struct Find<'a, P> {
    document: &'a Document,
    next: usize,
    predicate: P,
}

impl<'a, P: Predicate> std::fmt::Debug for Find<'a, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Find")
            .field("document", &self.document)
            .field("next", &self.next)
            // predicate may be closure not implementing Debug
            .finish()
    }
}

impl<'a, P: Predicate> Find<'a, P> {
    pub fn into_selection(self) -> Selection<'a> {
        Selection::new(self.document, self.map(|node| node.index()).collect())
    }
}

impl<'a, P: Predicate> Iterator for Find<'a, P> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Node<'a>> {
        while self.next < self.document.nodes.len() {
            let node = self.document.nth(self.next).unwrap();
            self.next += 1;
            if self.predicate.matches(&node) {
                return Some(node);
            }
        }
        None
    }
}
