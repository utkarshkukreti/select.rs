use std::collections::HashMap;

use dom::Dom;
use predicate::Predicate;
use selection::Selection;

pub type Id = usize;

#[derive(Clone, Debug, PartialEq)]
pub enum Data {
    Text(String),
    Element(String, HashMap<String, String>, Vec<Id>)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Raw {
    pub id: Id,
    pub parent: Option<Id>,
    pub prev: Option<Id>,
    pub next: Option<Id>,
    pub data: Data
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Node<'a> {
    dom: &'a Dom,
    id: Id
}

impl<'a> Node<'a> {
    pub fn new(dom: &'a Dom, id: Id) -> Node<'a> {
        Node {
            dom: dom,
            id: id
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }

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

    pub fn text(&self) -> String {
        let mut string = String::new();
        recur(&self.dom, self.id, &mut string);
        return string;

        fn recur(dom: &Dom, id: Id, string: &mut String) {
            match dom.nodes[id].data {
                Data::Text(ref text) => string.push_str(text),
                Data::Element(_, _, ref children) => {
                    for &child in children {
                        recur(dom, child, string)
                    }
                }
            }
        }
    }

    pub fn find<P: Predicate>(&self, p: P) -> Selection<'a> {
        Selection::new(self.dom, [self.id].iter().cloned().collect()).find(p)
    }
}
