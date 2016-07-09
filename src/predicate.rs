use node::{self, Node};

/// A trait implemented by all `Node` matchers.
pub trait Predicate: Sized {
    fn matches(&self, node: &Node) -> bool;
    fn or<T: Predicate>(self, other: T) -> Or<Self, T> {
        Or(self, other)
    }
    fn and<T: Predicate>(self, other: T) -> And<Self, T> {
        And(self, other)
    }
    fn not(self) -> Not<Self> {
        Not(self)
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
        node.attr("class").map_or(false,
                                  |classes| classes.split_whitespace().any(|class| class == self.0))
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
        match *node.data() {
            node::Data::Element(..) => true,
            _ => false,
        }
    }
}

/// Matches any Text Node.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Text;

impl Predicate for Text {
    fn matches(&self, node: &Node) -> bool {
        match *node.data() {
            node::Data::Text(..) => true,
            _ => false,
        }
    }
}

/// Matches any Comment Node.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Comment;

impl Predicate for Comment {
    fn matches(&self, node: &Node) -> bool {
        match *node.data() {
            node::Data::Comment(..) => true,
            _ => false,
        }
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
