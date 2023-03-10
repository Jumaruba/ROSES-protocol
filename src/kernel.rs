use crate::types::{Dot, TagElement};
use crate::{DotContext, NodeId};
use core::hash::Hash;
use std::fmt::Debug;
use std::{
    collections::{HashSet},
    fmt::Display,
};

#[derive(Clone, Debug, PartialEq, Eq)]

/// The kernel is a structure that performs operations of a crdt.
pub struct Kernel<E>
where
    E: Eq + Display + Clone + Hash + Debug,
{
    pub id: NodeId,
    pub elems: HashSet<TagElement<E>>,
    pub cc: DotContext,
}

impl<E> Kernel<E>
where
    E: Eq + Display + Clone + Hash + Debug,
{
    pub fn new(id: &NodeId) -> Self {
        Self {
            id: id.clone(),
            elems: HashSet::new(),
            cc: DotContext::new(),
        }
    }

    /// Removes all the entries related to the id.
    /// Cleans both elements and dot context.
    pub fn clear(&mut self) {
        self.elems.clear();
        self.cc.clear();
    }

    pub fn has_element(&self, tag: &TagElement<E>) -> bool {
        self.elems.contains(tag)
    }

    /// Gets elements of the kernel.
    /// TODO: to test
    pub fn elements(&self) -> HashSet<E> {
        let mut res: HashSet<E> = HashSet::new();
        self.elems.iter().for_each(|tag_element| {
            res.insert(tag_element.elem.clone());
        });
        res
    }

    /// Adds an element with key equals to self.id and return the added entry.
    /// TODO: to test
    pub fn add(&mut self, elem: E, sck: i64) -> TagElement<E> {
        let dot = self.cc.makedot(&self.id, sck);
        let tag_element = TagElement {
            id: self.id.clone(),
            sck,
            n: dot.n,
            elem,
        };
        self.elems.insert(tag_element.clone());
        tag_element
    }

    /// Remove an element from the set of elements.
    /// TODO: to test
    pub fn rm(&mut self, elem: &E) {
        let elems = self
            .elems
            .drain()
            .filter(|tag_element| {
                return *elem == tag_element.elem;
            })
            .collect();
        self.elems = elems;
    }

    /// TODO: to test
    pub fn rm_dot(&mut self, dot: &Dot) -> bool {
        todo!()
    }

    pub fn join(&mut self, other: &Self) {
        // Intersections and elements not known by other.
        let elems = self
            .elems
            .drain()
            .filter(|tag| {
                let dot = Dot::new(tag.id.clone(), tag.sck, tag.n);
                other.has_element(&tag) || !other.cc.dot_in(&dot)
            })
            .collect();
        self.elems = elems;

        // Elements known by other but not by self
        for tag in other.elems.iter() {
            let dot = Dot::new(tag.id.clone(), tag.sck, tag.n);
            if !self.cc.dot_in(&dot) {
                self.elems.insert(tag.clone());
            }
        }
        self.cc.join(&other.cc);
    }

    // --------------------------
    // UTILS
    // --------------------------

    /// Returns true if the node has ever received information about it, and false otherwise.
    pub fn has_update(&self) -> bool {
        self.cc.is_empty()
    }
}
