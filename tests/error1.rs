use std::collections::{HashMap, HashSet};

use handoff_register::{handoff::Handoff, types::{TagElem, Ck, Dot}};
mod utils;
use utils::id;

#[test]
pub fn state(){
    let mut c2: Handoff<i32> = Handoff::new(id("C"), 1);
    c2.ck.sck = 2;
    c2.ck.dck = 1;
    c2.te = HashMap::from([(
        id("S"),
        HashSet::from([
            TagElem::new(1,3,7),
            TagElem::new(1,1,3)
        ]))]);

    c2.cc.cc = HashMap::from([((id("S"),1),3)]);
   
    c2.tokens = HashMap::from([
        ((id("C"), id("S")),
        ((Ck::new(2,6)),
        1,
        HashSet::from([TagElem::new(2,1,7)])))
    ]);

    let mut s: Handoff<i32> = Handoff::new(id("S"), 0);
    s.ck.sck = 1; 
    s.ck.dck = 7;
    s.te = HashMap::from([(
        id("S"),
        HashSet::from([
            TagElem::new(1,1,3),
            TagElem::new(1,3,7),
            TagElem::new(1,5,4),
            TagElem::new(1,4,9),
        ]))]);

    s.cc.cc = HashMap::from([((id("S"),1),5)]);

    s.merge(&c2);
    println!("{}",s);
    c2.merge(&s); 
    println!("{}",c2);

}