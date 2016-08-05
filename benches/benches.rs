#![feature(plugin, test)]
#![plugin(speculate)]

extern crate test;

extern crate html5ever;
extern crate tendril;

extern crate select;

pub use select::document::Document;
pub use select::predicate::*;

speculate! {
    context "struct.Vec.html (228,512 bytes)" {
        before {
            let str = include_str!("../tests/fixtures/struct.Vec.html");
        }

        bench "constructing html5ever::rcdom::RcDom" |b| {{
            use html5ever::{parse_document, rcdom};
            use tendril::stream::TendrilSink;

            b.iter(|| {
                let rc_dom = parse_document(rcdom::RcDom::default(),
                                            Default::default()).one(str);
                rc_dom
            });
        };}

        bench "constructing Document" |b| {
            b.iter(|| Document::from(str));
        }

        context "Document::find()" {
            before {
                let document = Document::from(str);
            }

            bench "Any (11446 Nodes)" |b| {
                assert_eq!(document.find(Any).count(), 11446);
                b.iter(|| document.find(Any));
            }

            bench "Text (6926 Nodes)" |b| {
                assert_eq!(document.find(Text).count(), 6926);
                b.iter(|| document.find(Text));
            }

            bench "Element (4519 Nodes)" |b| {
                assert_eq!(document.find(Element).count(), 4519);
                b.iter(|| document.find(Element));
            }

            bench "Comment (1 Node)" |b| {
                assert_eq!(document.find(Comment).count(), 1);
                b.iter(|| document.find(Comment));
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

        context "Node::find().find().len() vs Node::find(Descendant(...)).count()" {
            before {
                let document = Document::from(str);
                let node = document.find(Name("html")).next().unwrap();
                let (parent, child) = (Name("body"), Name("span"));
                let expected = 1785;
            }

            bench "Node::find().find().len()" |b| {
                assert_eq!(node.find(parent).into_selection().find(child).len(), expected);
                b.iter(|| node.find(parent).into_selection().find(child).len());
            }

            bench "Node::find(Descendant(...)).count()" |b| {
                assert_eq!(node.find(Descendant(parent, child)).count(), expected);
                b.iter(|| node.find(Descendant(parent, child)).count());
            }
        }
    }
}
