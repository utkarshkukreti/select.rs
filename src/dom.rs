use node::{self, Node};
use predicate::Predicate;
use selection::Selection;

#[derive(Clone, Debug, PartialEq)]
pub struct Dom {
    pub nodes: Vec<node::Raw>
}

impl Dom {
    pub fn from_str(str: &str) -> Dom {
        use html5ever::{parse, one_input, rcdom};

        let mut dom = Dom {
            nodes: vec![]
        };

        let rc_dom: rcdom::RcDom = parse(one_input(str.into()),
                                         Default::default());
        recur(&mut dom, &rc_dom.document, None, None);
        return dom;

        fn recur(dom: &mut Dom,
                 node: &rcdom::Handle,
                 parent: Option<usize>,
                 prev: Option<usize>) -> Option<usize> {
            match node.borrow().node {
                rcdom::Document => {
                    let mut prev = None;
                    for child in &node.borrow().children {
                        prev = recur(dom, &child, None, prev)
                    }
                    None
                },
                rcdom::Doctype(..) => None,
                rcdom::Text(ref text) => {
                    let data = node::Data::Text(text.into());
                    Some(append(dom, data, parent, prev))
                },
                rcdom::Comment(ref comment) => {
                    let data = node::Data::Comment(comment.into());
                    Some(append(dom, data, parent, prev))
                },
                rcdom::Element(ref name, ref _element, ref attrs) => {
                    let name = name.local.as_slice().into();
                    let attrs = attrs.iter().map(|attr| {
                        (attr.name.local.as_slice().into(),
                         attr.value.clone().into())
                    }).collect();
                    let data = node::Data::Element(name, attrs, vec![]);
                    let index = append(dom, data, parent, prev);
                    let mut prev = None;
                    for child in &node.borrow().children {
                        prev = recur(dom, &child, Some(index), prev)
                    }
                    Some(index)
                }
            }
        }

        fn append(dom: &mut Dom,
                  data: node::Data,
                  parent: Option<usize>,
                  prev: Option<usize>) -> usize {
            let index = dom.nodes.len();

            dom.nodes.push(node::Raw {
                index: index,
                parent: parent,
                prev: prev,
                next: None,
                data: data
            });

            if let Some(parent) = parent {
                match &mut dom.nodes[parent].data {
                    &mut node::Data::Element(_, _, ref mut children) => {
                        children.push(index);
                    },
                    _ => unreachable!()
                }
            }

            if let Some(prev) = prev {
                dom.nodes[prev].next = Some(index);
            }

            index
        }
    }

    pub fn find<'a, P: Predicate>(&'a self, p: P) -> Selection<'a> {
        Selection::new(self, (0..self.nodes.len()).filter(|&index| {
            p.matches(&self.nth(index))
        }).collect())
    }

    pub fn nth(&self, n: usize) -> Node {
        assert!(n < self.nodes.len());
        Node::new(self, n)
    }
}
