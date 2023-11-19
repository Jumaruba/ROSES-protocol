use handoff_register::handoff::Handoff;
use handoff_register::types::NodeId;
use std::collections::HashSet;
use rand::{distributions::Alphanumeric, Rng};

/// These are the function that are not related to the protocol.
#[test]
pub fn test_add_elem(){
    // Arrange
    let node_id = NodeId::new(1, "A".to_string());
    let mut handoff = Handoff::new(node_id, 0);

    let mut expect = HashSet::new();
    expect.insert("A".to_string());

    // Act
    handoff.add_elem("A".to_string());

    // Assert
    let elems = handoff.fetch();
    std::assert_eq!(elems, expect);
}

#[test]
pub fn test_add_elem_random(){
    // Arrange
    let number_of_strings = 10;
    let mut expect = HashSet::new();
    for _ in 0..number_of_strings {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        expect.insert(s);
    }

    let node_id = NodeId::new(0, "A".to_string());
    let mut handoff = Handoff::new(node_id, 0);

    expect.iter().for_each(|elem| {
        handoff.add_elem(elem.clone());
    });

    assert_eq!(handoff.fetch(), expect);
}

#[test]
pub fn test_rm_elem(){
    // Arrange
    let node_id = NodeId::new(1, "A".to_string());
    let mut handoff = Handoff::new(node_id, 0);

    let mut expect = HashSet::new();

    // Act
    handoff.add_elem("A".to_string());
    handoff.add_elem("A".to_string());
    handoff.rm_elem("A".to_string());
    // Assert
    let elems = handoff.fetch();
    std::assert_eq!(elems, expect);
}