#![feature(plugin)]
#![plugin(speculate)]

pub use std::collections::HashMap;

extern crate select;
pub use select::document::Document;
pub use select::node;

speculate! {
    describe "document" {
        test "Document::from(&str)" {
            let document = Document::from("<a b=c>d<e><f></e>g<h><i></i><j><!--k-->");

            // html, head, and body are automatically added by the parser.
            assert_eq!(document.nodes.len(), 12);

            let html = document.nth(0).unwrap();
            let head = document.nth(1).unwrap();
            let body = document.nth(2).unwrap();
            let a = document.nth(3).unwrap();
            let d = document.nth(4).unwrap();
            let e = document.nth(5).unwrap();
            let f = document.nth(6).unwrap();
            let g = document.nth(7).unwrap();
            let h = document.nth(8).unwrap();
            let i = document.nth(9).unwrap();
            let j = document.nth(10).unwrap();
            let k = document.nth(11).unwrap();

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

        test "Docucment::from_read()" {
            use select::predicate::*;
            use std::io::Cursor;

            let html = "<html><body><p>Hello</p></body></html>";
            let cursor = Cursor::new(html);
            let document = Document::from_read(cursor);

            assert!(document.is_ok());
            assert_eq!(document.unwrap().find(Name("p")).count(), 1);
        }

        test "Document::find()" {
            use select::predicate::*;

            let document = Document::from(include_str!("fixtures/struct.Vec.html"));
            assert_eq!(document.find(Any).count(), 11446);
            assert_eq!(document.find(Name("div")).count(), 208);
            assert_eq!(document.find(Attr("id", "main")).count(), 1);
            assert_eq!(document.find(Class("struct")).count(), 168);
        }
    }
}
