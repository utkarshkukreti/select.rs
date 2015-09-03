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
                 parent: Option<node::Ref>,
                 prev: Option<node::Ref>) -> Option<node::Ref> {
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
                    let ref_ = append(dom, data, parent, prev);
                    let mut prev = None;
                    for child in &node.borrow().children {
                        prev = recur(dom, &child, Some(ref_), prev)
                    }
                    Some(ref_)
                }
            }
        }

        fn append(dom: &mut Dom,
                  data: node::Data,
                  parent: Option<node::Ref>,
                  prev: Option<node::Ref>) -> node::Ref {
            let ref_ = dom.nodes.len();

            dom.nodes.push(node::Raw {
                ref_: ref_,
                parent: parent,
                prev: prev,
                next: None,
                data: data
            });

            if let Some(parent) = parent {
                match &mut dom.nodes[parent].data {
                    &mut node::Data::Element(_, _, ref mut children) => {
                        children.push(ref_);
                    },
                    _ => unreachable!()
                }
            }

            if let Some(prev) = prev {
                dom.nodes[prev].next = Some(ref_);
            }

            ref_
        }
    }

    pub fn find<'a, P: Predicate>(&'a self, p: P) -> Selection<'a> {
        Selection::new(self, (0..self.nodes.len()).filter(|&ref_| {
            p.matches(&self.nth(ref_))
        }).collect())
    }

    pub fn nth(&self, n: usize) -> Node {
        assert!(n < self.nodes.len());
        Node::new(self, n)
    }
}
