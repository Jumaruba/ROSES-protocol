use std::collections::{HashMap, HashSet};

use thesis_code::{handoff::Handoff, nodeId::NodeId};

pub fn id(letter: &str) -> NodeId {
    NodeId::new(1, letter.to_string())
}

pub fn val(handoff: &Handoff<i32>) -> String {
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
    assert_eq!(h_2.slots, HashMap::from([(id("A"), (1, 1))]));
}

/// empty token
#[test]
pub fn create_token_1() {
    // Given
    let mut h_1: Handoff<i32> = Handoff::new(id("A"), 1);
    h_1.dck = 1;
    h_1.slots = HashMap::from([(id("B"), (1, 1))]);

    let mut h_2: Handoff<i32> = Handoff::new(id("B"), 0);

    // When
    h_2.create_token(&h_1);
    // Then
    assert_eq!(val(&h_2), "");
}

#[test]
pub fn create_token_2() {
    // Given
    let mut h_1: Handoff<i32> = Handoff::new(id("A"), 1);
    h_1.dck = 1;
    h_1.slots = HashMap::from([(id("B"), (1, 1))]);

    let mut h_2: Handoff<i32> = Handoff::new(id("B"), 0);
    h_2.add(2);
    h_2.add(3);

    let res: HashMap<
        (NodeId, NodeId),
        (
            (i64, i64),
            HashSet<(NodeId, i64, i64)>,
            HashSet<(i64, i64, i32)>,
        ),
    > = HashMap::from([( (id("B"), id("A")), ( (1, 1), HashSet::new(), HashSet::from([(1, 1, 2), (1, 2, 3)])))]);

    // When
    h_2.create_token(&h_1);
    // Then
    assert_eq!(h_2.get_tokens(), res);
}


#[test]
pub fn fill_slots(){
    // Given 
    let mut h_1: Handoff<i32> = Handoff::new(id("A"), 1);
    h_1.dck = 1;
    h_1.slots = HashMap::from([(id("B"), (1, 1))]);

   
}