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
            let selection = Selection::new(&dom,
                                           [0, 2, 3].iter().cloned().collect());
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

            assert_eq!(all.filter(Attr("id", "main")).iter().count(), 1);

            let structs = all.filter(Class("struct"));
            assert_eq!(structs.iter().count(), 168);
            for struct_ in structs.iter() {
                assert!(struct_.attr("class").unwrap().contains("struct"))
            };
        }

        test "Selection::find()" {
            use select::predicate::*;

            let dom = Dom::from_str(include_str!("fixtures/struct.Vec.html"));
            let all = dom.find(());

            let struct_divs = all.find(Class("struct")).find(Name("div"));
            assert_eq!(struct_divs.iter().count(), 204);
            for struct_div in struct_divs.iter() {
                assert_eq!(struct_div.name(), Some("div"));
            };

            let struct_as = all.find(Class("struct")).find(Name("a"));
            assert_eq!(struct_as.iter().count(), 1260);
            for struct_a in struct_as.iter() {
                assert_eq!(struct_a.name(), Some("a"));
            };
        }

        test "Selection::parent()" {
            use select::predicate::*;

            let dom = Dom::from_str(include_str!("fixtures/struct.Vec.html"));

            assert_eq!(dom.find(Name("div")).parent().iter().count(), 8);
            assert_eq!(dom.find(Name("span")).parent().iter().count(), 205);
        }
    }
}
