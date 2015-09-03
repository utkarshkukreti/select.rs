use std::collections::HashMap;

use dom::Dom;
use predicate::Predicate;
use selection::Selection;

#[derive(Clone, Debug, PartialEq)]
pub enum Data {
    Text(String),
    Element(String, HashMap<String, String>, Vec<usize>),
    Comment(String)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Raw {
    pub ref_: usize,
    pub parent: Option<usize>,
    pub prev: Option<usize>,
    pub next: Option<usize>,
    pub data: Data
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Node<'a> {
    dom: &'a Dom,
    ref_: usize
}

impl<'a> Node<'a> {
    pub fn new(dom: &'a Dom, ref_: usize) -> Node<'a> {
        Node {
            dom: dom,
            ref_: ref_
        }
    }

    pub fn ref_(&self) -> usize {
        self.ref_
    }

    pub fn data(&self) -> &Data {
        &self.dom.nodes[self.ref_].data
    }

    pub fn name(&self) -> Option<&str> {
        match self.dom.nodes[self.ref_].data {
            Data::Element(ref name, _, _) => Some(name),
            _ => None
        }
    }

    pub fn attr(&self, name: &str) -> Option<&str> {
        match self.dom.nodes[self.ref_].data {
            Data::Element(_, ref attrs, _) => attrs.get(name).map(|s| &s[..]),
            _ => None
        }
    }

    pub fn parent(&self) -> Option<Node<'a>> {
        self.dom.nodes[self.ref_].parent.map(|ref_| self.dom.nth(ref_))
    }

    pub fn prev(&self) -> Option<Node<'a>> {
        self.dom.nodes[self.ref_].prev.map(|ref_| self.dom.nth(ref_))
    }

    pub fn next(&self) -> Option<Node<'a>> {
        self.dom.nodes[self.ref_].next.map(|ref_| self.dom.nth(ref_))
    }

    pub fn text(&self) -> String {
        let mut string = String::new();
        recur(&self.dom, self.ref_, &mut string);
        return string;

        fn recur(dom: &Dom, ref_: usize, string: &mut String) {
            match dom.nodes[ref_].data {
                Data::Text(ref text) => string.push_str(text),
                Data::Element(_, _, ref children) => {
                    for &child in children {
                        recur(dom, child, string)
                    }
                },
                Data::Comment(_) => {}
            }
        }
    }

    pub fn find<P: Predicate>(&self, p: P) -> Selection<'a> {
        Selection::new(self.dom, [self.ref_].iter().cloned().collect()).find(p)
    }

    pub fn is<P: Predicate>(&self, p: P) -> bool {
        p.matches(self)
    }
}
