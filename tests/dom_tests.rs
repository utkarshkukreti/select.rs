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

            let html = dom.nth(0);
            let head = dom.nth(1);
            let body = dom.nth(2);
            let a = dom.nth(3);
            let d = dom.nth(4);
            let e = dom.nth(5);
            let f = dom.nth(6);
            let g = dom.nth(7);
            let h = dom.nth(8);
            let i = dom.nth(9);
            let j = dom.nth(10);

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
