#![feature(plugin)]
#![plugin(speculate)]

pub use std::collections::HashMap;

extern crate select;
pub use select::dom::Dom;
pub use select::node;

speculate! {
    describe "node" {
        before {
            let dom = Dom::from_str("<html><head></head><body id=something>\
                                     foo<bar>baz<quux class=another-thing>");

            let html = dom.nth(0);
            let head = dom.nth(1);
            let body = dom.nth(2);
            let foo = dom.nth(3);
            let bar = dom.nth(4);
            let baz = dom.nth(5);
            let quux = dom.nth(6);
        }

        test "Node::name()" {
            assert_eq!(html.name(), Some("html"));
            assert_eq!(head.name(), Some("head"));
            assert_eq!(body.name(), Some("body"));
            assert_eq!(foo.name(), None);
            assert_eq!(bar.name(), Some("bar"));
            assert_eq!(baz.name(), None);
            assert_eq!(quux.name(), Some("quux"));
        }

        test "Node::attr()" {
            assert_eq!(html.attr("id"), None);
            assert_eq!(head.attr("id"), None);
            assert_eq!(body.attr("id"), Some("something"));
            assert_eq!(body.attr("class"), None);
            assert_eq!(foo.attr("id"), None);
            assert_eq!(bar.attr("id"), None);
            assert_eq!(baz.attr("id"), None);
            assert_eq!(quux.attr("id"), None);
            assert_eq!(quux.attr("class"), Some("another-thing"));
        }

        test "Node::parent()" {
            assert_eq!(html.parent(), None);
            assert_eq!(head.parent(), Some(html));
            assert_eq!(body.parent(), Some(html));
            assert_eq!(foo.parent(), Some(body));
            assert_eq!(bar.parent(), Some(body));
            assert_eq!(baz.parent(), Some(bar));
            assert_eq!(quux.parent(), Some(bar));
        }

        test "Node::prev() / Node::next()" {
            assert_eq!(html.prev(), None);
            assert_eq!(html.next(), None);
            assert_eq!(head.prev(), None);
            assert_eq!(head.next(), Some(body));
            assert_eq!(body.prev(), Some(head));
            assert_eq!(body.next(), None);
            assert_eq!(foo.prev(), None);
            assert_eq!(foo.next(), Some(bar));
            assert_eq!(bar.prev(), Some(foo));
            assert_eq!(bar.next(), None);
            assert_eq!(baz.prev(), None);
            assert_eq!(baz.next(), Some(quux));
            assert_eq!(quux.prev(), Some(baz));
            assert_eq!(quux.next(), None);
        }

        test "Node::text()" {
            assert_eq!(html.text(), "foobaz");
            assert_eq!(head.text(), "");
            assert_eq!(body.text(), "foobaz");
            assert_eq!(foo.text(), "foo");
            assert_eq!(bar.text(), "baz");
            assert_eq!(baz.text(), "baz");
            assert_eq!(quux.text(), "");
        }
    }
}
