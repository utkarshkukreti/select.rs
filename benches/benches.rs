#![feature(test)]

extern crate test;

extern crate html5ever;

extern crate markup5ever_rcdom;

extern crate select;

pub use select::document::Document;
pub use select::predicate::*;

extern crate speculate;
use speculate::speculate;

speculate! {
    context "struct.Vec.html (228,512 bytes)" {
        before {
            let str = include_str!("../tests/fixtures/struct.Vec.html");
        }

        bench "constructing markup5ever_rcdom::RcDom" |b| {{
            use html5ever::parse_document;
            use markup5ever_rcdom::RcDom;
            use html5ever::tendril::stream::TendrilSink;

            b.iter(|| {
                let rc_dom = parse_document(RcDom::default(),
                                            Default::default()).one(str);
                rc_dom
            });
        };}

        bench "constructing Document" |b| {
            b.iter(|| Document::from(str));
        }

        context "Document::select(_).count()" {
            before {
                let document = Document::from(str);
            }

            bench "Any (11446 Nodes)" |b| {
                assert_eq!(document.select(Any).count(), 11446);
                b.iter(|| document.select(Any).count());
            }

            bench "Text (6926 Nodes)" |b| {
                assert_eq!(document.select(Text).count(), 6926);
                b.iter(|| document.select(Text).count());
            }

            bench "Element (4519 Nodes)" |b| {
                assert_eq!(document.select(Element).count(), 4519);
                b.iter(|| document.select(Element).count());
            }

            bench "Comment (1 Node)" |b| {
                assert_eq!(document.select(Comment).count(), 1);
                b.iter(|| document.select(Comment).count());
            }
        }

        context "Node::select().select().len() vs Node::select(Descendant(...)).count()" {
            before {
                let document = Document::from(str);
                let node = document.select(Name("html")).next().unwrap();
                let (parent, child) = (Name("body"), Name("span"));
                let expected = 1785;
            }

            bench "Node::select().select().len()" |b| {
                assert_eq!(node.select(parent).into_selection().select(child).len(), expected);
                b.iter(|| node.select(parent).into_selection().select(child).len());
            }

            bench "Node::select(Descendant(...)).count()" |b| {
                assert_eq!(node.select(Descendant(parent, child)).count(), expected);
                b.iter(|| node.select(Descendant(parent, child)).count());
            }
        }
    }

    context "Node::attr()" {
        before {
            let html = "<div a=b c=d e=f g=h i=j k=l m=n o=p q=r s=t u=v w=x y=z>";
            let document = Document::from(html);
            let node = document.nth(3).unwrap();
            assert_eq!(node.name(), Some("div"));
        }

        bench "hit first" |b| {
            assert!(node.attr("a").is_some());
            b.iter(|| node.attr("a"));
        }

        bench "hit last" |b| {
            assert!(node.attr("y").is_some());
            b.iter(|| node.attr("y"));
        }

        bench "miss" |b| {
            assert!(node.attr("z").is_none());
            b.iter(|| node.attr("z"));
        }
    }
}
