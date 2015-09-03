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
            bitset: self.bitset.iter().filter(|&index| {
                p.matches(&self.dom.nth(index))
            }).collect()
        }
    }

    pub fn find<P: Predicate>(&self, p: P) -> Selection<'a> {
        let mut bitset = BitSet::new();

        for index in self.bitset.iter() {
            recur(self.dom, &mut bitset, index);
        }

        return Selection {
            dom: self.dom,
            bitset: bitset.iter().filter(|&index| {
                p.matches(&self.dom.nth(index))
            }).collect()
        };

        fn recur(dom: &Dom, bitset: &mut BitSet, index: usize) {
            if bitset.contains(&index) {
                return
            }

            match dom.nodes[index].data {
                node::Data::Text(..) => {},
                node::Data::Element(_, _, ref children) => {
                    for &child in children {
                        recur(dom, bitset, child);
                        bitset.insert(child);
                    }
                },
                node::Data::Comment(..) => {}
            }
        }
    }

    pub fn parent(&self) -> Selection<'a> {
        Selection {
            dom: self.dom,
            bitset: self.iter().filter_map(|node| {
                node.parent().map(|parent| parent.index())
            }).collect()
        }
    }

    pub fn prev(&self) -> Selection<'a> {
        Selection {
            dom: self.dom,
            bitset: self.iter().filter_map(|node| {
                node.prev().map(|prev| prev.index())
            }).collect()
        }
    }

    pub fn next(&self) -> Selection<'a> {
        Selection {
            dom: self.dom,
            bitset: self.iter().filter_map(|node| {
                node.next().map(|next| next.index())
            }).collect()
        }
    }

    pub fn parents(&self) -> Selection<'a> {
        let mut bitset = BitSet::new();
        for mut node in self.iter() {
            while let Some(parent) = node.parent() {
                bitset.insert(parent.index());
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
            match self.dom.nodes[node.index()].data {
                node::Data::Text(_) => {},
                node::Data::Element(_, _, ref children) => {
                    for &child in children {
                        bitset.insert(child);
                    }
                },
                node::Data::Comment(..) => {}
            }
        }

        Selection {
            dom: self.dom,
            bitset: bitset
        }
    }

    pub fn first(&self) -> Option<Node<'a>> {
        self.bitset.iter().next().map(|index| self.dom.nth(index))
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
        self.inner.next().map(|index| self.selection.dom.nth(index))
    }
}
