#![feature(plugin)]
#![plugin(speculate)]

pub use std::collections::HashMap;

extern crate select;
pub use select::document::Document;
pub use select::node;

speculate! {
    describe "document" {
        test "Document::from_str()" {
            let document = Document::from_str("<a b=c>d<e><f></e>g<h><i></i><j><!--k-->");

            // html, head, and body are automatically added by the parser.
            assert_eq!(document.nodes.len(), 12);

            let html = document.nth(0);
            let head = document.nth(1);
            let body = document.nth(2);
            let a = document.nth(3);
            let d = document.nth(4);
            let e = document.nth(5);
            let f = document.nth(6);
            let g = document.nth(7);
            let h = document.nth(8);
            let i = document.nth(9);
            let j = document.nth(10);
            let k = document.nth(11);

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

            assert_eq!(k.name(), None);
            assert_eq!(k.parent(), Some(j));
        }

        test "Document::find()" {
            use select::predicate::*;

            let document = Document::from_str(include_str!("fixtures/struct.Vec.html"));
            assert_eq!(document.find(()).iter().count(), 11446);
            assert_eq!(document.find(Name("div")).iter().count(), 208);
            assert_eq!(document.find(Attr("id", "main")).iter().count(), 1);
            assert_eq!(document.find(Class("struct")).iter().count(), 168);
        }
    }
}
