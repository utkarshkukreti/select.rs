use node::{self, Node};

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
///
/// This includes text nodes, comments, and elements. There is no equivalent CSS selector.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Any;

impl Predicate for Any {
    fn matches(&self, _: &Node) -> bool {
        true
    }
}

/// Matches no Node.
///
/// This is roughly equivalent to `:not(*)`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Nothing;

impl Predicate for Nothing {
    fn matches(&self, _: &Node) -> bool {
        false
    }
}

/// Matches Element Node with tag name `T`.
///
/// This is equivalent to the `tag` CSS selector.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Name<T>(pub T);

impl<'a> Predicate for Name<&'a str> {
    fn matches(&self, node: &Node) -> bool {
        node.name() == Some(self.0)
    }
}

/// Matches Element Node containing class `T`.
///
/// This is equivalent to the `.class` CSS selector.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Class<T>(pub T);

impl<'a> Predicate for Class<&'a str> {
    fn matches(&self, node: &Node) -> bool {
        node.classes().any(|class| class == self.0)
    }
}

/// Matches Element Node with ID `T`.
///
/// This is equivalent to the `#id` CSS selector.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Id<T>(pub T);

impl<'a> Predicate for Id<&'a str> {
    fn matches(&self, node: &Node) -> bool {
        node.id() == Some(self.0)
    }
}

/// Matches if the Predicate `T` does not match.
///
/// This is equivalent to the `:not(selector)` CSS selector.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Not<T>(pub T);

impl<T: Predicate> Predicate for Not<T> {
    fn matches(&self, node: &Node) -> bool {
        !self.0.matches(node)
    }
}

/// Matches Element Node containing attribute `N` with value `V`.
///
/// This is equivalent to the `[attr=val]` CSS selector.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Attr<N, V>(pub N, pub V);

impl<'a> Predicate for Attr<&'a str, &'a str> {
    fn matches(&self, node: &Node) -> bool {
        node.attr(self.0) == Some(self.1)
    }
}

/// Matches Element Node containing attribute `N` which matches function `F`.
///
/// There is no equivalent CSS selector.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AttrMatches<N, V>(pub N, pub V);

impl<'a, F: Fn(&str) -> bool> Predicate for AttrMatches<&'a str, F> {
    fn matches(&self, node: &Node) -> bool {
        node.attr(self.0).map_or(false, &self.1)
    }
}

/// Matches Element Node containing attribute `N` which contains pattern `V`.
///
/// This is equivalent to the `[attr*=val]` CSS selector.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AttrContains<N, V>(pub N, pub V);

impl<'a> Predicate for AttrContains<&'a str, &'a str> {
    fn matches(&self, node: &Node) -> bool {
        node.attr(self.0).map_or(false, |a| a.contains(self.1))
    }
}

/// Matches Element Node containing attribute `N` which starts with pattern `V`.
///
/// This is equivalent to the `[attr^=val]` CSS selector.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AttrStartsWith<N, V>(pub N, pub V);

impl<'a> Predicate for AttrStartsWith<&'a str, &'a str> {
    fn matches(&self, node: &Node) -> bool {
        node.attr(self.0).map_or(false, |a| a.starts_with(self.1))
    }
}

/// Matches Element Node containing attribute `N` which ends with pattern `V`.
///
/// This is equivalent to the `[attr$=val]` CSS selector.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AttrEndsWith<N, V>(pub N, pub V);

impl<'a> Predicate for AttrEndsWith<&'a str, &'a str> {
    fn matches(&self, node: &Node) -> bool {
        node.attr(self.0).map_or(false, |a| a.ends_with(self.1))
    }
}

/// Matches Element Node containing attribute `N` contains the word `V` surrounded by either the
/// edges of the string, or whitespace.
///
/// This is equivalent to the `[attr~=val]` CSS selector.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AttrContainsWord<N, V>(pub N, pub V);

impl<'a> Predicate for AttrContainsWord<&'a str, &'a str> {
    fn matches(&self, node: &Node) -> bool {
        node.attr(self.0).map_or(false, |a| a.split_whitespace().any(|w| w == self.1))
    }
}

