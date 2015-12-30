#![feature(plugin, test)]
#![plugin(speculate)]

extern crate test;

extern crate html5ever;
extern crate select;

pub use select::document::Document;
pub use select::predicate::*;

speculate! {
    context "struct.Vec.html (228,512 bytes)" {
        before {
            let str = include_str!("../tests/fixtures/struct.Vec.html");
        }

        bench "constructing html5ever::rcdom::RcDom" |b| {{
            use html5ever::{parse, one_input, rcdom};
            b.iter(|| {
                let rc_dom: rcdom::RcDom = parse(one_input(str.into()),
                                                 Default::default());
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

            bench "() (11446 Nodes)" |b| {
                assert_eq!(document.find(()).iter().count(), 11446);
                b.iter(|| document.find(()));
            }

            bench "Text (6926 Nodes)" |b| {
                assert_eq!(document.find(Text).iter().count(), 6926);
                b.iter(|| document.find(Text));
            }

            bench "Element (4519 Nodes)" |b| {
                assert_eq!(document.find(Element).iter().count(), 4519);
                b.iter(|| document.find(Element));
            }

            bench "Comment (1 Node)" |b| {
                assert_eq!(document.find(Comment).iter().count(), 1);
                b.iter(|| document.find(Comment));
            }
        }

        context "Node::attr()" {
            before {
                let html = "<div a=b c=d e=f g=h i=j k=l m=n o=p q=r s=t u=v w=x y=z>";
                let document = Document::from(html);
                let node = document.nth(3);
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
}
