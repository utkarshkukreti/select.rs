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
            let mut iter = selection.iter();
            let html = iter.next().unwrap();
            let body = iter.next().unwrap();
            let article = iter.next().unwrap();
            assert_eq!(iter.next(), None);
            assert_eq!(html.name(), Some("html"));
            assert_eq!(body.name(), Some("body"));
            assert_eq!(article.attr("id"), Some("post-0"));
        }

        test "Selection::filter()" {
            use select::predicate::*;

            let dom = Dom::from_str(include_str!("fixtures/struct.Vec.html"));
            let all = dom.find(());

            assert_eq!(all.filter(()).iter().count(), 11445);

            let divs = all.filter(Name("div"));
            assert_eq!(divs.iter().count(), 208);
            for div in divs.iter() {
                assert_eq!(div.name(), Some("div"))
            }

            assert_eq!(all.filter(Id("main")).iter().count(), 1);

            let structs = all.filter(Class("struct"));
            assert_eq!(structs.iter().count(), 168);
            for struct_ in structs.iter() {
                assert!(struct_.attr("class").unwrap().contains("struct"))
            };
        }
    }
}
