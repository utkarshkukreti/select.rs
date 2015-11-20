use node::{self, Node};
use predicate::Predicate;
use selection::Selection;

/// An HTML document.
#[derive(Clone, Debug, PartialEq)]
pub struct Document {
    pub nodes: Vec<node::Raw>
}

impl Document {
    /// Parses the given `&str` into a `Document`.
    pub fn from_str(str: &str) -> Document {
        use html5ever::{parse, one_input, rcdom};

        let mut document = Document {
            nodes: vec![]
        };

        let rc_dom: rcdom::RcDom = parse(one_input(str.into()),
                                         Default::default());
        recur(&mut document, &rc_dom.document, None, None);
        return document;

        fn recur(document: &mut Document,
                 node: &rcdom::Handle,
                 parent: Option<usize>,
                 prev: Option<usize>) -> Option<usize> {
            match node.borrow().node {
                rcdom::Document => {
                    let mut prev = None;
                    for child in &node.borrow().children {
                        prev = recur(document, &child, None, prev)
                    }
                    None
                },
                rcdom::Doctype(..) => None,
                rcdom::Text(ref text) => {
                    let data = node::Data::Text(text.clone());
                    Some(append(document, data, parent, prev))
                },
                rcdom::Comment(ref comment) => {
                    let data = node::Data::Comment(comment.clone());
                    Some(append(document, data, parent, prev))
                },
                rcdom::Element(ref name, ref _element, ref attrs) => {
                    let name = name.local.clone();
                    let attrs = attrs.iter().map(|attr| {
                        (attr.name.local.clone(), attr.value.clone())
                    }).collect();
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
                  prev: Option<usize>) -> usize {
            let index = document.nodes.len();

            document.nodes.push(node::Raw {
                index: index,
                parent: parent,
                prev: prev,
                next: None,
                data: data
            });

            if let Some(parent) = parent {
                match document.nodes[parent].data {
                    node::Data::Element(_, _, ref mut children) => {
                        children.push(index);
                    },
                    _ => unreachable!()
                }
            }

            if let Some(prev) = prev {
                document.nodes[prev].next = Some(index);
            }

            index
        }
    }

    /// Returns a `Selection` containing nodes passing the given predicate `p`.
    pub fn find<'a, P: Predicate>(&'a self, p: P) -> Selection<'a> {
        Selection::new(self, (0..self.nodes.len()).filter(|&index| {
            p.matches(&self.nth(index))
        }).collect())
    }

    /// Returns the `n`th node of the document as a `Node`, indexed from 0.
    ///
    /// # Panics
    ///
    /// Panics if `n` is not less than the number of nodes in `self`.
    pub fn nth(&self, n: usize) -> Node {
        assert!(n < self.nodes.len());
        Node::new(self, n)
    }
}
