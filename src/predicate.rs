use node::Node;

pub trait Predicate {
    fn matches(&self, node: &Node) -> bool;
}

impl Predicate for () {
    fn matches(&self, _: &Node) -> bool {
        true
    }
}

pub struct Name<T>(pub T);

impl<T: AsRef<str>> Predicate for Name<T> {
    fn matches(&self, node: &Node) -> bool {
        node.name() == Some(self.0.as_ref())
    }
}

pub struct Class<T>(pub T);

impl<T: AsRef<str>> Predicate for Class<T> {
    fn matches(&self, node: &Node) -> bool {
        node.attr("class").map(|classes| {
            classes.split_whitespace().any(|class| class == self.0.as_ref())
        }).unwrap_or(false)
    }
}

pub struct Not<T>(pub T);

impl<T: Predicate> Predicate for Not<T> {
    fn matches(&self, node: &Node) -> bool {
        !self.0.matches(node)
    }
}

pub struct Attr<N, V>(pub N, pub V);

impl<N: AsRef<str>, V: AsRef<str>> Predicate for Attr<N, V> {
    fn matches(&self, node: &Node) -> bool {
        node.attr(self.0.as_ref()) == Some(self.1.as_ref())
    }
}
