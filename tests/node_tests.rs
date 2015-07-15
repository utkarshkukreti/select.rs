#![feature(plugin)]
#![plugin(speculate)]

pub use std::collections::HashMap;

extern crate select;
pub use select::dom::Dom;
pub use select::node;

speculate! {
    describe "node" {
        before {
            let mut attrs = HashMap::new();
            attrs.insert("id".into(), "bar".into());

            let dom = Dom {
                nodes: vec![
                    node::Raw {
                        id: 0, parent: None, prev: None, next: None,
                        data: node::Data::Text("foo".into())
                    },
                    node::Raw {
                        id: 1, parent: None, prev: None, next: None,
                        data: node::Data::Element("div".into(),
                                                  attrs,
                                                  vec![])
                    },
                    node::Raw {
                        id: 2, parent: Some(1), prev: None, next: None,
                        data: node::Data::Text("baz".into())
                    }
                ]
            };

            let node0 = node::Node { dom: &dom, id: 0 };
            let node1 = node::Node { dom: &dom, id: 1 };
            let node2 = node::Node { dom: &dom, id: 2 };
        }

        test "Node::name()" {
            assert_eq!(node0.name(), None);
            assert_eq!(node1.name(), Some("div"));
        }

        test "Node::attr()" {
            assert_eq!(node0.attr("class"), None);
            assert_eq!(node1.attr("id"), Some("bar"));
            assert_eq!(node1.attr("class"), None);
        }

        test "Node::parent()" {
            assert_eq!(node0.parent(), None);
            assert_eq!(node1.parent(), None);
            assert_eq!(node2.parent(), Some(node1));
        }
    }
}
