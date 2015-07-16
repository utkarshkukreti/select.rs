use dom::Dom;
use bit_set::BitSet;

#[derive(Clone, Debug, PartialEq)]
pub struct Selection<'a> {
    pub dom: &'a Dom,
    pub bitset: BitSet
}
