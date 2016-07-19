use document::Document;
use bit_set::{self, BitSet};
use node::{self, Node};
use predicate::Predicate;

#[derive(Clone, Debug, PartialEq)]
pub struct Selection<'a> {
    document: &'a Document,
    bit_set: BitSet,
}

impl<'a> Selection<'a> {
    pub fn new(document: &'a Document, bit_set: BitSet) -> Selection<'a> {
        Selection {
            document: document,
            bit_set: bit_set,
        }
    }

    pub fn iter<'sel>(&'sel self) -> Iter<'sel, 'a> {
        Iter {
            selection: self,
            inner: self.bit_set.iter(),
        }
    }

    pub fn filter<P: Predicate>(&self, p: P) -> Selection<'a> {
        Selection {
            document: self.document,
            bit_set: self.bit_set
                .iter()
                .filter(|&index| p.matches(&self.document.nth(index).unwrap()))
                .collect(),
        }
    }

    pub fn find<P: Predicate>(&self, p: P) -> Selection<'a> {
        let mut bit_set = BitSet::new();

        for index in &self.bit_set {
            recur(self.document, &mut bit_set, index);
        }

        return Selection {
            document: self.document,
            bit_set: bit_set.iter()
                .filter(|&index| p.matches(&self.document.nth(index).unwrap()))
                .collect(),
        };

        fn recur(document: &Document, bit_set: &mut BitSet, index: usize) {
            if bit_set.contains(index) {
                return;
            }

            match document.nodes[index].data {
                node::Data::Text(..) => {}
                node::Data::Element(_, _, ref children) => {
                    for &child in children {
                        recur(document, bit_set, child);
                        bit_set.insert(child);
                    }
                }
                node::Data::Comment(..) => {}
            }
        }
    }

    pub fn parent(&self) -> Selection<'a> {
        Selection {
            document: self.document,
            bit_set: self.iter()
                .filter_map(|node| node.parent().map(|parent| parent.index()))
                .collect(),
        }
    }

    pub fn prev(&self) -> Selection<'a> {
        Selection {
            document: self.document,
            bit_set: self.iter()
                .filter_map(|node| node.prev().map(|prev| prev.index()))
                .collect(),
        }
    }

    pub fn next(&self) -> Selection<'a> {
        Selection {
            document: self.document,
            bit_set: self.iter()
                .filter_map(|node| node.next().map(|next| next.index()))
                .collect(),
        }
    }

    pub fn parents(&self) -> Selection<'a> {
        let mut bit_set = BitSet::new();
        for mut node in self {
            while let Some(parent) = node.parent() {
                bit_set.insert(parent.index());
                node = parent;
            }
        }

        Selection {
            document: self.document,
            bit_set: bit_set,
        }
    }

    pub fn children(&self) -> Selection<'a> {
        let mut bit_set = BitSet::new();
        for node in self {
            match self.document.nodes[node.index()].data {
                node::Data::Text(_) => {}
                node::Data::Element(_, _, ref children) => {
                    for &child in children {
                        bit_set.insert(child);
                    }
                }
                node::Data::Comment(..) => {}
            }
        }

        Selection {
            document: self.document,
            bit_set: bit_set,
        }
    }

    pub fn first(&self) -> Option<Node<'a>> {
        self.bit_set.iter().next().map(|index| self.document.nth(index).unwrap())
    }

    pub fn last(&self) -> Option<Node<'a>> {
        self.bit_set.iter().last().map(|index| self.document.nth(index).unwrap())
    }

    pub fn len(&self) -> usize {
        self.bit_set.len()
    }
}

#[derive(Clone)]
pub struct Iter<'sel, 'doc: 'sel> {
    selection: &'sel Selection<'doc>,
    inner: bit_set::Iter<'sel, u32>,
}

impl<'sel, 'doc> Iterator for Iter<'sel, 'doc> {
    type Item = Node<'doc>;

    fn next(&mut self) -> Option<Node<'doc>> {
        self.inner.next().map(|index| self.selection.document.nth(index).unwrap())
    }
}

impl<'sel, 'doc> IntoIterator for &'sel Selection<'doc> {
    type Item = Node<'doc>;
    type IntoIter = Iter<'sel, 'doc>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
