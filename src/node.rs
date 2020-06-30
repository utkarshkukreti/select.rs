use std::{fmt, io};

use html5ever::tendril::StrTendril;
use html5ever::{serialize, LocalName, QualName};

use document::Document;
use predicate::Predicate;
use selection::Selection;

/// The Node type specific data stored by every Node.
#[derive(Clone, Debug, PartialEq)]
pub enum Data {
    Text(StrTendril),
    Element(QualName, Vec<(QualName, StrTendril)>),
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

/// A single node of an HTML document. Nodes may be HTML elements, comments, or text nodes.
#[derive(Copy, Clone, PartialEq)]
pub struct Node<'a> {
    document: &'a Document,
    index: usize,
}

impl<'a> Node<'a> {
    /// Create a Node referring to the `index`th Node of a document.
    pub fn new(document: &'a Document, index: usize) -> Option<Node<'a>> {
        if index < document.nodes.len() {
            Some(Node { document, index })
        } else {
            None
        }
    }

    /// Get the index of this Node in its Document.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Obtain the inner representation of this Node.
    pub fn raw(&self) -> &'a Raw {
        &self.document.nodes[self.index]
    }

    /// Get the text node, HTML element, or comment from a Node.
    pub fn data(&self) -> &'a Data {
        &self.raw().data
    }

    /// Get the name of a Node if it is an HTML element, or None otherwise.
    pub fn name(&self) -> Option<&'a str> {
        match *self.data() {
            Data::Element(ref name, _) => Some(&name.local),
            _ => None,
        }
    }

    /// Get the value of the attribute `name` from a Node representing a HTML element.
    pub fn attr(&self, name: &str) -> Option<&'a str> {
        match *self.data() {
            Data::Element(_, ref attrs) => {
                let name = LocalName::from(name);
                attrs
                    .iter()
                    .find(|&&(ref name_, _)| name == name_.local)
                    .map(|&(_, ref value)| value.as_ref())
            }
            _ => None,
        }
    }

    /// Get an iterator over the names and values of attributes of the Element.
    /// Returns an empty iterator for non Element nodes.
    pub fn attrs(&self) -> Attrs<'a> {
        match *self.data() {
            Data::Element(_, ref attrs) => Attrs {
                inner: Some(attrs.iter()),
            },
            _ => Attrs { inner: None },
        }
    }

    pub fn parent(&self) -> Option<Node<'a>> {
        self.raw()
            .parent
            .map(|index| self.document.nth(index).unwrap())
    }

    pub fn prev(&self) -> Option<Node<'a>> {
        self.raw()
            .prev
            .map(|index| self.document.nth(index).unwrap())
    }

    pub fn next(&self) -> Option<Node<'a>> {
        self.raw()
            .next
            .map(|index| self.document.nth(index).unwrap())
    }

    pub fn first_child(&self) -> Option<Node<'a>> {
        self.raw()
            .first_child
            .map(|index| self.document.nth(index).unwrap())
    }

    pub fn last_child(&self) -> Option<Node<'a>> {
        self.raw()
            .last_child
            .map(|index| self.document.nth(index).unwrap())
    }

    /// Get the combined textual content of a Node and all of its children.
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

    /// Serialize a Node to an HTML string.
    pub fn html(&self) -> String {
        let mut buf = Vec::new();
        serialize::serialize(&mut buf, self, Default::default()).unwrap();
        String::from_utf8(buf).unwrap()
    }

    /// Serialize a Node's children to an HTML string.
    pub fn inner_html(&self) -> String {
        let mut buf = Vec::new();
        for child in self.children() {
            serialize::serialize(&mut buf, &child, Default::default()).unwrap();
        }
        String::from_utf8(buf).unwrap()
    }

    /// Search for Nodes fulfilling `predicate` in the descendants of a Node.
    pub fn find<P: Predicate>(&self, predicate: P) -> Find<'a, P> {
        Find {
            document: self.document,
            descendants: self.descendants(),
            predicate,
        }
    }

    /// Evaluate a predicate on this Node.
    pub fn is<P: Predicate>(&self, p: P) -> bool {
        p.matches(self)
    }

    /// Get the text of a text Node, or None if the node is not text.
    pub fn as_text(&self) -> Option<&'a str> {
        match *self.data() {
            Data::Text(ref text) => Some(&text),
            _ => None,
        }
    }

    /// Get the text of a comment Node, or None if the node is not a comment.
    pub fn as_comment(&self) -> Option<&'a str> {
        match *self.data() {
            Data::Comment(ref comment) => Some(&comment),
            _ => None,
        }
    }

    /// Construct an iterator over a Node's child Nodes.
    pub fn children(&self) -> Children<'a> {
        Children {
            document: self.document,
            next: self.first_child(),
        }
    }

    /// Construct an iterator over a Node's descendant (transitive children) Nodes.
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
        struct Attrs<'a>(&'a [(QualName, StrTendril)]);

        impl<'a> fmt::Debug for Attrs<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                self.0
                    .iter()
                    .fold(f.debug_list(), |mut f, &(ref name, ref value)| {
                        f.entry(&(&*name.local, &&**value));
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
            Data::Element(ref name, ref attrs) => f
                .debug_struct("Element")
                .field("name", &&*name.local)
                .field("attrs", &Attrs(attrs))
                .field("children", &Children(self))
                .finish(),
            Data::Comment(ref comment) => f.debug_tuple("Comment").field(&&**comment).finish(),
        }
    }
}

impl<'a> serialize::Serialize for Node<'a> {
    fn serialize<S: serialize::Serializer>(
        &self,
        serializer: &mut S,
        traversal_scope: serialize::TraversalScope,
    ) -> io::Result<()> {
        match *self.data() {
            Data::Text(ref text) => serializer.write_text(&text),
            Data::Element(ref name, ref attrs) => {
                let attrs = attrs.iter().map(|&(ref name, ref value)| (name, &**value));

                serializer.start_elem(name.clone(), attrs)?;

                for child in self.children() {
                    serialize::Serialize::serialize(&child, serializer, traversal_scope.clone())?;
                }

                serializer.end_elem(name.clone())?;

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

impl<'a, P: Predicate> std::fmt::Debug for Find<'a, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Find")
            .field("document", &self.document)
            .field("descendants", &self.descendants)
            // predicate may be closure not implementing Debug
            .finish()
    }
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

pub struct Attrs<'a> {
    inner: Option<::std::slice::Iter<'a, (QualName, StrTendril)>>,
}

impl<'a> Iterator for Attrs<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.as_mut().and_then(|it| {
            it.next()
                .map(|&(ref name, ref value)| (name.local.as_ref(), value.as_ref()))
        })
    }
}
