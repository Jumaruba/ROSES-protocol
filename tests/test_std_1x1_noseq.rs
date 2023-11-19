use std::collections::HashSet;

use handoff_register::{handoff::Handoff, types::NodeId};
use handoff_register::types::Payload;

mod tester;
use tester::utils::{show_blue, show_red};


#[test]
pub fn rm_element() {
    let mut handoff: Handoff<i32> = Handoff::new(NodeId::new(1, "A".to_string()), 1);
    handoff.rm_elem(21);
    println!("{:?}", handoff);
}

/// There are no failures and no removals.
#[test]
pub fn test_add() {
    let mut h1: Handoff<i32> = Handoff::new(NodeId::new(1, "A".to_string()), 1);
    h1.add_elem(22);
    let mut h0: Handoff<i32> = Handoff::new(NodeId::new(1, "B".to_string()), 0);

    h0.merge(&h1); show_blue("CREATE SLOT", &h0);
    h1.merge(&h0); show_blue("CREATE TOKEN", &h1);
    h0.merge(&h1); show_blue("FILL SLOT", &h0);
    h1.merge(&h0); show_blue("DISCARD TOKEN", &h1);
    h0.merge(&h1); show_blue("DISCARD TRANSLATION", &h0);
}

#[test]
pub fn test_rm_1() {
    let mut h1: Handoff<i32> = Handoff::new(NodeId::new(1, "A".to_string()), 1);
    h1.add_elem(22);
    let mut h0: Handoff<i32> = Handoff::new(NodeId::new(1, "B".to_string()), 0);

    h1.rm_elem(22); show_red("RM 22", &h1);
    h0.merge(&h1); show_blue("CREATE SLOT", &h0);
    h1.merge(&h0); show_blue("CREATE TOKEN", &h1);
    h0.merge(&h1); show_blue("FILL SLOT", &h0);
    h1.merge(&h0); show_blue("DISCARD TOKEN", &h1);
    h0.merge(&h1); show_blue("DISCARD TRANSLATION", &h0);


    assert_eq!(HashSet::new(), h1.fetch());
    assert_eq!(HashSet::new(), h0.fetch());

}

#[test]
pub fn test_rm_2() {
    let mut h1: Handoff<i32> = Handoff::new(NodeId::new(1, "A".to_string()), 1);
    h1.add_elem(22);
    let mut h0: Handoff<i32> = Handoff::new(NodeId::new(1, "B".to_string()), 0);

    h0.merge(&h1); show_blue("CREATE SLOT", &h0);
    h1.rm_elem(22); show_red("RM 22", &h1);
    h1.merge(&h0); show_blue("CREATE TOKEN", &h1);
    h0.merge(&h1); show_blue("FILL SLOT", &h0);
    h1.merge(&h0); show_blue("DISCARD TOKEN", &h1);
    h0.merge(&h1); show_blue("DISCARD TRANSLATION", &h0);

    assert_eq!(HashSet::new(), h1.fetch());
    assert_eq!(HashSet::new(), h0.fetch());

}

#[test]
pub fn test_rm_3() {
    let mut h1: Handoff<i32> = Handoff::new(NodeId::new(1, "A".to_string()), 1);
    h1.add_elem(22);
    let mut h0: Handoff<i32> = Handoff::new(NodeId::new(1, "B".to_string()), 0);

    h0.merge(&h1); show_blue("CREATE SLOT", &h0);
    h1.merge(&h0); show_blue("CREATE TOKEN", &h1);
    h1.rm_elem(22); show_red("RM 22", &h1);
    h0.merge(&h1); show_blue("FILL SLOT", &h0);
    h1.merge(&h0); show_blue("DISCARD TOKEN", &h1);
    h0.merge(&h1); show_blue("DISCARD TRANSLATION", &h0);

    assert_eq!(HashSet::new(), h1.fetch());
    assert_eq!(HashSet::new(), h0.fetch());
}

#[test]
pub fn test_rm_4() {
    let mut h1: Handoff<i32> = Handoff::new(NodeId::new(1, "A".to_string()), 1);
    h1.add_elem(22);
    let mut h0: Handoff<i32> = Handoff::new(NodeId::new(1, "B".to_string()), 0);

    h0.merge(&h1); show_blue("CREATE SLOT", &h0);
    h1.merge(&h0); show_blue("CREATE TOKEN", &h1);
    h0.merge(&h1); show_blue("FILL SLOT", &h0);
    h1.rm_elem(22); show_red("RM 22", &h1);
    h1.merge(&h0); show_blue("DISCARD TOKEN", &h1);
    h0.merge(&h1); show_blue("DISCARD TRANSLATION", &h0);

    assert_eq!(HashSet::new(), h1.fetch());
    assert_eq!(HashSet::new(), h0.fetch());

}

