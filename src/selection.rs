use dom::Dom;
use bit_set::BitSet;
use node::Node;

#[derive(Clone, Debug, PartialEq)]
pub struct Selection<'a> {
    pub dom: &'a Dom,
    pub bitset: BitSet
}

impl<'a> Selection<'a> {
    pub fn iter(&'a self) -> Iter<'a> {
        Iter {
            selection: self,
            next: 0
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Iter<'a> {
    pub selection: &'a Selection<'a>,
    pub next: usize
}

impl<'a> Iterator for Iter<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Node<'a>> {
        while self.next < self.selection.dom.nodes.len() {
            self.next += 1;
            if self.selection.bitset.contains(&(self.next - 1)) {
                return Some(Node {
                    dom: &self.selection.dom,
                    id: self.next - 1
                });
            }
        }
        None
    }
}
