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
