use handoff_register::{handoff::Handoff, types::NodeId};

#[test]
pub fn add(){
    let mut h: Handoff<i32> = Handoff::new(NodeId::new(1, "S".to_string()), 0);
    h.add_elem(1);
    h.add_elem(1);
    println!("{}", h);
}