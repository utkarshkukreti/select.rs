use std::collections::HashMap;

use dom::Dom;

pub type Ref = usize;

#[derive(Clone, Debug, PartialEq)]
pub enum Data {
    Text(String),
    Element(String, HashMap<String, String>, Vec<Ref>)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Raw {
    pub id: Ref,
    pub parent: Option<Ref>,
    pub prev: Option<Ref>,
    pub next: Option<Ref>,
    pub data: Data
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Node<'a> {
    pub dom: &'a Dom,
    pub id: Ref
}

impl<'a> Node<'a> {
    pub fn name(&self) -> Option<&str> {
        match self.dom.nodes[self.id].data {
            Data::Text(..) => None,
            Data::Element(ref name, _, _) => Some(name)
        }
    }

    pub fn attr(&self, name: &str) -> Option<&str> {
        match self.dom.nodes[self.id].data {
            Data::Text(..) => None,
            Data::Element(_, ref attrs, _) => attrs.get(name).map(|s| &s[..])
        }
    }

    pub fn parent(&self) -> Option<Node<'a>> {
        self.dom.nodes[self.id].parent.map(|id| self.dom.nth(id))
    }

    pub fn prev(&self) -> Option<Node<'a>> {
        self.dom.nodes[self.id].prev.map(|id| self.dom.nth(id))
    }

    pub fn next(&self) -> Option<Node<'a>> {
        self.dom.nodes[self.id].next.map(|id| self.dom.nth(id))
    }
}
