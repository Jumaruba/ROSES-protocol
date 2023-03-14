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


macro_rules! n_client_nodes { () => {3}}
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
            state: 4, 
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
    let mut vec_cli = gen_cli_node();
    add_opers(&mut vec_cli);
    let mut end : Vec<HandoffWrapper> = Vec::new();
    
    let mut server: Handoff<i32> = Handoff::new(NodeId::new(0, "S".to_string()), 0);

    let mut rng = rand::thread_rng();
    let mut opers: Vec<Op<i32>> = Vec::new();

    while !vec_cli.is_empty(){
        let index = rng.gen_range(0..vec_cli.len());
        let rnd_h = &mut vec_cli[index];

        if let (true, op) =  rnd_h.prepare_merge() {
            println!("======== STATE {} ========", rnd_h.state);
            if rnd_h.state % 2 == 1{
                server.merge(&rnd_h.h.clone());
                println!(">>> {}", rnd_h.h.clone());
                println!(">>> {}", server);
            }else {
                rnd_h.h.merge(&server);
                println!(">>> {}", rnd_h.h.clone());
                println!(">>> {}", server);
            }
            if  let Some(op) = op{
                opers.push(op.clone());
            }
        } else {
            end.push(vec_cli.remove(index));
        }

    }

    let server_elems = server.fetch();
    let mut aworset = AworsetOpt::new(crdt_sample::NodeId::new(1, "A".to_string()));
    for i in opers.iter() {
        apply_aworset_op(&mut aworset, i.clone());
    }
    let elems = aworset.elements();
    println!("{:?}", opers);
    println!("{:?}", end); 
    println!("{:?}", server); 
    assert_eq!(elems, server_elems);
    //let res = format!("{}", cli);
    //assert_eq!("", res);
}
