use crate::document::Document;
use crate::node::Node;
use crate::predicate::Predicate;
use bit_set::{self, BitSet};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Selection<'a> {
    document: &'a Document,
    bit_set: BitSet,
}

impl<'a> Selection<'a> {
    pub fn new(document: &'a Document, bit_set: BitSet) -> Selection<'a> {
        Selection { document, bit_set }
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
            bit_set: self
                .bit_set
                .iter()
                .filter(|&index| p.matches(&self.document.nth(index).unwrap()))
                .collect(),
        }
    }

    pub fn select<P: Predicate>(&self, p: P) -> Selection<'a> {
        let mut bit_set = BitSet::new();

        for node in self {
            recur(&node, &mut bit_set);
        }

        return Selection {
            document: self.document,
            bit_set: bit_set
                .iter()
                .filter(|&index| p.matches(&self.document.nth(index).unwrap()))
                .collect(),
        };

        fn recur(node: &Node, bit_set: &mut BitSet) {
            if bit_set.contains(node.index()) {
                return;
            }

            for child in node.children() {
                recur(&child, bit_set);
                bit_set.insert(child.index());
            }
        }
    }

    #[deprecated = "renamed to select()"]
    pub fn find<P: Predicate>(&self, p: P) -> Selection<'a> {
        self.select(p)
    }

    pub fn parent(&self) -> Selection<'a> {
        Selection {
            document: self.document,
            bit_set: self
                .iter()
                .filter_map(|node| node.parent().map(|parent| parent.index()))
                .collect(),
        }
    }

    pub fn prev(&self) -> Selection<'a> {
        Selection {
            document: self.document,
            bit_set: self
                .iter()
                .filter_map(|node| node.prev().map(|prev| prev.index()))
                .collect(),
        }
    }

    pub fn next(&self) -> Selection<'a> {
        Selection {
            document: self.document,
            bit_set: self
                .iter()
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
            bit_set,
        }
    }

    pub fn children(&self) -> Selection<'a> {
        let mut bit_set = BitSet::new();
        for node in self {
            for child in node.children() {
                bit_set.insert(child.index());
            }
        }

        Selection {
            document: self.document,
            bit_set,
        }
    }

    pub fn first(&self) -> Option<Node<'a>> {
        self.bit_set
            .iter()
            .next()
            .map(|index| self.document.nth(index).unwrap())
    }

    pub fn last(&self) -> Option<Node<'a>> {
        self.bit_set
            .iter()
            .last()
            .map(|index| self.document.nth(index).unwrap())
    }

    pub fn len(&self) -> usize {
        self.bit_set.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bit_set.is_empty()
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
        self.inner
            .next()
            .map(|index| self.selection.document.nth(index).unwrap())
    }
}

impl<'sel, 'doc> IntoIterator for &'sel Selection<'doc> {
    type Item = Node<'doc>;
    type IntoIter = Iter<'sel, 'doc>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
