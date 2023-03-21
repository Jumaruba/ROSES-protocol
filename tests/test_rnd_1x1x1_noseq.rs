
use std::collections::HashSet;

use crdt_sample::AworsetOpt;
use handoff_register::{handoff::Handoff, types::NodeId};
mod utils; 
use rand::Rng;
use utils::{id, gen_rnd_opers, Op, apply_handoff_op, apply_aworset_op};
mod parse; 

macro_rules! n_oper {() => {10}} // Each has this number of operations to perform
macro_rules! n_tests { () => {10} }

pub fn new_operation(cli: &mut Handoff<i32>, opers: &mut Vec<Op<i32>>,aworset: &mut AworsetOpt<i32>) {
    let mut rng = rand::thread_rng(); 
    let apply_oper = rng.gen_range(0..20); 
    if apply_oper > 14  {
        apply_handoff_op(cli, opers.first().unwrap().clone());
        C2T!(OPER, cli, Op, opers.first().unwrap().clone());
        apply_aworset_op(aworset, opers.first().unwrap().clone());
        opers.remove(0);
    } 
}

pub fn propagate(cli: &mut Handoff<i32> , server_s: &mut Handoff<i32>, server_t: &mut Handoff<i32>) { 
    let mut rng = rand::thread_rng(); 
    // Client propagates
    if rng.gen_range(0..10) <= 5 {
        C2T!(MERGE, server_s, cli, false);
    } else {
        // Server sends state. 
        C2T!(MERGE, cli, server_s, false);
    }

    // Server S propagates. 
    if rng.gen_range(0..10) <= 5 {
        C2T!(MERGE, server_t, server_s, false);
    } else {
        // Server t propagates. 
        C2T!(MERGE, server_s, server_t, false);
    }

}

pub fn sync(cli: &mut Handoff<i32> , server_s: &mut Handoff<i32>, server_t: &mut Handoff<i32>){

    C2T!(START_SYNC, cli, server_t);
    for _ in 0..5{
        C2T!(MERGE, server_s, cli, false);
        C2T!(MERGE, cli, server_s, false);
    }

    for _ in 0..5{
        C2T!(MERGE, server_t, server_s, false);
        C2T!(MERGE, server_s, server_t, false);
    }
    
    C2T!(END_SYNC);
}

pub fn test() -> (HashSet<i32> , HashSet<i32>, HashSet<i32>, HashSet<i32>) {
    let mut cli: Handoff<i32> = Handoff::new(id("C"), 2);
    let mut server_s: Handoff<i32> = Handoff::new(NodeId::new(1, "S".to_string()), 1);
    let mut server_t: Handoff<i32> = Handoff::new(NodeId::new(0, "S".to_string()), 0);
    C2T!(CREATE, cli);
    C2T!(CREATE, server_s);
    C2T!(CREATE, server_t);

    let mut opers: Vec<Op<i32>>  = gen_rnd_opers(1,10, n_oper!());   // Operations the client will apply. 
    let mut aworset: AworsetOpt<i32> = AworsetOpt::new(crdt_sample::NodeId::new(1, "AW".to_string()));

    while !opers.is_empty() {
        new_operation(&mut cli, &mut opers,  &mut aworset);
        propagate(&mut cli, &mut server_s, &mut server_t);
    }

    sync(&mut cli, &mut server_s, &mut server_t);
    return (aworset.elements(), server_s.fetch(), server_t.fetch(), cli.fetch());


}
#[test]
pub fn test_rnd_1x1x1_noseq(){
    for i in 0..n_tests!(){
        C2T!(BEGIN);
        println!("======== TEST {} ========", i);
        let res = test();
        C2T!(END);
        assert_eq!(res.0, res.1);
        assert_eq!(res.0, res.2);
        assert_eq!(res.0, res.3);
    } 
}