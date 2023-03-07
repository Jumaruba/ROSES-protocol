use crate::types::{TagElement, Dot};
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
    pub elems: HashMap<NodeId, HashSet<TagElement<E>>>, 
    cc: DotContext,
}

impl<E> Kernel<E>
where
    E: Eq + Display + Clone + Hash + Debug,
{
    pub fn new(id: &NodeId) -> Self {
        Self {
            id: id.clone(),
            elems: HashMap::new(),
            cc: DotContext::new(),
        }
    }

    // --------------------------
    // STANDARD FUNCTIONS
    // Functions that modifies the structures.
    // --------------------------

    /// Returns the set of a node.
    pub fn get_set(&self, id: &NodeId) -> Option<&HashSet<TagElement<E>>> {
        self.elems.get(id)
    }

    /// Removes all the entries related to the id.
    /// Cleans both elements and dot context.
    /// TODO: To test
    pub fn clean_id(&mut self, id: &NodeId) {
        self.elems.remove(id);
        self.cc.clean_id(id);
    }

    pub fn get_last_tag(&self, sck: i64) -> i64 {
        todo!()
    }

    
    pub fn get_cc(&self) -> HashSet<Dot> {
        self.cc.cc2set(&self.id)
    }
    
    /// TODO: to test
    pub fn has_element(&self, id: &NodeId, tag: &TagElement<E>) -> bool {
        self.elems.contains_key(id) && self.elems[id].contains(tag)
    }

    pub fn dot_in(&self, d: &Dot) -> bool{
        self.cc.dot_in(d)
    }
    // --------------------------
    // OPERATIONS
    // CRDT's core operations. 
    // --------------------------

    /// Gets elements of the kernel.
    /// TODO: to test
    pub fn elements(&self) -> HashSet<E> {
        let mut res: HashSet<E> = HashSet::new();
        for (_, hash) in self.elems.iter() {
            hash.iter().for_each(|tag_element| {
                res.insert(tag_element.elem.clone());
            });
        }
        res
    }

    /// Adds an element with key equals to self.id and return the added entry.
    /// TODO: to test
    pub fn add(&mut self, elem: E, sck: i64) -> TagElement<E> {
        let (_, _, n) = self.cc.makedot(&self.id, sck);
        let tag_element = TagElement {sck, n, elem};

        self.elems
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
        self.elems.iter_mut().for_each(|(_, set)| {
            *set = set
                .drain()
                .filter(|tag_element| {
                    return *elem == tag_element.elem;
                })
                .collect();
        });
    }

    /// TODO: to test
    pub fn rename(&mut self, transl: &(Dot, Dot)){
        self.cc.rename_cc(transl.clone());
        self.rename_elems(transl);
    }

    /// TODO: To test
    pub fn join(&mut self, other: &Self) {
        // Intersections and elements not known by other.
        self.elems.iter_mut().for_each(|(id, hash)| {
            *hash = hash.drain().filter(|tag| {
                other.has_element(&id, &tag) || !other.cc.dot_in(&Dot{id: id.clone(), sck:tag.sck, n:tag.n})
            }).collect();
        });

        // Elements known by other but not by self
        for (id, hash) in other.elems.iter(){
            for tag in hash.iter() {
                let dot = Dot{id: id.clone(), sck: tag.sck, n: tag.sck};
                if !self.cc.dot_in(&dot) {
                    self.elems.entry(id.clone()).and_modify(|val| {
                        val.insert(tag.clone());
                    }).or_insert(HashSet::from([tag.clone()]));
                }
            }
        }
        self.cc.join(&other.cc);
    }

    // --------------------------
    // UTILS
    // --------------------------

    /// Returns true if the node has ever received information about it, and false otherwise.
    pub fn has_seen(&self, id: &NodeId) -> bool {
        self.cc.id_in(id)
    }

    /// TODO : to test and improve
    pub fn rename_elems(&mut self, transl: &(Dot, Dot)){
        let mut to_add: HashSet<(Dot, E)> = HashSet::new();
        self.elems.entry(transl.0.id.clone()).and_modify(|hash| {
            *hash = hash.drain().filter(|entries| {
                if entries.n != transl.0.n && entries.sck != transl.0.sck {
                    to_add.insert((transl.1.clone(), entries.elem.clone()));
                    return true; 
                }
                return false;
            }).collect();
        });
    }
}
