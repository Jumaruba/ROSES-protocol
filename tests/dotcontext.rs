use std::collections::{HashMap, HashSet};

use thesis_code::{dotcontext::DotContext, nodeId::NodeId, types::Dot};

/// Creates a NodeId.
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

#[test]
pub fn get_cc_1() {
    let dc1 = get_dotcontext_1();
    let curr = dc1.cc2set(&id("A"));
    let res = HashSet::from([
        Dot {
            id: id("A"),
            sck: 3,
            n: 4,
        },
        Dot {
            id: id("A"),
            sck: 2,
            n: 3,
        },
    ]);
    assert_eq!(res, curr);
}

#[test]
pub fn get_cc_2() {
    let dc1 = get_dotcontext_1();
    let curr = dc1.cc2set(&id("B"));
    let res = HashSet::from([Dot {
        id: id("B"),
        sck: 4,
        n: 5,
    }]);
    assert_eq!(res, curr);
}

#[test]
pub fn rm_cc_1() {
    let mut dc2 = get_dotcontext_2();
    dc2.rm_cc(&Dot {
        id: id("A"),
        sck: 2,
        n: 3,
    });
    let res = HashMap::from([
        (id("A"), HashMap::from([(3, 4)])),
        (id("B"), HashMap::from([(4, 5)])),
    ]);
    assert_eq!(res, dc2.cc);
}

#[test]
pub fn rm_cc_2() {
    let mut dc2 = get_dotcontext_2();
    dc2.rm_cc(&Dot {
        id: id("B"),
        sck: 4,
        n: 5,
    });

    let res = HashMap::from([
        (id("A"), HashMap::from([(2, 3), (3,4)])),
    ]);

    assert_eq!(res, dc2.cc);
}
