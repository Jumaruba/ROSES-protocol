use std::collections::{HashMap, HashSet};

use handoff_register::{
    handoff::Handoff,
    types::{NodeId, TagElem, Ck},
};
mod parse;
mod utils;
use utils::id;

use crate::utils::Op;

/// Case 1: replace the S1 elements in C0
#[test]
pub fn test_std_1x1x1_noseq_1() {
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
    C2T!(MERGE, cli, server_1, false);
    C2T!(END);

    assert_eq!(
        HashMap::from([(server_0.id.clone(), HashSet::from([(TagElem::new(1, 1, 9))]))]),
        cli.te
    );

    assert_eq!(HashMap::from([
        (server_0.id.clone(), (1,1)),
        (cli.id.clone(), (2,0)),
        (server_1.id.clone(), (2,0))]),cli.cc.cc)
}


/// Case 2: replace the S1 elements in C0
#[test]
pub fn test_std_1x1x1_noseq_2() {
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

    cli.rm_elem(9);
    C2T!(OPER, cli, Op, Op::RM(9));
    C2T!(MERGE, server_0, server_1, false);
    C2T!(MERGE, server_1, server_0, false);
    C2T!(MERGE, server_0, server_1, false);
    C2T!(MERGE, server_1, server_0, false);
    C2T!(MERGE, cli, server_1, false);
    C2T!(END);

    /*assert_eq!(
        HashMap::from([(server_0.id.clone(), HashSet::from([(TagElem::new(1, 1, 9))]))]),
        cli.te
    );

    assert_eq!(HashMap::from([
        (server_0.id.clone(), (1,1)),
        (cli.id.clone(), (2,0)),
        (server_1.id.clone(), (2,0))]),cli.cc.cc)*/

    assert_eq!(false, true);
}