/// Matches Element Node containing attribute `N` being either exactly the value `V` or `V`
/// followed by `"-"`, then some string.
///
/// This is equivalent to the `[attr|=val]` CSS selector.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AttrLang<N, V>(pub N, pub V);

impl<'a> Predicate for AttrLang<&'a str, &'a str> {
    fn matches(&self, node: &Node) -> bool {
        node.attr(self.0).and_then(|a| a.split('-').next()).map_or(false, |a| a == self.1)
    }
}

/// Matches Element Node containing attribute `N`.
///
/// This is equivalent to the `[attr]` CSS selector.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HasAttr<N>(pub N);

impl<'a> Predicate for HasAttr<&'a str> {
    fn matches(&self, node: &Node) -> bool {
        node.attr(self.0).is_some()
    }
}

/// Matches if the function returns true.
///
/// There is no equivalent CSS selector.
impl<F: Fn(&Node) -> bool> Predicate for F {
    fn matches(&self, node: &Node) -> bool {
        self(node)
    }
}

/// Matches any Element Node.
///
/// This is equivalent to the `*` CSS selector.
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
///
/// There is no equivalent CSS selector.
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
///
/// There is no equivalent CSS selector.
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
///
/// This is equivalent to the `a, b` selector.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Or<A, B>(pub A, pub B);

impl<A: Predicate, B: Predicate> Predicate for Or<A, B> {
    fn matches(&self, node: &Node) -> bool {
        self.0.matches(node) || self.1.matches(node)
    }
}

/// Matches if the inner Predicate `A` and `B` both match the Node.
///
/// This is equivalent to combining CSS selectors, usually by concatenating, e.g. `tag.class`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct And<A, B>(pub A, pub B);

impl<A: Predicate, B: Predicate> Predicate for And<A, B> {
    fn matches(&self, node: &Node) -> bool {
        self.0.matches(node) && self.1.matches(node)
    }
}

/// Matches if inner Predicate `B` matches the node and `A` matches the parent
/// of the node.
///
/// This is equivalent to the `parent > child` CSS selector.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Child<A, B>(pub A, pub B);

impl<A: Predicate, B: Predicate> Predicate for Child<A, B> {
    fn matches(&self, node: &Node) -> bool {
        self.1.matches(node) && node.parent().map_or(false, |parent| self.0.matches(&parent))
    }
}

/// Matches if inner Predicate `B` matches the node and `A` matches any of the
/// parents of node.
///
/// This is equivalent to the `parent descendant` CSS selector.
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

/// Matches if inner Predicate `B` matches the node and `A` matches the node before this one.
///
/// This is equivalent to the `before + after` CSS selector.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ImmediatelyAfter<A, B>(pub A, pub B);

impl<A: Predicate, B: Predicate> Predicate for ImmediatelyAfter<A, B> {
    fn matches(&self, node: &Node) -> bool {
        self.1.matches(node) && node.prev().map_or(false, |prev| self.0.matches(&prev))
    }
}

/// Matches if inner Predicate `B` matches the node and `A` matches any node before this one.
///
/// This is equivalent to the `before ~ after` CSS selector.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct After<A, B>(pub A, pub B);

impl<A: Predicate, B: Predicate> Predicate for After<A, B> {
    fn matches(&self, node: &Node) -> bool {
        if self.1.matches(node) {
            let mut node = *node;
            while let Some(prev) = node.prev() {
                if self.0.matches(&prev) {
                    return true;
                }
                node = prev;
            }
        }
        false
    }
}

/// Matches a node which has no children, except comments.
///
/// This is equivalent to the `:empty` CSS selector.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Empty;

impl Predicate for Empty {
    fn matches(&self, node: &Node) -> bool {
        node.children().all(|n| n.as_comment().is_none())
    }
}

/// Matches the root node of the document.
///
/// This is equivalent to the `:root` CSS selector.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Root;

impl Predicate for Root {
    fn matches(&self, node: &Node) -> bool {
        node.parent().is_none()
    }
}
