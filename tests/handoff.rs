use std::collections::{HashMap, HashSet};

use thesis_code::{handoff::Handoff, nodeId::NodeId};

pub fn id(letter: &str) -> NodeId{
    NodeId::new(1, letter.to_string())
}

pub fn val(handoff: &Handoff<i32>) -> String{
    return format!("{:?}", handoff);
}

#[test]
pub fn create_slot() {
    // Given 
    let mut h_1: Handoff<i32> = Handoff::new(id("A"), 1);
    h_1.add(2);
    let mut h_2: Handoff<i32> = Handoff::new(id("B"), 0);

    // When 
    h_2.create_slot(&h_1);

    // Then
    assert_eq!(h_2.dck, 2);
    assert_eq!(h_2.slots, HashMap::from([(id("A"), (1,1))]));
}
