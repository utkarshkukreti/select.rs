#![allow(
    unused_variables,
    clippy::blacklisted_name,
    clippy::many_single_char_names
)]

pub use select::document::Document;
pub use select::node;
pub use select::predicate::*;

use speculate::speculate;

speculate! {
    describe "predicate" {
        before {
            let document = Document::from("<html><head></head><body>\
<article id='post-0' class='post category-foo tag-bar'>foo</article>\
<!--A Comment-->\
<div class='a'><div class='b'><div class='c'><div class='d'></div></div></div></div>
</body></html>");
            let html = document.nth(0).unwrap();
            let head = document.nth(1).unwrap();
            let body = document.nth(2).unwrap();
            let article = document.nth(3).unwrap();
            let foo = document.nth(4).unwrap();
            let comment = document.nth(5).unwrap();
            let a = document.nth(6).unwrap();
            let b = document.nth(7).unwrap();
            let c = document.nth(8).unwrap();
            let d = document.nth(9).unwrap();
        }

        test "Any" {
            assert_eq!(super::Any.matches(&html), true);
            assert_eq!(super::Any.matches(&head), true);
            assert_eq!(super::Any.matches(&body), true);
            assert_eq!(super::Any.matches(&article), true);
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

        test "Class()" {
            assert_eq!(Class("post").matches(&html), false);
            assert_eq!(Class("post").matches(&article), true);
            assert_eq!(Class("category-foo").matches(&article), true);
            assert_eq!(Class("tag-bar").matches(&article), true);
            assert_eq!(Class("foo").matches(&article), false);
            assert_eq!(Class("bar").matches(&article), false);
        }

        test "Not()" {
            assert_eq!(Not(Name("html")).matches(&html), false);
            assert_eq!(Not(Name("html")).matches(&head), true);
            assert_eq!(Not(Name("head")).matches(&html), true);
            assert_eq!(Not(Name("head")).matches(&head), false);
        }

        test "Attr()" {
            assert_eq!(Attr("id", "post-0").matches(&html), false);
            assert_eq!(Attr("id", "post-0").matches(&article), true);
            assert_eq!(Attr("id", ()).matches(&html), false);
            assert_eq!(Attr("id", ()).matches(&article), true);
            assert_eq!(Attr("id", regex::Regex::new("post").unwrap()).matches(&article), true);
            assert_eq!(Attr("class", regex::Regex::new("^category$").unwrap()).matches(&article), false);
            assert_eq!(Attr("class", regex::Regex::new("category").unwrap()).matches(&article), true);
        }

        test "Fn(&Node) -> bool" {
            let f = |node: &node::Node| node.name() == Some("html");
            assert_eq!(f.matches(&html), true);
            assert_eq!(f.matches(&head), false);
            assert_eq!(f.matches(&body), false);
        }

        test "Element" {
            assert_eq!(super::Element.matches(&html), true);
            assert_eq!(super::Element.matches(&head), true);
            assert_eq!(super::Element.matches(&body), true);
            assert_eq!(super::Element.matches(&article), true);
            assert_eq!(super::Element.matches(&foo), false);
        }

        test "Text" {
            assert_eq!(super::Text.matches(&html), false);
            assert_eq!(super::Text.matches(&head), false);
            assert_eq!(super::Text.matches(&body), false);
            assert_eq!(super::Text.matches(&article), false);
            assert_eq!(super::Text.matches(&foo), true);
            assert_eq!(super::Text.matches(&comment), false);
        }

        test "Comment" {
            assert_eq!(super::Comment.matches(&html), false);
            assert_eq!(super::Comment.matches(&head), false);
            assert_eq!(super::Comment.matches(&body), false);
            assert_eq!(super::Comment.matches(&article), false);
            assert_eq!(super::Comment.matches(&foo), false);
            assert_eq!(super::Comment.matches(&comment), true);
        }

        test "Or()" {
            let html_or_head = Or(Name("html"), Name("head"));
            assert_eq!(html_or_head.matches(&html), true);
            assert_eq!(html_or_head.matches(&head), true);
            assert_eq!(html_or_head.matches(&body), false);
            assert_eq!(html_or_head.matches(&article), false);
            assert_eq!(html_or_head.matches(&foo), false);
        }

        test "And()" {
            let article_and_post_0 = And(Name("article"), Attr("id", "post-0"));
            assert_eq!(article_and_post_0.matches(&html), false);
            assert_eq!(article_and_post_0.matches(&head), false);
            assert_eq!(article_and_post_0.matches(&body), false);
            assert_eq!(article_and_post_0.matches(&article), true);
            assert_eq!(article_and_post_0.matches(&foo), false);
        }

        test "Child()" {
            let html_article = Child(Name("html"), Name("article"));
            assert_eq!(html_article.matches(&html), false);
            assert_eq!(html_article.matches(&article), false);

            let body_article = Child(Name("body"), Name("article"));
            assert_eq!(body_article.matches(&html), false);
            assert_eq!(body_article.matches(&article), true);
        }

        test "Descendant()" {
            let check = |parent: &str, child: &str, matching: Option<usize>| {
                let selector = Descendant(Class(parent), Class(child));
                for node in &[a, b, c, d] {
                    let expected = matching.map_or(false, |index| node.index() == index);
                    assert_eq!(selector.matches(node), expected);
                }
            };

            check("a", "a", None);
            check("a", "b", Some(b.index()));
            check("a", "c", Some(c.index()));
            check("a", "d", Some(d.index()));
            check("b", "a", None);
            check("b", "b", None);
            check("b", "c", Some(c.index()));
            check("b", "d", Some(d.index()));
            check("c", "a", None);
            check("c", "b", None);
            check("c", "c", None);
            check("c", "d", Some(d.index()));
            check("d", "a", None);
            check("d", "b", None);
            check("d", "c", None);
            check("d", "d", None);
        }

        // https://github.com/utkarshkukreti/select.rs/issues/35
        test "Box<Predicate>" {
            let post_0: Box<dyn Predicate> = Box::new(Attr("id", "post-0"));
            assert_eq!(post_0.matches(&html), false);
            assert_eq!(post_0.matches(&head), false);
            assert_eq!(post_0.matches(&article), true);
            let not_html: Box<dyn Predicate> = Box::new(Not(Name("html")));
            assert_eq!(not_html.matches(&html), false);
            assert_eq!(not_html.matches(&head), true);
            assert_eq!(not_html.matches(&article), true);
        }
    }
}
