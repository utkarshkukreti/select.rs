use std::io;

use html5ever::serialize;
use string_cache::{Atom, Namespace, QualName};
use tendril::StrTendril;

use document::Document;
use predicate::Predicate;
use selection::Selection;

/// The Node type specific data stored by every Node.
#[derive(Clone, Debug, PartialEq)]
pub enum Data {
    Text(StrTendril),
    Element(Atom, Vec<(Atom, StrTendril)>, Vec<usize>),
    Comment(StrTendril),
}

/// Internal representation of a Node. Not of much use without a reference to a
/// Document.
#[derive(Clone, Debug, PartialEq)]
pub struct Raw {
    pub index: usize,
    pub parent: Option<usize>,
    pub prev: Option<usize>,
    pub next: Option<usize>,
    pub data: Data,
}

/// A Node.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Node<'a> {
    document: &'a Document,
    index: usize,
}

impl<'a> Node<'a> {
    pub fn new(document: &'a Document, index: usize) -> Option<Node<'a>> {
        if index < document.nodes.len() {
            Some(Node {
                document: document,
                index: index,
            })
        } else {
            None
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn raw(&self) -> &Raw {
        &self.document.nodes[self.index]
    }

    pub fn data(&self) -> &Data {
        &self.raw().data
    }

    pub fn name(&self) -> Option<&str> {
        match *self.data() {
            Data::Element(ref name, _, _) => Some(name),
            _ => None,
        }
    }

    pub fn attr(&self, name: &str) -> Option<&str> {
        match *self.data() {
            Data::Element(_, ref attrs, _) => {
                let name = Atom::from(name);
                attrs.iter()
                    .find(|&&(ref name_, _)| name == *name_)
                    .map(|&(_, ref value)| value.as_ref())
            }
            _ => None,
        }
    }

    pub fn parent(&self) -> Option<Node<'a>> {
        self.raw().parent.map(|index| self.document.nth(index).unwrap())
    }

    pub fn prev(&self) -> Option<Node<'a>> {
        self.raw().prev.map(|index| self.document.nth(index).unwrap())
    }

    pub fn next(&self) -> Option<Node<'a>> {
        self.raw().next.map(|index| self.document.nth(index).unwrap())
    }

    pub fn first_child(&self) -> Option<Node<'a>> {
        match *self.data() {
            Data::Element(_, _, ref children) => {
                children.first().map(|&index| self.document.nth(index).unwrap())
            }
            _ => None,
        }
    }

    pub fn last_child(&self) -> Option<Node<'a>> {
        match *self.data() {
            Data::Element(_, _, ref children) => {
                children.last().map(|&index| self.document.nth(index).unwrap())
            }
            _ => None,
        }
    }

    pub fn text(&self) -> String {
        let mut string = String::new();
        recur(&self.document, self.index, &mut string);
        return string;

        fn recur(document: &Document, index: usize, string: &mut String) {
            match document.nodes[index].data {
                Data::Text(ref text) => string.push_str(text),
                Data::Element(_, _, ref children) => {
                    for &child in children {
                        recur(document, child, string)
                    }
                }
                Data::Comment(_) => {}
            }
        }
    }

    pub fn html(&self) -> String {
        let mut buf = Vec::new();
        serialize::serialize(&mut buf, self, Default::default()).unwrap();
        String::from_utf8(buf).unwrap()
    }

    pub fn inner_html(&self) -> String {
        let mut buf = Vec::new();
        if let Data::Element(_, _, ref children) = *self.data() {
            for &child in children {
                serialize::serialize(&mut buf,
                                     &self.document.nth(child).unwrap(),
                                     Default::default())
                    .unwrap();
            }
        }
        String::from_utf8(buf).unwrap()
    }

    pub fn find<P: Predicate>(&self, p: P) -> Selection<'a> {
        Selection::new(self.document, [self.index].iter().cloned().collect()).find(p)
    }

    pub fn is<P: Predicate>(&self, p: P) -> bool {
        p.matches(self)
    }

    pub fn as_text(&self) -> Option<&str> {
        match *self.data() {
            Data::Text(ref text) => Some(&text),
            _ => None,
        }
    }

    pub fn as_comment(&self) -> Option<&str> {
        match *self.data() {
            Data::Comment(ref comment) => Some(&comment),
            _ => None,
        }
    }

    pub fn children(&self) -> Selection<'a> {
        Selection::new(self.document,
                       match *self.data() {
                           Data::Element(_, _, ref children) => children.iter().cloned().collect(),
                           _ => [].iter().cloned().collect(),
                       })
    }

    pub fn descendants(&self) -> Descendants<'a> {
        Descendants {
            start: *self,
            current: *self,
            done: false,
        }
    }
}

impl<'a> serialize::Serializable for Node<'a> {
    fn serialize<'w, W: io::Write>(&self,
                                   serializer: &mut serialize::Serializer<'w, W>,
                                   traversal_scope: serialize::TraversalScope)
                                   -> io::Result<()> {
        match *self.data() {
            Data::Text(ref text) => serializer.write_text(&text),
            Data::Element(ref name, ref attrs, ref children) => {
                let ns = Namespace("".into());
                let name = QualName::new(ns.clone(), name.clone());

                // FIXME: I couldn't get this to work without this awful hack.
                let attrs = attrs.iter()
                    .map(|&(ref name, ref value)| {
                        (QualName::new(ns.clone(), name.clone()), value.as_ref())
                    })
                    .collect::<Vec<(QualName, &str)>>();
                let attrs = attrs.iter().map(|&(ref name, ref value)| (name, *value));

                try!(serializer.start_elem(name.clone(), attrs));

                for &child in children {
                    let child = self.document.nth(child).unwrap();
                    try!(serialize::Serializable::serialize(&child, serializer, traversal_scope));
                }

                try!(serializer.end_elem(name.clone()));

                Ok(())
            }
            Data::Comment(ref comment) => serializer.write_comment(&comment),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Descendants<'a> {
    start: Node<'a>,
    current: Node<'a>,
    done: bool,
}

impl<'a> Iterator for Descendants<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Node<'a>> {
        if self.done {
            return None;
        }

        // If this is the start, we can only descdend into children.
        if self.start.index() == self.current.index() {
            if let Some(first_child) = self.current.first_child() {
                self.current = first_child;
            } else {
                self.done = true;
                return None;
            }
        } else {
            // Otherwise we can also go to next sibling.
            if let Some(first_child) = self.current.first_child() {
                self.current = first_child;
            } else if let Some(next) = self.current.next() {
                self.current = next;
            } else {
                loop {
                    // This unwrap should never fail.
                    let parent = self.current.parent().unwrap();
                    if parent.index() == self.start.index() {
                        self.done = true;
                        return None;
                    }
                    if let Some(next) = parent.next() {
                        self.current = next;
                        break;
                    }
                    self.current = parent;
                }
            }
        }

        Some(self.current)
    }
}
