#![allow(
    unused_variables,
    clippy::disallowed_names,
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
            assert!(super::Any.matches(&html));
            assert!(super::Any.matches(&head));
            assert!(super::Any.matches(&body));
            assert!(super::Any.matches(&article));
        }

        test "Name()" {
            assert!(Name("html").matches(&html));
            assert!(!Name("head").matches(&html));
            assert!(!Name("body").matches(&html));
            assert!(!Name("html").matches(&head));
            assert!(Name("head").matches(&head));
            assert!(!Name("body").matches(&head));
            assert!(!Name("html").matches(&body));
            assert!(!Name("head").matches(&body));
            assert!(Name("body").matches(&body));
        }

        test "Class()" {
            assert!(!Class("post").matches(&html));
            assert!(Class("post").matches(&article));
            assert!(Class("category-foo").matches(&article));
            assert!(Class("tag-bar").matches(&article));
            assert!(!Class("foo").matches(&article));
            assert!(!Class("bar").matches(&article));
        }

        test "Not()" {
            assert!(!Not(Name("html")).matches(&html));
            assert!(Not(Name("html")).matches(&head));
            assert!(Not(Name("head")).matches(&html));
            assert!(!Not(Name("head")).matches(&head));
        }

        test "Attr()" {
            assert!(!Attr("id", "post-0").matches(&html));
            assert!(Attr("id", "post-0").matches(&article));
            assert!(!Attr("id", ()).matches(&html));
            assert!(Attr("id", ()).matches(&article));
        }

        test "Fn(&Node) -> bool" {
            let f = |node: &node::Node| node.name() == Some("html");
            assert!(f.matches(&html));
            assert!(!f.matches(&head));
            assert!(!f.matches(&body));
        }

        test "Element" {
            assert!(super::Element.matches(&html));
            assert!(super::Element.matches(&head));
            assert!(super::Element.matches(&body));
            assert!(super::Element.matches(&article));
            assert!(!super::Element.matches(&foo));
        }

        test "Text" {
            assert!(!super::Text.matches(&html));
            assert!(!super::Text.matches(&head));
            assert!(!super::Text.matches(&body));
            assert!(!super::Text.matches(&article));
            assert!(super::Text.matches(&foo));
            assert!(!super::Text.matches(&comment));
        }

        test "Comment" {
            assert!(!super::Comment.matches(&html));
            assert!(!super::Comment.matches(&head));
            assert!(!super::Comment.matches(&body));
            assert!(!super::Comment.matches(&article));
            assert!(!super::Comment.matches(&foo));
            assert!(super::Comment.matches(&comment));
        }

        test "Or()" {
            let html_or_head = Or(Name("html"), Name("head"));
            assert!(html_or_head.matches(&html));
            assert!(html_or_head.matches(&head));
            assert!(!html_or_head.matches(&body));
            assert!(!html_or_head.matches(&article));
            assert!(!html_or_head.matches(&foo));
        }

        test "And()" {
            let article_and_post_0 = And(Name("article"), Attr("id", "post-0"));
            assert!(!article_and_post_0.matches(&html));
            assert!(!article_and_post_0.matches(&head));
            assert!(!article_and_post_0.matches(&body));
            assert!(article_and_post_0.matches(&article));
            assert!(!article_and_post_0.matches(&foo));
        }

        test "Child()" {
            let html_article = Child(Name("html"), Name("article"));
            assert!(!html_article.matches(&html));
            assert!(!html_article.matches(&article));

            let body_article = Child(Name("body"), Name("article"));
            assert!(!body_article.matches(&html));
            assert!(body_article.matches(&article));
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
            assert!(!post_0.matches(&html));
            assert!(!post_0.matches(&head));
            assert!(post_0.matches(&article));
            let not_html: Box<dyn Predicate> = Box::new(Not(Name("html")));
            assert!(!not_html.matches(&html));
            assert!(not_html.matches(&head));
            assert!(not_html.matches(&article));
        }
    }
}
