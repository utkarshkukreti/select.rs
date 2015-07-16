#![feature(plugin)]
#![plugin(speculate)]

extern crate select;
pub use select::dom::Dom;
pub use select::selection::*;

speculate! {
    describe "selection" {
        test "Iter" {
            let dom = Dom::from_str("<html><head></head><body>\
<article id='post-0' class='post category-foo tag-bar'></article>\
</body></html>");
            let selection = Selection {
                dom: &dom,
                bitset: [0, 2, 3].iter().cloned().collect()
            };
            let mut iter = Iter {
                selection: &selection,
                next: 0
            };
            let html = iter.next().unwrap();
            let body = iter.next().unwrap();
            let article = iter.next().unwrap();
            assert_eq!(iter.next(), None);
            assert_eq!(html.name(), Some("html"));
            assert_eq!(body.name(), Some("body"));
            assert_eq!(article.attr("id"), Some("post-0"));
        }
    }
}
