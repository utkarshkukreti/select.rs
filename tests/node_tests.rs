#![allow(unused_variables)]

pub use std::collections::HashMap;

extern crate select;
pub use select::document::Document;
pub use select::node;

extern crate speculate;
use speculate::speculate;

speculate! {
    describe "node" {
        before {
            let document = Document::from("<html><head></head><body id=something>\
                                           foo<bar>baz<quux class=another-thing>\
                                           <!--comment-->");

            let html = document.nth(0).unwrap();
            let head = document.nth(1).unwrap();
            let body = document.nth(2).unwrap();
            let foo = document.nth(3).unwrap();
            let bar = document.nth(4).unwrap();
            let baz = document.nth(5).unwrap();
            let quux = document.nth(6).unwrap();
            let comment = document.nth(7).unwrap();
        }

        test "Node::name()" {
            assert_eq!(html.name(), Some("html"));
            assert_eq!(head.name(), Some("head"));
            assert_eq!(body.name(), Some("body"));
            assert_eq!(foo.name(), None);
            assert_eq!(bar.name(), Some("bar"));
            assert_eq!(baz.name(), None);
            assert_eq!(quux.name(), Some("quux"));

            // Lifetime
            let name = {
                html.clone().name()
            };
            assert_eq!(name, Some("html"));
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

            // Lifetime
            let attr = {
                html.clone().attr("id")
            };
            assert_eq!(attr, None);
        }

        test "Node::raw()" {
            // Lifetime
            let raw = {
                &html.raw().data
            };
        }

        test "Node::data()" {
            // Lifetime
            let data = {
                html.data()
            };
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

        test "Node::first_child()" {
            for i in 0..document.nodes.len() {
                let node = document.nth(i).unwrap();
                assert_eq!(node.first_child(), node.children().next());
            }
        }

        test "Node::last_child()" {
            for i in 0..document.nodes.len() {
                let node = document.nth(i).unwrap();
                assert_eq!(node.last_child(), node.children().last());
            }
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

        test "Node::find()" {
            {
                use select::predicate::*;
                let document = Document::from(include_str!("fixtures/struct.Vec.html"));
                let main = document.find(Attr("id", "main")).next().unwrap();

                let (div, span) = (Name("div"), Name("span"));

                assert_eq!(main.find(div).count(), 204);
                assert_eq!(main.find(span).count(), 1785);
                assert_eq!(main.find(div.child(div).descendant(span.child(span))).count(), 3);
                assert_eq!(main.find(div.child(div).descendant(span).child(span)).count(), 3);
            };
        }

        test "Node::is()" {
            {
                use select::predicate::*;
                let document = Document::from(include_str!("fixtures/struct.Vec.html"));
                for div in document.find(Name("div")) {
                    assert!(div.is(Name("div")));
                }
            };
        }

        test "Node::as_text()" {
            assert_eq!(foo.as_text(), Some("foo"));
            assert_eq!(bar.as_text(), None);
            assert_eq!(baz.as_text(), Some("baz"));

            // Lifetime
            let text = {
                foo.as_text()
            };
            assert_eq!(text, Some("foo"));
        }

        test "Node::as_comment()" {
            assert_eq!(foo.as_comment(), None);
            assert_eq!(comment.as_comment(), Some("comment"));

            // Lifetime
            let comment = {
                comment.as_comment()
            };
            assert_eq!(comment, Some("comment"));
        }

        test "Node::html()" {
            assert_eq!(html.html(), "<html><head></head><body id=\"something\">\
                                     foo<bar>baz<quux class=\"another-thing\">\
                                     <!--comment--></quux></bar></body></html>");
            assert_eq!(head.html(), "<head></head>");
            assert_eq!(foo.html(), "foo");
            assert_eq!(quux.html(), "<quux class=\"another-thing\"><!--comment--></quux>");
            assert_eq!(comment.html(), "<!--comment-->");

            let document = Document::from("<div a=b c=d e=f g=h i=j>");
            let div = document.nth(3).unwrap();
            assert_eq!(div.name(), Some("div"));
            assert_eq!(div.html(), r#"<div a="b" c="d" e="f" g="h" i="j"></div>"#);
        }

        test "Node::inner_html()" {
            assert_eq!(html.inner_html(), "<head></head><body id=\"something\">\
                                           foo<bar>baz<quux class=\"another-thing\">\
                                           <!--comment--></quux></bar></body>");
            assert_eq!(head.inner_html(), "");
            assert_eq!(foo.inner_html(), "");
            assert_eq!(quux.inner_html(), "<!--comment-->");
            assert_eq!(comment.inner_html(), "");
        }

        test "Node::children()" {
            let mut children = html.children();
            assert_eq!(children.next().unwrap().name(), Some("head"));
            assert_eq!(children.next().unwrap().name(), Some("body"));
            assert_eq!(children.next(), None);

            assert_eq!(body.children().count(), 2);

            assert_eq!(baz.children().count(), 0);

            assert_eq!(quux.children().count(), 1);
        }

        test "Node::descendants()" {
            use select::predicate::*;
            let document = Document::from(include_str!("fixtures/struct.Vec.html"));
            for i in 0..document.nodes.len() {
                let node = document.nth(i).unwrap();
                let actual = node.descendants().map(|node| node.index()).collect::<Vec<_>>();
                let expected = node.find(Any).map(|node| node.index()).collect::<Vec<_>>();
                assert_eq!(actual, expected);
            }
        }

        test "Node::attrs()" {
            let mut attrs = quux.attrs();
            assert_eq!(attrs.next(), Some(("class", "another-thing")));
            assert_eq!(attrs.next(), None);
        }

        test "std::fmt::Debug for Node" {
            assert_eq!(format!("{:?}", bar).replace(" ", ""), r#"Element {
                name: "bar",
                attrs: [],
                children: [
                    Text("baz"),
                    Element {
                        name: "quux",
                        attrs: [("class", "another-thing")],
                        children: [Comment("comment")]
                    }
                ]}"#.replace("\n", "").replace(" ", ""));

            assert_eq!(format!("{:?}", baz), "Text(\"baz\")");

            assert_eq!(format!("{:?}", quux).replace(" ", ""), r#"Element {
                name: "quux",
                attrs: [("class", "another-thing")],
                children: [Comment("comment")]
            }"#.replace("\n", "").replace(" ", ""));

            assert_eq!(format!("{:?}", comment), "Comment(\"comment\")");
        }

        test "Children::into_selection()" {
            let document = Document::from(include_str!("fixtures/struct.Vec.html"));
            for i in 0..document.nodes.len() {
                let node = document.nth(i).unwrap();
                let actual = node.children().into_selection().iter().map(|node| {
                    node.index()
                }).collect::<Vec<_>>();
                let expected = node.children().map(|node| node.index()).collect::<Vec<_>>();
                assert_eq!(actual, expected);
            }
        }

        // https://github.com/utkarshkukreti/select.rs/pull/38
        test "issue #38" {
            {
                use select::predicate::*;
                let _bar = {
                    let body = html.find(Name("body")).next().unwrap();
                    body.find(Name("bar")).next().unwrap()
                };
            }
        }
    }
}
