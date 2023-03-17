use std::collections::{HashMap, HashSet};

use handoff_register::{handoff::Handoff, types::{TagElem, Ck, Dot}};
mod utils;
use utils::id;

#[test]
pub fn transl_1(){
    let mut c2: Handoff<i32> = Handoff::new(id("C"), 1);
    c2.ck.sck = 2;
    c2.ck.dck = 1;
    c2.tokens = HashMap::from([
        ((id("C"), id("S")),
        ((Ck::new(1,1)),
        2,
        HashSet::from([
            TagElem::new(1,2,6),
            TagElem::new(1,1,9)    
        ])))
    ]);
    


    let mut s: Handoff<i32> = Handoff::new(id("S"), 0);
    s.slots = HashMap::from([(id("C"),Ck::new(1,1))]);
    s.transl = HashSet::from([(
        Dot::new(id("C"),1,2), Dot::new(id("S"), 1,2)
    )]);

    s.merge(&c2);
    c2.rm_elem(9);
    c2.merge(&s); 
    s.merge(&c2);

    let te = HashMap::from([(
        id("S"),
        HashSet::from([
            TagElem::new(1,2,6),
        ]))]);

    assert_eq!(s.te, te);
    assert_eq!(c2.te, te);

}

#[test]
pub fn transl_2(){
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

    println!("ADD 6" );
    c2.add_elem(6);
    s.merge(&c2);
    c2.merge(&s); 
    println!("ADD 5" );
    c2.add_elem(5);
    s.merge(&c2);
    c2.merge(&s); 
    println!("ADD 9" );
    c2.add_elem(9);
    s.merge(&c2);
    c2.merge(&s); 
    s.merge(&c2);
    c2.merge(&s);
    s.merge(&c2);
    s.merge(&c2);

    assert_eq!(c2.fetch(), HashSet::from([6,5,9]))

}

#[test]
pub fn transl_3(){
    let mut c2: Handoff<i32> = Handoff::new(id("C"), 1);
    let mut s: Handoff<i32> = Handoff::new(id("S"), 0);

    println!("ADD 5");
    c2.add_elem(5);
    c2.merge(&s);
    s.merge(&c2);   // Create slot. 
    c2.merge(&s);   // Create token.
    s.merge(&c2);   // Fill slot create translation.

    println!("RM 5");
    c2.rm_elem(5);

    // Received elements from other node.
    c2.cc.insert_cc(&Dot::new(id("S"), 1, 1));
    c2.te.insert(id("S"), HashSet::from([
        TagElem::new(1,1,5)
    ]));
    
    // Translation remove the conflict.
    
}