pub use select::document::Document;
pub use select::selection::*;

use speculate::speculate;

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
            let all = document.select(Any).into_selection();

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

        test "Selection::select()" {
            use select::predicate::*;

            let document = Document::from(include_str!("fixtures/struct.Vec.html"));
            let all = document.select(Any).into_selection();

            let struct_divs = all.select(Class("struct")).select(Name("div"));
            assert_eq!(struct_divs.len(), 204);
            for struct_div in &struct_divs {
                assert_eq!(struct_div.name(), Some("div"));
            };

            let struct_as = all.select(Class("struct")).select(Name("a"));
            assert_eq!(struct_as.len(), 1260);
            for struct_a in &struct_as {
                assert_eq!(struct_a.name(), Some("a"));
            };
        }

        test "Selection::parent()" {
            use select::predicate::*;

            let document = Document::from(include_str!("fixtures/struct.Vec.html"));

            assert_eq!(document.select(Name("div")).into_selection().parent().len(), 8);
            assert_eq!(document.select(Name("span")).into_selection().parent().len(), 205);
        }

        test "Selection::prev() / Selection::next()" {
            use select::predicate::*;

            let document = Document::from(include_str!("fixtures/struct.Vec.html"));

            assert_eq!(document.select(Name("div")).into_selection().prev().len(), 208);
            assert_eq!(document.select(Name("div")).into_selection().next().len(), 203);
            assert_eq!(document.select(Name("span")).into_selection().prev().len(), 1729);
            assert_eq!(document.select(Name("span")).into_selection().next().len(), 1690);
        }

        test "Selection::parents()" {
            use select::predicate::*;

            let document = Document::from(include_str!("fixtures/struct.Vec.html"));

            assert_eq!(document.select(Name("div")).into_selection().parents().len(), 10);
            assert_eq!(document.select(Name("span")).into_selection().parents().len(), 308);
        }

        test "Selection::children()" {
            use select::predicate::*;

            let document = Document::from(include_str!("fixtures/struct.Vec.html"));

            let div_children = document.select(Name("div")).into_selection().children();
            assert_eq!(div_children.len(), 1210);
            for div_child in &div_children {
                assert_eq!(div_child.parent().unwrap().name(), Some("div"));
            }

            let span_children = document.select(Name("span")).into_selection().children();
            assert_eq!(span_children.len(), 1986);
            for span_child in &span_children {
                assert_eq!(span_child.parent().unwrap().name(), Some("span"));
            };
        }

        test "Selection::first()" {
            use select::predicate::*;

            let document = Document::from(include_str!("fixtures/struct.Vec.html"));

            assert!(document.select(Name("div")).into_selection().first().is_some());
            assert!(document.select(Name("divv")).into_selection().first().is_none());
        }

        test "Selection::last()" {
            use select::predicate::*;

            let document = Document::from(include_str!("fixtures/struct.Vec.html"));

            assert!(document.select(Name("div")).into_selection().last().is_some());
            assert!(document.select(Name("divv")).into_selection().last().is_none());
        }

        test "Selection::len() == Selection::iter().count()" {
            use select::predicate::*;

            let document = Document::from(include_str!("fixtures/struct.Vec.html"));

            fn check<P: Predicate>(document: &Document, predicate: P) {
                let selection = document.select(predicate).into_selection();
                assert_eq!(selection.len(), selection.iter().count());
            }

            check(&document, Any);
            check(&document, Attr("id", "main"));
            check(&document, Class("struct"));
            check(&document, Name("div"));
            check(&document, Name("span"));
        }

        test "Iter (lifetimes)" {
            let document = Document::from("<html><head></head><body>\
<article id='post-0' class='post category-foo tag-bar'></article>\
</body></html>");
            let html = {
                let selection = Selection::new(&document,
                                               [0, 2, 3].iter().cloned().collect());
                selection.iter().next().unwrap()
            };
            assert_eq!(html.name(), Some("html"));
        }
    }
}
