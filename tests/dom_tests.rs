#![feature(plugin)]
#![plugin(speculate)]

pub use std::collections::HashMap;

extern crate select;
pub use select::dom::Dom;
pub use select::node;

speculate! {
    describe "dom" {
        test "Dom::from_str()" {
            let dom = Dom::from_str("<a b=c>d<e><f></e>g<h><i></i><j>");

            // html, head, and body are automatically added by the parser.
            assert_eq!(dom.nodes.len(), 11);

            let html = node::Node { dom: &dom, id: 0 };
            let head = node::Node { dom: &dom, id: 1 };
            let body = node::Node { dom: &dom, id: 2 };
            let a = node::Node { dom: &dom, id: 3 };
            let d = node::Node { dom: &dom, id: 4 };
            let e = node::Node { dom: &dom, id: 5 };
            let f = node::Node { dom: &dom, id: 6 };
            let g = node::Node { dom: &dom, id: 7 };
            let h = node::Node { dom: &dom, id: 8 };
            let i = node::Node { dom: &dom, id: 9 };
            let j = node::Node { dom: &dom, id: 10 };

            assert_eq!(html.name(), Some("html"));

            assert_eq!(head.name(), Some("head"));

            assert_eq!(body.name(), Some("body"));

            assert_eq!(a.name(), Some("a"));
            assert_eq!(a.attr("b"), Some("c"));

            assert_eq!(d.name(), None);
            assert_eq!(d.next(), Some(e));
            assert_eq!(d.parent(), Some(a));

            assert_eq!(e.name(), Some("e"));
            assert_eq!(e.prev(), Some(d));

            assert_eq!(f.name(), Some("f"));

            assert_eq!(g.name(), None);

            assert_eq!(h.name(), Some("h"));

            assert_eq!(i.name(), Some("i"));
            assert_eq!(i.parent(), Some(h));

            assert_eq!(j.name(), Some("j"));
            assert_eq!(j.parent(), Some(h));
        }

        test "Dom::find()" {
            use select::predicate::*;

            let dom = Dom::from_str(include_str!("fixtures/struct.Vec.html"));
            assert_eq!(dom.find(()).iter().count(), 11445);
            assert_eq!(dom.find(Name("div")).iter().count(), 208);
            assert_eq!(dom.find(Attr("id", "main")).iter().count(), 1);
            assert_eq!(dom.find(Class("struct")).iter().count(), 168);
        }
    }
}
