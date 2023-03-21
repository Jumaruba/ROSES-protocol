use std::collections::{HashMap, HashSet};

use handoff_register::{
    handoff::Handoff,
    types::{Ck, Dot, NodeId, TagElem},
};
mod utils;
use utils::id;

#[test]
pub fn transl_1() {
    let mut c2: Handoff<i32> = Handoff::new(id("C"), 1);
    c2.ck.sck = 2;
    c2.ck.dck = 1;
    c2.tokens = HashMap::from([(
        (id("C"), id("S")),
        (
            (Ck::new(1, 1)),
            2,
            HashSet::from([TagElem::new(1, 2, 6), TagElem::new(1, 1, 9)]),
        ),
    )]);

    let mut s: Handoff<i32> = Handoff::new(id("S"), 0);
    s.slots = HashMap::from([(id("C"), Ck::new(1, 1))]);
    s.transl = HashSet::from([(Dot::new(id("C"), 1, 2), Dot::new(id("S"), 1, 2))]);

    s.merge(&c2);
    c2.rm_elem(9);
    c2.merge(&s);
    s.merge(&c2);

    let te = HashMap::from([(id("S"), HashSet::from([TagElem::new(1, 2, 6)]))]);

    assert_eq!(s.te, te);
    assert_eq!(c2.te, te);
}

#[test]
pub fn transl_2() {
    let mut c2: Handoff<i32> = Handoff::new(id("C"), 1);
    /*
       CLI:: C1, (3, 1)
       tier: 1
       elems: {S1: {(1, 1, 6)}}
       cc: {(S1, 1): 1}
       dc: {}
       slots: {}
       tokens: {(C1, S1): ((2, 2), 1, {(2, 1, 5)})}
       transl: {}
    */

    let mut c2: Handoff<i32> = Handoff::new(id("C"), 1);
    let mut s: Handoff<i32> = Handoff::new(id("S"), 0);

    println!("ADD 6");
    c2.add_elem(6);
    s.merge(&c2);
    c2.merge(&s);
    println!("ADD 5");
    c2.add_elem(5);
    s.merge(&c2);
    c2.merge(&s);
    println!("ADD 9");
    c2.add_elem(9);
    s.merge(&c2);
    c2.merge(&s);
    s.merge(&c2);
    c2.merge(&s);
    s.merge(&c2);
    s.merge(&c2);

    assert_eq!(c2.fetch(), HashSet::from([6, 5, 9]))
}

// Cache transl and discard translation on the right time.
#[test]
pub fn transl_4() {
    let mut C1: Handoff<i32> = Handoff::new(NodeId::new(1, "C".to_string()), 1);
    let mut S0: Handoff<i32> = Handoff::new(NodeId::new(0, "S".to_string()), 0);
    let mut S1: Handoff<i32> = Handoff::new(NodeId::new(1, "S".to_string()), 0);
    println!("ADD 6");
    C1.add_elem(6);
    println!("{}", C1);
    S0.merge(&C1);
    println!("{}", S0);
    C1.merge(&S1);
    println!("{}", C1);
    S0.merge(&C1);
    println!("{}", S0);
    C1.merge(&S1);
    println!("{}", C1);
    println!("ADD 5");
    C1.add_elem(5);
    println!("{}", C1);
    S1.merge(&C1);
    println!("{}", S1);
    C1.merge(&S0);
    println!("{}", C1);
    S1.merge(&C1);
    println!("{}", S1);
    C1.merge(&S0);
    println!("{}", C1);
    println!("ADD 1");
    C1.add_elem(1);
    println!("{}", C1);
    S0.merge(&C1);
    println!("{}", S0);
    C1.merge(&S1);
    println!("{}", C1);
    S1.merge(&C1);
    println!("{}", S1);
    C1.merge(&S1);
    println!("{}", C1);
    println!("ADD 5");
    C1.add_elem(5);
    println!("{}", C1);
    S0.merge(&C1);
    println!("{}", S0);
    C1.merge(&S1);
    println!("{}", C1);
    S1.merge(&S0);
    println!("{}", S1);
    S0.merge(&S1);
    println!("{}", S0);
    println!("RM 5");
    C1.rm_elem(5);
    println!("{}", C1);
    S0.merge(&C1);
    println!("{}", S0);
    C1.merge(&S0);
    println!("{}", C1);
    S0.merge(&C1);
    println!("{}", S0);
    C1.merge(&S1);
    println!("{}", C1);
    S1.merge(&C1);
    println!("{}", S1);
    C1.merge(&S0);
    println!("{}", C1);
    S1.merge(&C1);
    println!("{}", S1);
    C1.merge(&S1);
    println!("{}", C1);
    S0.merge(&C1);
    println!("{}", S0);
    C1.merge(&S1);
    println!("{}", C1);
    S0.merge(&S1);
    println!("{}", S0);
    println!("SYNC STEP");
    C1.merge(&S0);
    println!("{}", C1);
    C1.merge(&S1);
    println!("{}", C1);
    S0.merge(&S1);
    println!("{}", S0);
    S1.merge(&S0);
    println!("{}", S1);
    assert_eq!(HashSet::from([1, 6]), S1.fetch());
    assert_eq!(HashSet::from([1, 6]), S1.fetch());
    assert_eq!(HashSet::from([1, 6]), S1.fetch());
}

