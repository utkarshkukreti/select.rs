use std::{fmt, io};

use html5ever::serialize;
use html5ever_atoms::{LocalNameStaticSet, QualName};
use string_cache::Atom;
use tendril::StrTendril;

use document::Document;
use predicate::Predicate;
use selection::Selection;

/// The Node type specific data stored by every Node.
#[derive(Clone, Debug, PartialEq)]
pub enum Data {
    Text(StrTendril),
    Element(Atom<LocalNameStaticSet>, Vec<(Atom<LocalNameStaticSet>, StrTendril)>),
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
    pub first_child: Option<usize>,
    pub last_child: Option<usize>,
    pub data: Data,
}

/// A Node.
#[derive(Copy, Clone, PartialEq)]
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

    pub fn raw(&self) -> &'a Raw {
        &self.document.nodes[self.index]
    }

    pub fn data(&self) -> &'a Data {
        &self.raw().data
    }

    pub fn name(&self) -> Option<&'a str> {
        match *self.data() {
            Data::Element(ref name, _) => Some(name),
            _ => None,
        }
    }

    pub fn attr(&self, name: &str) -> Option<&'a str> {
        match *self.data() {
            Data::Element(_, ref attrs) => {
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
        self.raw().first_child.map(|index| self.document.nth(index).unwrap())
    }

    pub fn last_child(&self) -> Option<Node<'a>> {
        self.raw().last_child.map(|index| self.document.nth(index).unwrap())
    }

    pub fn text(&self) -> String {
        let mut string = String::new();
        recur(self, &mut string);
        return string;

        fn recur(node: &Node, string: &mut String) {
            if let Some(text) = node.as_text() {
                string.push_str(text);
            }
            for child in node.children() {
                recur(&child, string)
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
        for child in self.children() {
            serialize::serialize(&mut buf, &child, Default::default()).unwrap();
        }
        String::from_utf8(buf).unwrap()
    }

    pub fn find<P: Predicate>(&self, predicate: P) -> Find<P> {
        Find {
            document: self.document,
            descendants: self.descendants(),
            predicate: predicate,
        }
    }

    pub fn is<P: Predicate>(&self, p: P) -> bool {
        p.matches(self)
    }

    pub fn as_text(&self) -> Option<&'a str> {
        match *self.data() {
            Data::Text(ref text) => Some(&text),
            _ => None,
        }
    }

    pub fn as_comment(&self) -> Option<&'a str> {
        match *self.data() {
            Data::Comment(ref comment) => Some(&comment),
            _ => None,
        }
    }

    pub fn children(&self) -> Children<'a> {
        Children {
            document: self.document,
            next: self.first_child(),
        }
    }

    pub fn descendants(&self) -> Descendants<'a> {
        Descendants {
            start: *self,
            current: *self,
            done: false,
        }
    }
}

impl<'a> fmt::Debug for Node<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        struct Attrs<'a>(&'a [(Atom<LocalNameStaticSet>, StrTendril)]);

        impl<'a> fmt::Debug for Attrs<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                self.0
                    .iter()
                    .fold(f.debug_list(), |mut f, &(ref name, ref value)| {
                        f.entry(&(&&**name, &&**value));
                        f
                    })
                    .finish()
            }
        }

        struct Children<'a>(&'a Node<'a>);

        impl<'a> fmt::Debug for Children<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                f.debug_list().entries(self.0.children()).finish()
            }
        }

        match *self.data() {
            Data::Text(ref text) => f.debug_tuple("Text").field(&&**text).finish(),
            Data::Element(ref name, ref attrs) => {
                f.debug_struct("Element")
                    .field("name", &&**name)
                    .field("attrs", &Attrs(attrs))
                    .field("children", &Children(self))
                    .finish()
            }
            Data::Comment(ref comment) => f.debug_tuple("Comment").field(&&**comment).finish(),
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
            Data::Element(ref name, ref attrs) => {
                let ns = Atom::from("");
                let name = QualName::new(ns.clone(), name.clone());

                // FIXME: I couldn't get this to work without this awful hack.
                let attrs = attrs.iter()
                    .map(|&(ref name, ref value)| {
                        (QualName::new(ns.clone(), name.clone()), value.as_ref())
                    })
                    .collect::<Vec<(QualName, &str)>>();
                let attrs = attrs.iter().map(|&(ref name, ref value)| (name, *value));

                try!(serializer.start_elem(name.clone(), attrs));

                for child in self.children() {
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

pub struct Find<'a, P: Predicate> {
    document: &'a Document,
    descendants: Descendants<'a>,
    predicate: P,
}

impl<'a, P: Predicate> Find<'a, P> {
    pub fn into_selection(self) -> Selection<'a> {
        Selection::new(self.document, self.map(|node| node.index()).collect())
    }
}

impl<'a, P: Predicate> Iterator for Find<'a, P> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Node<'a>> {
        for node in &mut self.descendants {
            if self.predicate.matches(&node) {
                return Some(node);
            }
        }
        None
    }
}

pub struct Children<'a> {
    document: &'a Document,
    next: Option<Node<'a>>,
}

impl<'a> Children<'a> {
    pub fn into_selection(self) -> Selection<'a> {
        Selection::new(self.document, self.map(|node| node.index()).collect())
    }
}

impl<'a> Iterator for Children<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Node<'a>> {
        if let Some(next) = self.next {
            self.next = next.next();
            Some(next)
        } else {
            None
        }
    }
}
