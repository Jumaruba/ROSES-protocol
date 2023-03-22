use std::collections::HashSet;

use handoff_register::{handoff::Handoff, types::NodeId};
mod parse; 
mod utils;
use utils::id;

use crate::utils::Op;

#[test]
pub fn test_std_1x1x1_noseq() {
    C2T!(BEGIN);
    let mut cli: Handoff<i32> = Handoff::new(id("C"), 2);
    let mut server_1: Handoff<i32> = Handoff::new(NodeId::new(1, "S".to_string()), 1);
    let mut server_0: Handoff<i32> = Handoff::new(NodeId::new(0, "S".to_string()), 0);
    C2T!(CREATE, cli);
    C2T!(CREATE, server_1);
    C2T!(CREATE, server_0);

    cli.add_elem(9);
    C2T!(OPER, cli, Op, Op::ADD(9));
    C2T!(MERGE, server_1, cli, false);
    C2T!(MERGE, cli, server_1, false);
    C2T!(MERGE, server_1, cli, false);
    C2T!(MERGE, cli, server_1, false);
    C2T!(MERGE, server_1, cli, false);

    C2T!(MERGE, server_0, server_1, false);
    C2T!(MERGE, server_1, server_0, false);
    C2T!(MERGE, server_0, server_1, false);
    C2T!(MERGE, server_1, server_0, false);
    println!("HERE");
    C2T!(MERGE, cli, server_1, false);
    C2T!(END);
    
    println!("FINAL RESULT ============");
    println!("{}", server_0);
    println!("{}", server_1);
    println!("{}", cli);

    assert_eq!(false, true);
}