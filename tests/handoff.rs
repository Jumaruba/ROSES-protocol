use thesis_code::{handoff::Handoff, nodeId::NodeId};

#[test]
pub fn rm_element(){
    let mut handoff: Handoff<i32> = Handoff::new(NodeId::new(1, "A".to_string()),1);
    handoff.rm(21);
    println!("{:?}", handoff);
}

#[test]
pub fn fill_slots() {
    let mut h1: Handoff<i32> = Handoff::new(NodeId::new(1, "A".to_string()), 1);
    h1.add(22);
    let mut h0: Handoff<i32> = Handoff::new(NodeId::new(1, "B".to_string()), 0);

    // Create slot
    h0.merge(&h1);
    println!("CREATE SLOT: {}", h0);
    // Create token
    h1.merge(&h0);
    println!("CREATE TOKEN: {}", h1);
    // Fill slot and create translation
    h0.merge(&h1);
    println!("FILL SLOT: {}", h0);
    // Discard token
    h1.merge(&h0);
    println!("DISCARD TOKEN: {}", h1);
    // Discard translation
    h0.merge(&h1);  
    println!("DISCARD TRANSLATION: {}", h0);
}