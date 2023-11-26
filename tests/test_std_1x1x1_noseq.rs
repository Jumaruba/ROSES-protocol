mod tester;
use tester::utils::{id};
use handoff_register::handoff::Handoff;
use std::collections::HashSet;

#[test]
pub fn test1() {
    let mut c2: Handoff<i32> = Handoff::new(id("C", 2), 2);
    let mut s1: Handoff<i32> = Handoff::new(id("S", 1), 1);
    let mut s0: Handoff<i32> = Handoff::new(id("S", 0), 0);

    // Set the initial state.
    c2.add_elem(7);
    s1.merge(&c2);  // Create slot.
    c2.merge(&s1);  // Create token.
    s1.merge(&c2);  // Fill the slot and create translation.
    c2.merge(&s1);  // Translate its elements.

    println!("{:?}", c2);

    s0.merge(&s1);  // Create slot.
    s1.merge(&s0);  // Create token.
    s0.merge(&s1);  // Fill slot and create translation.

    // Start procedures.
    c2.rm_elem(7);
    assert_eq!(c2.fetch(), HashSet::new());
    println!("C2: {:?}", c2);
    s1.merge(&c2);  // S1 should remove (7,7)
    assert_eq!(s1.fetch(), HashSet::new());
    s0.merge(&s1);  // Nothing happens
    s1.merge(&s0);  // Translate.
    println!("{:?}", s1);
    println!("{:?}", s0);
    s0.merge(&s1);  // 7 should be removed from s0.

    assert_eq!(s0.fetch(), HashSet::new());
    println!("{:?}", s0);
}
