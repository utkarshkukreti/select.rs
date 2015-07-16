use dom::Dom;
use bit_set::BitSet;
use node::{self, Node};
use predicate::Predicate;

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

    pub fn filter<P: Predicate>(&'a self, p: P) -> Selection<'a> {
        Selection {
            dom: self.dom,
            bitset: self.iter().filter_map(|node| {
                if p.matches(&node) {
                    Some(node.id)
                } else {
                    None
                }
            }).collect()
        }
    }

    pub fn find<P: Predicate>(&'a self, p: P) -> Selection<'a> {
        let mut bitset = BitSet::new();

        for id in self.bitset.iter() {
            recur(self.dom, &mut bitset, id);
        }

        return Selection {
            dom: self.dom,
            bitset: bitset.iter().filter(|&id| {
                let node = node::Node { dom: self.dom, id: id };
                p.matches(&node)
            }).collect()
        };

        fn recur(dom: &Dom, bitset: &mut BitSet, id: node::Ref) {
            if bitset.contains(&id) {
                return
            }

            match dom.nodes[id].data {
                node::Data::Text(..) => {},
                node::Data::Element(_, _, ref children) => {
                    for &child in children {
                        recur(dom, bitset, child);
                        bitset.insert(child);
                    }
                }
            }
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
