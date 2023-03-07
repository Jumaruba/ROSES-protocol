use std::collections::{HashMap, HashSet};

use thesis_code::{nodeId::NodeId, dotcontext::DotContext};

pub fn id(id: &str) -> NodeId {
    return NodeId::new(1, id.to_string());
}


pub fn dotcontext_add_cc_vals(dotcontext: &mut DotContext, arr: &[(&str, i64, i64)]) {
    for &(id_, sck, n) in arr.iter() {
        dotcontext
            .cc
            .entry(id(id_))
            .and_modify(|val| {
                val.insert(sck, n);
            })
            .or_insert(HashMap::from([(sck, n)]));
    }
}

pub fn dotcontext_add_dots(dotcontext: &mut DotContext, arr: &[(&str, i64, i64)]) {
    for &(id_, sck, n) in arr.iter() {
        dotcontext
            .dc
            .entry(id(id_))
            .and_modify(|val| {
                val.insert((sck, n));
            })
            .or_insert(HashSet::from([(sck, n)]));
    }
}

pub fn get_dotcontext_1() -> DotContext {
    let mut dotcontext = DotContext::new();
    dotcontext_add_cc_vals(&mut dotcontext, &[("A", 2, 3), ("B", 4, 5), ("A", 3, 4)]);
    dotcontext
}

// Contains dc
pub fn get_dotcontext_2() -> DotContext {
    let mut dotcontext = DotContext::new();
    dotcontext_add_cc_vals(&mut dotcontext, &[("A", 2, 3), ("B", 4, 5), ("A", 3, 4)]);
    dotcontext_add_dots(&mut dotcontext, &[("A", 2, 5), ("A", 2, 7), ("B", 4, 7)]);
    dotcontext
}

// Contains dc
pub fn get_dotcontext_3() -> DotContext {
    let mut dotcontext = DotContext::new();
    dotcontext_add_cc_vals(&mut dotcontext, &[("A", 2, 8), ("B", 4, 5), ("A", 3, 4)]);
    dotcontext_add_dots(&mut dotcontext, &[("A", 2, 7), ("B", 4, 9), ("C", 2, 4)]);
    dotcontext
}


