use crdt_sample::AworsetOpt;
use handoff_register::handoff::Handoff;
use handoff_register::types::NodeId;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashSet;
mod utils;
use utils::Op;
use utils::Op::{ADD, RM};
use utils::{apply_aworset_op, apply_handoff_op, id, HandoffWrapper, gen_rnd_opers};


macro_rules! n_client_nodes { () => {10}}
macro_rules! n_tests { () => {100} }
macro_rules! n_oper {() => {3}} // Each has this number of operations to perform



pub fn gen_cli_node() -> Vec<HandoffWrapper>{
    let mut res = Vec::new();
    for i in 0..n_client_nodes!() {
        let h: Handoff<i32> = Handoff::new(NodeId::new(i, "C".to_string()), 1);
        let wrapper = HandoffWrapper {
            h,
            opers : Vec::new(),
            curr_oper: 0,
            state: 3, 
            receive: true
        };
        res.push(wrapper);
    }
    res
}

pub fn add_opers(vh: &mut Vec<HandoffWrapper>) {
    for h in vh.iter_mut(){ 
        let mut opers = gen_rnd_opers(1, 10, n_oper!());
        h.opers.append(&mut opers);
    }
}

#[test]
pub fn main(){
    let mut cli = gen_cli_node();
    let mut end : Vec<HandoffWrapper> = Vec::new();
    add_opers(&mut cli);
    let mut server: Handoff<i32> = Handoff::new(NodeId::new(0, "S".to_string()), 0);
    let mut rng = rand::thread_rng();
    while !cli.is_empty(){
        let index = rng.gen_range(0..cli.len());
        let rnd_h = &mut cli[index];
        if rnd_h.prepare_merge() {
            if rnd_h.state % 2 == 0{
                server.join(&rnd_h.h.clone());
            }else {
                rnd_h.h.join(&server);
            }
        } else {
            end.push(cli.remove(index));
        }
    }

    assert_eq!(format!("{:?}", end), "");
    //let res = format!("{}", cli);
    //assert_eq!("", res);
}
