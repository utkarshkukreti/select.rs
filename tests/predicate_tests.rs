#![feature(plugin)]
#![plugin(speculate)]

extern crate select;
pub use select::dom::Dom;
pub use select::node;
pub use select::predicate::*;

speculate! {
    describe "predicate" {
        before {
            let dom = Dom::from_str("<html><head></head><body>\
<article id='post-0'></article>\
</body></html>");
            let html = node::Node { dom: &dom, id: 0 };
            let head = node::Node { dom: &dom, id: 1 };
            let body = node::Node { dom: &dom, id: 2 };
            let article = node::Node { dom: &dom, id: 3 };
        }

        test "Name()" {
            assert_eq!(Name("html").matches(&html), true);
            assert_eq!(Name("head").matches(&html), false);
            assert_eq!(Name("body").matches(&html), false);
            assert_eq!(Name("html").matches(&head), false);
            assert_eq!(Name("head").matches(&head), true);
            assert_eq!(Name("body").matches(&head), false);
            assert_eq!(Name("html").matches(&body), false);
            assert_eq!(Name("head").matches(&body), false);
            assert_eq!(Name("body").matches(&body), true);
        }

        test "Id()" {
            assert_eq!(Id("post-0").matches(&html), false);
            assert_eq!(Id("post-0").matches(&article), true);
        }
    }
}
