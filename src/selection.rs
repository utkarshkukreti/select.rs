use dom::Dom;
use bit_set::{self, BitSet};
use node::{self, Node};
use predicate::Predicate;

#[derive(Clone, Debug, PartialEq)]
pub struct Selection<'a> {
    dom: &'a Dom,
    bitset: BitSet
}

impl<'a> Selection<'a> {
    pub fn new(dom: &'a Dom, bitset: BitSet) -> Selection<'a> {
        Selection {
            dom: dom,
            bitset: bitset
        }
    }

    pub fn iter(&'a self) -> Iter<'a> {
        Iter {
            selection: self,
            inner: self.bitset.iter()
        }
    }

    pub fn filter<P: Predicate>(&self, p: P) -> Selection<'a> {
        Selection {
            dom: self.dom,
            bitset: self.bitset.iter().filter(|&id| {
                p.matches(&self.dom.nth(id))
            }).collect()
        }
    }

    pub fn find<P: Predicate>(&self, p: P) -> Selection<'a> {
        let mut bitset = BitSet::new();

        for id in self.bitset.iter() {
            recur(self.dom, &mut bitset, id);
        }

        return Selection {
            dom: self.dom,
            bitset: bitset.iter().filter(|&id| {
                p.matches(&self.dom.nth(id))
            }).collect()
        };

        fn recur(dom: &Dom, bitset: &mut BitSet, id: node::Id) {
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

    pub fn parent(&self) -> Selection<'a> {
        Selection {
            dom: self.dom,
            bitset: self.iter().filter_map(|node| {
                node.parent().map(|parent| parent.id())
            }).collect()
        }
    }

    pub fn prev(&self) -> Selection<'a> {
        Selection {
            dom: self.dom,
            bitset: self.iter().filter_map(|node| {
                node.prev().map(|prev| prev.id())
            }).collect()
        }
    }

    pub fn next(&self) -> Selection<'a> {
        Selection {
            dom: self.dom,
            bitset: self.iter().filter_map(|node| {
                node.next().map(|next| next.id())
            }).collect()
        }
    }

    pub fn parents(&self) -> Selection<'a> {
        let mut bitset = BitSet::new();
        for mut node in self.iter() {
            while let Some(parent) = node.parent() {
                bitset.insert(parent.id());
                node = parent;
            }
        }

        Selection {
            dom: self.dom,
            bitset: bitset
        }
    }

    pub fn children(&self) -> Selection<'a> {
        let mut bitset = BitSet::new();
        for node in self.iter() {
            match self.dom.nodes[node.id()].data {
                node::Data::Text(_) => {},
                node::Data::Element(_, _, ref children) => {
                    for &child in children {
                        bitset.insert(child);
                    }
                }
            }
        }

        Selection {
            dom: self.dom,
            bitset: bitset
        }
    }
}

#[derive(Clone)]
pub struct Iter<'a> {
    selection: &'a Selection<'a>,
    inner: bit_set::Iter<'a, u32>
}

impl<'a> Iterator for Iter<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Node<'a>> {
        self.inner.next().map(|id| self.selection.dom.nth(id))
    }
}
