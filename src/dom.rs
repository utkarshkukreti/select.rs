use node::{self, Node};
use predicate::Predicate;
use selection::Selection;

#[derive(Clone, Debug, PartialEq)]
pub struct Dom {
    pub nodes: Vec<node::Raw>
}

impl Dom {
    pub fn from_str(str: &str) -> Dom {
        use html5ever::{parse, one_input};
        use html5ever_dom_sink::common;
        use html5ever_dom_sink::owned_dom::{self, OwnedDom};

        let mut dom = Dom {
            nodes: vec![]
        };

        let owned_dom: OwnedDom = parse(one_input(str.into()),
                                        Default::default());
        recur(&mut dom, &owned_dom.document, None, None);
        return dom;

        fn recur(dom: &mut Dom,
                 node: &owned_dom::Node,
                 parent: Option<node::Ref>,
                 prev: Option<node::Ref>) -> Option<node::Ref> {
            match node.node {
                common::Document => {
                    let mut prev = None;
                    for child in &node.children {
                        prev = recur(dom, &child, None, prev)
                    }
                    None
                },
                common::Doctype(..) => None,
                common::Text(ref text) => {
                    let data = node::Data::Text(text.into());
                    Some(append(dom, data, parent, prev))
                },
                common::Comment(..) => None,
                common::Element(ref name, ref attrs) => {
                    let name = name.local.as_slice().into();
                    let attrs = attrs.iter().map(|attr| {
                        (attr.name.local.as_slice().into(),
                         attr.value.clone().into())
                    }).collect();
                    let data = node::Data::Element(name, attrs, vec![]);
                    let id = append(dom, data, parent, prev);
                    let mut prev = None;
                    for child in &node.children {
                        prev = recur(dom, &child, Some(id), prev)
                    }
                    Some(id)
                }
            }
        }

        fn append(dom: &mut Dom,
                  data: node::Data,
                  parent: Option<node::Ref>,
                  prev: Option<node::Ref>) -> node::Ref {
            let id = dom.nodes.len();

            dom.nodes.push(node::Raw {
                id: id,
                parent: parent,
                prev: prev,
                next: None,
                data: data
            });

            if let Some(parent) = parent {
                match &mut dom.nodes[parent].data {
                    &mut node::Data::Element(_, _, ref mut children) => {
                        children.push(id);
                    },
                    _ => unreachable!()
                }
            }

            if let Some(prev) = prev {
                dom.nodes[prev].next = Some(id);
            }

            id
        }
    }

    pub fn find<'a, P: Predicate>(&'a self, p: P) -> Selection<'a> {
        Selection {
            dom: self,
            bitset: (0..self.nodes.len()).filter(|&id| {
                let node = Node { dom: self, id: id };
                p.matches(&node)
            }).collect()
        }
    }

    pub fn nth(&self, n: usize) -> Node {
        assert!(n < self.nodes.len());
        Node {
            dom: self,
            id: n
        }
    }
}