#[test]
pub fn test_rm_5() {
    let mut h1: Handoff<i32> = Handoff::new(NodeId::new(1, "A".to_string()), 1);
    h1.add_elem(22);
    let mut h0: Handoff<i32> = Handoff::new(NodeId::new(1, "B".to_string()), 0);

    h0.merge(&h1); show_blue("CREATE SLOT", &h0);
    h1.merge(&h0); show_blue("CREATE TOKEN", &h1);
    h0.merge(&h1); show_blue("FILL SLOT", &h0);
    h1.merge(&h0); show_blue("DISCARD TOKEN", &h1);
    h1.rm_elem(22); show_red("RM 22", &h1);
    h0.merge(&h1); show_blue("DISCARD TRANSLATION", &h0);

    assert_eq!(HashSet::new(), h1.fetch());
    assert_eq!(HashSet::new(), h0.fetch());

}

#[test]
pub fn test_add_1() {
    let mut h1: Handoff<i32> = Handoff::new(NodeId::new(1, "A".to_string()), 1);
    h1.add_elem(22);
    let mut h0: Handoff<i32> = Handoff::new(NodeId::new(1, "B".to_string()), 0);

    h0.merge(&h1); show_blue("CREATE SLOT", &h0);
    h1.merge(&h0); show_blue("CREATE TOKEN", &h1);
    h0.merge(&h1); show_blue("FILL SLOT", &h0);
    h1.add_elem(3); show_red("ADD 3", &h1);
    h1.merge(&h0); show_blue("DISCARD TOKEN", &h1);
    h0.merge(&h1); show_blue("DISCARD TRANSLATION", &h0);

    assert_eq!(HashSet::from([3,22]), h1.fetch());
    assert_eq!(HashSet::from([22]), h0.fetch());

}

pub fn apply_sequence(mut h0: Handoff<i32>, mut h1: Handoff<i32>) -> (Handoff<i32>, Handoff<i32>){

    h0.merge(&h1); show_blue("CREATE SLOT", &h0);
    h1.merge(&h0); show_blue("CREATE TOKEN", &h1);
    h0.merge(&h1); show_blue("FILL SLOT",&h0);
    h1.merge(&h0); show_blue("DISCARD TOKEN", &h1);
    h0.merge(&h1); show_blue("DISCARD TRANSLATION", &h0);

    (h0, h1)
}

// A simple sequential test
#[test]
pub fn test_std_seq_1(){
    // Arrange
    let mut h1: Handoff<i32> = Handoff::new(NodeId::new(1, "A".to_string()), 1);
    let mut h0: Handoff<i32> = Handoff::new(NodeId::new(1, "B".to_string()), 0);
    let mut result = HashSet::new();
    result.insert(2000);
    result.insert(3000);

    // Act
    h1.add_elem(1000);
    let (mut h0, mut h1) = apply_sequence(h0, h1);
    h1.add_elem(2000);
    let (mut h0, mut h1) = apply_sequence(h0, h1);
    h1.add_elem(3000);
    let (mut h0, mut h1) = apply_sequence(h0, h1);
    h1.rm_elem(1000);
    let (mut h0, mut h1) = apply_sequence(h0, h1);


    // Assert
    assert_eq!(h1.fetch(), result);
    assert_eq!(h0.fetch(), result);
}

#[test]
pub fn test_translation(){
    // Arrange
    let client_node_id = NodeId::new(1, "A".to_string());
    let mut h1: Handoff<i32> = Handoff::new(client_node_id.clone(), 1);
    let mut h0: Handoff<i32> = Handoff::new(NodeId::new(1, "B".to_string()), 0);
    let mut result = HashSet::new();
    result.insert(1000);

    // Act
    h1.add_elem(1000);
    let (mut h0, mut h1) = apply_sequence(h0, h1);


    let binding = HashSet::new();
    let a1_elems = h1.payload.get(&client_node_id).unwrap_or(&binding);
    assert_eq!(a1_elems.clone().is_empty(), true);

    // Assert
    assert_eq!(h1.fetch(), result);
    assert_eq!(h0.fetch(), result);
}