#[test]
pub fn transl_3() {
    let mut C1: Handoff<i32> = Handoff::new(NodeId::new(1, "C".to_string()), 1);
    let mut S0: Handoff<i32> = Handoff::new(NodeId::new(0, "S".to_string()), 0);
    let mut S1: Handoff<i32> = Handoff::new(NodeId::new(1, "S".to_string()), 0);
    let mut S2: Handoff<i32> = Handoff::new(NodeId::new(2, "S".to_string()), 0);
    println!("ADD 7");
    C1.add_elem(7);
    println!("{}", C1);
    S2.merge(&C1);
    println!("{}", S2);
    C1.merge(&S0);
    println!("{}", C1);
    S0.merge(&C1);
    println!("{}", S0);
    C1.merge(&S2);
    println!("{}", C1);
    println!("ADD 5");
    C1.add_elem(5);
    println!("{}", C1);
    S1.merge(&C1);
    println!("{}", S1);
    C1.merge(&S1);
    println!("{}", C1);
    S2.merge(&C1);
    println!("{}", S2);
    C1.merge(&S1);
    println!("{}", C1);
    println!("ADD 2");
    C1.add_elem(2);
    println!("{}", C1);
    S1.merge(&C1);
    println!("{}", S1);
    C1.merge(&S1);
    println!("{}", C1);
    S0.merge(&C1);
    println!("{}", S0);
    C1.merge(&S2);
    println!("{}", C1);
    println!("ADD 3");
    C1.add_elem(3);
    println!("{}", C1);
    S0.merge(&C1);
    println!("{}", S0);
    C1.merge(&S1);
    println!("{}", C1);
    S2.merge(&C1);
    println!("{}", S2);
    C1.merge(&S1);
    println!("{}", C1);
    println!("ADD 6");
    C1.add_elem(6);
    println!("{}", C1);
    S1.merge(&C1);
    println!("{}", S1);
    C1.merge(&S1);
    println!("{}", C1);
    S2.merge(&C1);
    println!("{}", S2);
    C1.merge(&S1);
    println!("{}", C1);
    println!("ADD 4");
    C1.add_elem(4);
    println!("{}", C1);
    S2.merge(&C1);
    println!("{}", S2);
    C1.merge(&S1);
    println!("{}", C1);
    S1.merge(&C1);
    println!("{}", S1);
    C1.merge(&S0);
    println!("{}", C1);
    println!("ADD 8");
    C1.add_elem(8);
    println!("{}", C1);
    S1.merge(&S2);
    println!("{}", S1);
    C1.merge(&S2);
    println!("{}", C1);
    S0.merge(&C1);
    println!("{}", S0);
    C1.merge(&S2);
    println!("{}", C1);
    println!("RM 3");
    C1.rm_elem(3);
    println!("{}", C1);
    S0.merge(&S1);
    println!("{}", S0);
    C1.merge(&S2);
    println!("{}", C1);
    S2.merge(&S0);
    println!("{}", S2);
    C1.merge(&S2);
    println!("{}", C1);
    println!("ADD 6");
    C1.add_elem(6);
    println!("{}", C1);
    S1.merge(&C1);
    println!("{}", S1);
    C1.merge(&S2);
    println!("{}", C1);
    S0.merge(&C1);
    println!("{}", S0);
    C1.merge(&S2);
    println!("{}", C1);
    println!("RM 4");
    C1.rm_elem(4);
    println!("{}", C1);
    S1.merge(&S2);
    println!("{}", S1);
    C1.merge(&S1);
    println!("{}", C1);
    S2.merge(&C1);
    println!("{}", S2);
    C1.merge(&S2);
    println!("{}", C1);
    S1.merge(&C1);
    println!("{}", S1);
    C1.merge(&S0);
    println!("{}", C1);
    S2.merge(&C1);
    println!("{}", S2);
    C1.merge(&S0);
    println!("{}", C1);
    S1.merge(&C1);
    println!("{}", S1);
    C1.merge(&S0);
    println!("{}", C1);
    S0.merge(&C1);
    println!("{}", S0);
    S2.merge(&S0);
    println!("{}", S2);
    S2.merge(&C1);
    println!("{}", S2);
    C1.merge(&S1);
    println!("{}", C1);
    S0.merge(&C1);
    println!("{}", S0);
    println!("SYNC STEP");
    C1.merge(&S0);
    println!("{}", C1);
    C1.merge(&S1);
    println!("{}", C1);
    C1.merge(&S2);
    println!("{}", C1);
    S0.merge(&S1);
    println!("{}", S0);
    S0.merge(&S2);
    println!("{}", S0);
    S1.merge(&S0);
    println!("{}", S1);
    S1.merge(&S2);
    println!("{}", S1);
    S2.merge(&S0);
    println!("{}", S2);
    S2.merge(&S1);
    println!("{}", S2);
    /*

    thread 'test_rnd_1xn_seq' panicked at 'assertion failed: `(left == right)`
      left: `{6, 8, 5, 7, 2}`,
     right: `{2, 8, 6, 5, 7, 3}`', tests/test_rnd_1xn_seq.rs:137:9
     */
    assert_eq!(HashSet::from([6, 8, 5, 7, 2]), C1.fetch());
    assert_eq!(HashSet::from([6, 8, 5, 7, 2]), S1.fetch());
    assert_eq!(HashSet::from([6, 8, 5, 7, 2]), S2.fetch());
    assert_eq!(HashSet::from([6, 8, 5, 7, 2]), S0.fetch());
}
