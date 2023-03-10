use std::collections::{HashMap, HashSet};

use thesis_code::{dotcontext::DotContext, nodeId::NodeId, types::Dot};

/// Creates a NodeId.
pub fn id(id: &str) -> NodeId {
    return NodeId::new(1, id.to_string());
}

pub fn dotcontext_add_cc_vals(dotcontext: &mut DotContext, arr: &[(&str, i64, i64)]) {
    for &(id_, sck, n) in arr.iter() {
        dotcontext.cc.insert((id(id_), sck), n);
    }
}

pub fn dotcontext_add_dots(dotcontext: &mut DotContext, arr: &[(&str, i64, i64)]) {
    for &(id_, sck, n) in arr.iter() {
        dotcontext.dc.insert(Dot::new(id(id_), sck, n));
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

/// Id entry exists
#[test]
pub fn makedot_1() {
    let mut dc1 = get_dotcontext_1();
    let dot = dc1.makedot(&id("A"), 3);
    assert_eq!(
        Dot {
            id: id("A"),
            sck: 3,
            n: 5
        },
        dot
    );
}

/// Id entry does not exists
#[test]
pub fn makedot_2() {
    let mut dc1 = get_dotcontext_1();
    let dot = dc1.makedot(&id("B"), 2);
    assert_eq!(
        Dot {
            id: id("B"),
            sck: 2,
            n: 1
        },
        dot
    );
}


