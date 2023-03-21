use handoff_register::{handoff::Handoff, types::NodeId};
mod parse; 

// Generates the client nodes (aw and handoff)
pub fn gen_clients() -> Vec<Handoff<i32>> {
    let mut c = Vec::new();
    for i in 0..3 {
        let h: Handoff<i32> = Handoff::new(NodeId::new(i, "C".to_string()), 1);
        C2T!(CREATE, h);
        c.push(h);
    }
    c
}

// Generates the servers nodes (aw and handoff)
pub fn gen_servers() -> Vec<Handoff<i32>> {
    let mut s = Vec::new();
    for i in 0..3 {
        let h: Handoff<i32> = Handoff::new(NodeId::new(i, "S".to_string()), 0);
        C2T!(CREATE, h);
        s.push(h);
    }
    s 
}

pub fn test_1() {
    let clis = gen_clients();
    let servers = gen_servers();

    

}