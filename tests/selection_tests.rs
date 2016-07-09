#![feature(plugin)]
#![plugin(speculate)]

extern crate select;
pub use select::document::Document;
pub use select::selection::*;

speculate! {
    describe "selection" {
        test "Iter" {
            let document = Document::from("<html><head></head><body>\
<article id='post-0' class='post category-foo tag-bar'></article>\
</body></html>");
            let selection = Selection::new(&document,
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

            let document = Document::from(include_str!("fixtures/struct.Vec.html"));
            let all = document.find(Any);

            assert_eq!(all.filter(Any).len(), 11446);

            let divs = all.filter(Name("div"));
            assert_eq!(divs.len(), 208);
            for div in &divs {
                assert_eq!(div.name(), Some("div"))
            }

            assert_eq!(all.filter(Attr("id", "main")).len(), 1);

            let structs = all.filter(Class("struct"));
            assert_eq!(structs.len(), 168);
            for struct_ in &structs {
                assert!(struct_.attr("class").unwrap().contains("struct"))
            };
        }

        test "Selection::find()" {
            use select::predicate::*;

            let document = Document::from(include_str!("fixtures/struct.Vec.html"));
            let all = document.find(Any);

            let struct_divs = all.find(Class("struct")).find(Name("div"));
            assert_eq!(struct_divs.len(), 204);
            for struct_div in &struct_divs {
                assert_eq!(struct_div.name(), Some("div"));
            };

            let struct_as = all.find(Class("struct")).find(Name("a"));
            assert_eq!(struct_as.len(), 1260);
            for struct_a in &struct_as {
                assert_eq!(struct_a.name(), Some("a"));
            };
        }

        test "Selection::parent()" {
            use select::predicate::*;

            let document = Document::from(include_str!("fixtures/struct.Vec.html"));

            assert_eq!(document.find(Name("div")).parent().len(), 8);
            assert_eq!(document.find(Name("span")).parent().len(), 205);
        }

        test "Selection::prev() / Selection::next()" {
            use select::predicate::*;

            let document = Document::from(include_str!("fixtures/struct.Vec.html"));

            assert_eq!(document.find(Name("div")).prev().len(), 208);
            assert_eq!(document.find(Name("div")).next().len(), 203);
            assert_eq!(document.find(Name("span")).prev().len(), 1729);
            assert_eq!(document.find(Name("span")).next().len(), 1690);
        }

        test "Selection::parents()" {
            use select::predicate::*;

            let document = Document::from(include_str!("fixtures/struct.Vec.html"));

            assert_eq!(document.find(Name("div")).parents().len(), 10);
            assert_eq!(document.find(Name("span")).parents().len(), 308);
        }

        test "Selection::children()" {
            use select::predicate::*;

            let document = Document::from(include_str!("fixtures/struct.Vec.html"));

            let div_children = document.find(Name("div")).children();
            assert_eq!(div_children.len(), 1210);
            for div_child in &div_children {
                assert_eq!(div_child.parent().unwrap().name(), Some("div"));
            }

            let span_children = document.find(Name("span")).children();
            assert_eq!(span_children.len(), 1986);
            for span_child in &span_children {
                assert_eq!(span_child.parent().unwrap().name(), Some("span"));
            };
        }

        test "Selection::first()" {
            use select::predicate::*;

            let document = Document::from(include_str!("fixtures/struct.Vec.html"));

            assert!(document.find(Name("div")).first().is_some());
            assert!(document.find(Name("divv")).first().is_none());
        }
    }
}
