use std::collections::HashMap;

pub type Ref = usize;

#[derive(Clone, Debug, PartialEq)]
pub enum Data {
    Text(String),
    Element(String, HashMap<String, String>, Vec<Ref>)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Raw {
    id: Ref,
    parent: Option<Ref>,
    prev: Option<Ref>,
    next: Option<Ref>,
    data: Data
}
