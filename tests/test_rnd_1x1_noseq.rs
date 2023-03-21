
use std::collections::HashSet;

use crdt_sample::AworsetOpt;
use handoff_register::handoff::Handoff;
mod utils; 
use rand::Rng;
use utils::{id, gen_rnd_opers, Op, apply_handoff_op, apply_aworset_op};
mod parse;

macro_rules! n_oper {() => {10}} // Each has this number of operations to perform
macro_rules! n_tests { () => {1000} }

pub fn new_operation(cli: &mut Handoff<i32>, opers: &mut Vec<Op<i32>>, curr_state: &mut i32, aworset: &mut AworsetOpt<i32>) {
    let mut rng = rand::thread_rng(); 
    let apply_oper = rng.gen_range(0..20); 
    if apply_oper > 14 || *curr_state == 4 {
        apply_handoff_op(cli, opers.first().unwrap().clone());
        apply_aworset_op(aworset, opers.first().unwrap().clone());
        C2T!(OPER, cli, Op, opers[0]);
        opers.remove(0);
    } 
}

pub fn apply_step(cli: &mut Handoff<i32> , server: &mut Handoff<i32>, curr_step: &mut i32) { 
    // Send to server 
    if *curr_step % 2 == 0 {
        C2T!(MERGE, server, cli, false);
        *curr_step -=1; 
    } // Receive from server 
    else {
        C2T!(MERGE, cli, server, false);
        *curr_step-= 1; 
    }

}

pub fn test() -> (HashSet<i32> , HashSet<i32>, HashSet<i32>) {
    let mut cli: Handoff<i32> = Handoff::new(id("C"), 1);
    let mut server: Handoff<i32> = Handoff::new(id("S"), 0);
    let mut opers: Vec<Op<i32>>  = gen_rnd_opers(1,10, n_oper!());
    C2T!(CREATE, cli);
    C2T!(CREATE, server);
    let mut aworset: AworsetOpt<i32> = AworsetOpt::new(crdt_sample::NodeId::new(1, "AW".to_string()));
    let mut curr_step = 4 ;  

    while !opers.is_empty() {
        new_operation(&mut cli, &mut opers, &mut curr_step, &mut aworset);

        apply_step(&mut cli, &mut server, &mut curr_step);
        if curr_step == 0 {
            curr_step = 4;
        }
    }
    while curr_step >= 0 {
        apply_step(&mut cli, &mut server, &mut curr_step);
    }
    return (aworset.elements(), server.fetch(), cli.fetch());


}
#[test]
pub fn test_rnd_1x1_noseq(){
    for _ in 0..n_tests!(){
        C2T!(BEGIN);
        let res = test();
        C2T!(END);
        assert_eq!(res.0, res.1);
        assert_eq!(res.1, res.2);
    } 
}