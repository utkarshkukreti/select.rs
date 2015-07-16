use dom::Dom;
use bit_set::BitSet;

#[derive(Clone, Debug, PartialEq)]
pub struct Selection<'a> {
    pub dom: &'a Dom,
    pub bitset: BitSet
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Iter<'a> {
    pub selection: &'a Selection<'a>,
    pub next: usize
}
