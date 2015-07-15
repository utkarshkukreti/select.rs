use node::Node;

pub trait Predicate {
    fn matches(&self, node: &Node) -> bool;
}

pub struct Name<T>(pub T);

impl<T: AsRef<str>> Predicate for Name<T> {
    fn matches(&self, node: &Node) -> bool {
        node.name() == Some(self.0.as_ref())
    }
}

pub struct Id<T>(pub T);

impl<T: AsRef<str>> Predicate for Id<T> {
    fn matches(&self, node: &Node) -> bool {
        node.attr("id") == Some(self.0.as_ref())
    }
}
