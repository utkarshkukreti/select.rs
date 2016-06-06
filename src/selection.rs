use document::Document;
use bit_set::{self, BitSet};
use node::{self, Node};
use predicate::Predicate;

#[derive(Clone, Debug, PartialEq)]
pub struct Selection<'a> {
    document: &'a Document,
    bitset: BitSet
}

impl<'a> Selection<'a> {
    pub fn new(document: &'a Document, bitset: BitSet) -> Selection<'a> {
        Selection {
            document: document,
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
            document: self.document,
            bitset: self.bitset.iter().filter(|&index| {
                p.matches(&self.document.nth(index).unwrap())
            }).collect()
        }
    }

    pub fn find<P: Predicate>(&self, p: P) -> Selection<'a> {
        let mut bitset = BitSet::new();

        for index in &self.bitset {
            recur(self.document, &mut bitset, index);
        }

        return Selection {
            document: self.document,
            bitset: bitset.iter().filter(|&index| {
                p.matches(&self.document.nth(index).unwrap())
            }).collect()
        };

        fn recur(document: &Document, bitset: &mut BitSet, index: usize) {
            if bitset.contains(index) {
                return
            }

            match document.nodes[index].data {
                node::Data::Text(..) => {},
                node::Data::Element(_, _, ref children) => {
                    for &child in children {
                        recur(document, bitset, child);
                        bitset.insert(child);
                    }
                },
                node::Data::Comment(..) => {}
            }
        }
    }

    pub fn parent(&self) -> Selection<'a> {
        Selection {
            document: self.document,
            bitset: self.iter().filter_map(|node| {
                node.parent().map(|parent| parent.index())
            }).collect()
        }
    }

    pub fn prev(&self) -> Selection<'a> {
        Selection {
            document: self.document,
            bitset: self.iter().filter_map(|node| {
                node.prev().map(|prev| prev.index())
            }).collect()
        }
    }

    pub fn next(&self) -> Selection<'a> {
        Selection {
            document: self.document,
            bitset: self.iter().filter_map(|node| {
                node.next().map(|next| next.index())
            }).collect()
        }
    }

    pub fn parents(&self) -> Selection<'a> {
        let mut bitset = BitSet::new();
        for mut node in self {
            while let Some(parent) = node.parent() {
                bitset.insert(parent.index());
                node = parent;
            }
        }

        Selection {
            document: self.document,
            bitset: bitset
        }
    }

    pub fn children(&self) -> Selection<'a> {
        let mut bitset = BitSet::new();
        for node in self {
            match self.document.nodes[node.index()].data {
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
            document: self.document,
            bitset: bitset
        }
    }

    pub fn first(&self) -> Option<Node<'a>> {
        self.bitset.iter().next().map(|index| self.document.nth(index).unwrap())
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
        self.inner.next().map(|index| self.selection.document.nth(index).unwrap())
    }
}

impl<'a> IntoIterator for &'a Selection<'a> {
    type Item = Node<'a>;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
