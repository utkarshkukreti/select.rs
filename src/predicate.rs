use node::Node;

pub trait Predicate {
    fn matches(&self, node: &Node) -> bool;
}
