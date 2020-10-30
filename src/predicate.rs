use crate::node::{self, Node};
use regex::Regex;

/// A trait implemented by all `Node` matchers.
pub trait Predicate {
    fn matches(&self, node: &Node) -> bool;
    fn or<T: Predicate>(self, other: T) -> Or<Self, T>
    where
        Self: Sized,
    {
        Or(self, other)
    }
    fn and<T: Predicate>(self, other: T) -> And<Self, T>
    where
        Self: Sized,
    {
        And(self, other)
    }
    fn not(self) -> Not<Self>
    where
        Self: Sized,
    {
        Not(self)
    }
    fn child<T: Predicate>(self, other: T) -> Child<Self, T>
    where
        Self: Sized,
    {
        Child(self, other)
    }
    fn descendant<T: Predicate>(self, other: T) -> Descendant<Self, T>
    where
        Self: Sized,
    {
        Descendant(self, other)
    }
}

/// Matches any Node.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Any;

impl Predicate for Any {
    fn matches(&self, _: &Node) -> bool {
        true
    }
}

/// Matches Element Node with name `T`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Name<T>(pub T);

impl<'a> Predicate for Name<&'a str> {
    fn matches(&self, node: &Node) -> bool {
        node.name() == Some(self.0)
    }
}

/// Matches Element Node containing class `T`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Class<T>(pub T);

impl<'a> Predicate for Class<&'a str> {
    fn matches(&self, node: &Node) -> bool {
        node.attr("class").map_or(false, |classes| {
            classes.split_whitespace().any(|class| class == self.0)
        })
    }
}

/// Matches if the Predicate `T` does not match.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Not<T>(pub T);

impl<T: Predicate> Predicate for Not<T> {
    fn matches(&self, node: &Node) -> bool {
        !self.0.matches(node)
    }
}

/// Matches Element Node containing attribute `N` with value `V` if `V` is an
/// `&str`, or any value if `V` is `()`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Attr<N, V>(pub N, pub V);

impl<'a> Predicate for Attr<&'a str, &'a str> {
    fn matches(&self, node: &Node) -> bool {
        node.attr(self.0) == Some(self.1)
    }
}

impl<'a> Predicate for Attr<&'a str, ()> {
    fn matches(&self, node: &Node) -> bool {
        node.attr(self.0).is_some()
    }
}

impl<'a> Predicate for Attr<&'a str, Regex> {
    fn matches(&self, node: &Node) -> bool {
        if let Some(attr) = node.attr(self.0) {
            return self.1.is_match(attr)
        }
        false
    }
}


/// Matches if the function returns true.
impl<F: Fn(&Node) -> bool> Predicate for F {
    fn matches(&self, node: &Node) -> bool {
        self(node)
    }
}

/// Matches any Element Node.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Element;

impl Predicate for Element {
    fn matches(&self, node: &Node) -> bool {
        matches!(*node.data(), node::Data::Element(..))
    }
}

/// Matches any Text Node.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Text;

impl Predicate for Text {
    fn matches(&self, node: &Node) -> bool {
        matches!(*node.data(), node::Data::Text(..))
    }
}

/// Matches any Comment Node.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Comment;

impl Predicate for Comment {
    fn matches(&self, node: &Node) -> bool {
        matches!(*node.data(), node::Data::Comment(..))
    }
}

/// Matches if either inner Predicate `A` or `B` matches the Node.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Or<A, B>(pub A, pub B);

impl<A: Predicate, B: Predicate> Predicate for Or<A, B> {
    fn matches(&self, node: &Node) -> bool {
        self.0.matches(node) || self.1.matches(node)
    }
}

/// Matches if the inner Predicate `A` and `B` both match the Node.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct And<A, B>(pub A, pub B);

impl<A: Predicate, B: Predicate> Predicate for And<A, B> {
    fn matches(&self, node: &Node) -> bool {
        self.0.matches(node) && self.1.matches(node)
    }
}

/// Matches if inner Predicate `B` matches the node and `A` matches the parent
/// of the node.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Child<A, B>(pub A, pub B);

impl<A: Predicate, B: Predicate> Predicate for Child<A, B> {
    fn matches(&self, node: &Node) -> bool {
        if let Some(parent) = node.parent() {
            self.1.matches(node) && self.0.matches(&parent)
        } else {
            false
        }
    }
}

/// Matches if inner Predicate `B` matches the node and `A` matches any of the
/// parents of node.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Descendant<A, B>(pub A, pub B);

impl<A: Predicate, B: Predicate> Predicate for Descendant<A, B> {
    fn matches(&self, node: &Node) -> bool {
        if self.1.matches(node) {
            let mut node = *node;
            while let Some(parent) = node.parent() {
                if self.0.matches(&parent) {
                    return true;
                }
                node = parent;
            }
        }
        false
    }
}
