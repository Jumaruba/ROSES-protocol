use crate::types::{Dot, TagItem};
use crate::{DotContext, NodeId};
use core::hash::Hash;
use std::fmt::Debug;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

#[derive(Clone, Debug, PartialEq, Eq)]

/// The kernel is a structure that performs operations of a crdt.
pub struct Kernel<E>
where
    E: Eq + Display + Clone + Hash + Debug,
{
    pub id: NodeId,
    pub ti: HashMap<NodeId, HashSet<TagItem<E>>>, // Tagged Items
    pub cc: DotContext, // Causal Context
}

impl<E> Kernel<E>
where
    E: Eq + Display + Clone + Hash + Debug,
{
    pub fn new(id: &NodeId) -> Self {
        Self {
            id: id.clone(),
            ti: HashMap::new(),
            cc: DotContext::new(),
        }
    }


    pub fn get_ti(&self, id: &NodeId) -> HashSet<TagItem<E>>{
        self.ti.get(id).unwrap_or(&HashSet::new()).clone()
    }

    pub fn add_cc(&mut self, dot: &Dot) {
        self.cc.cc.insert((dot.id.clone(), dot.sck), dot.n);
    }

    pub fn insert_dot_elem(&mut self, dot: Dot, elem: &E) {
        self.cc.insert_dot(&dot, Some(false));
        let tag_e = TagItem::new(dot.sck, dot.n, elem.clone());
        self.ti
            .entry(dot.id)
            .and_modify(|set| {
                set.insert(tag_e.clone());
            })
            .or_insert(HashSet::from([(tag_e)]));
    }

    /// Removes all the entries related to the id.
    /// Cleans both elements and dot context.
    pub fn clean_id(&mut self, id: &NodeId, sck: i64) {
        self.ti.remove(id);
        self.cc.clean_id(id, sck);
    }

    pub fn has_element(&self, id: &NodeId, tag: &TagItem<E>) -> bool {
        self.ti.contains_key(id) && self.ti[id].contains(tag)
    }

    /// Gets elements of the kernel.
    /// TODO: to test
    pub fn elements(&self) -> HashSet<E> {
        let mut res: HashSet<E> = HashSet::new();
        for (_, hash) in self.ti.iter() {
            hash.iter().for_each(|tag_element| {
                res.insert(tag_element.elem.clone());
            });
        }
        res
    }

    /// Adds an element with key equals to self.id and return the added entry.
    /// TODO: to test
    pub fn add(&mut self, elem: E, sck: i64) -> TagItem<E> {
        let dot = self.cc.makedot(&self.id, sck);
        let tag_element = TagItem {
            sck,
            n: dot.n,
            elem,
        };

        self.ti
            .entry(self.id.clone())
            .and_modify(|set| {
                set.insert(tag_element.clone());
            })
            .or_insert(HashSet::from([tag_element.clone()]));

        tag_element
    }



    /// Remove an element from the set of elements.
    /// TODO: to test
    pub fn rm(&mut self, elem: &E) {
        self.ti.iter_mut().for_each(|(_, set)| {
            *set = set
                .drain()
                .filter(|tag_element| {
                    return *elem == tag_element.elem;
                })
                .collect();
        });
    }

    /// TODO: To test
    pub fn join(&mut self, other: &Self) {
        // Intersections and elements not known by other.
        self.ti.iter_mut().for_each(|(id, hash)| {
            *hash = hash
                .drain()
                .filter(|tag| {
                    other.has_element(&id, &tag)
                        || !other.cc.dot_in(&Dot {
                            id: id.clone(),
                            sck: tag.sck,
                            n: tag.n,
                        })
                })
                .collect();
        });

        // Elements known by other but not by self
        for (id, hash) in other.ti.iter() {
            for tag in hash.iter() {
                let dot = Dot {
                    id: id.clone(),
                    sck: tag.sck,
                    n: tag.sck,
                };
                if !self.cc.dot_in(&dot) {
                    self.ti
                        .entry(id.clone())
                        .and_modify(|val| {
                            val.insert(tag.clone());
                        })
                        .or_insert(HashSet::from([tag.clone()]));
                }
            }
        }
        self.cc.join(&other.cc);
    }


    // --------------------------
    // UTILS
    // --------------------------

    /// Returns true if the node has ever received information about it, and false otherwise.
    pub fn has_seen(&self, id: &NodeId, sck: i64) -> bool {
        self.cc.id_in(id, sck)
    }
}
