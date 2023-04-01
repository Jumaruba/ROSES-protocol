// TODO 

use crdt_sample::AworsetOpt;
use handoff_register::handoff::Handoff;
use handoff_register::types::NodeId;
use rand::Rng;
use std::collections::HashSet;
mod tester;
use tester::Op;
use tester::utils::{apply_aworset_op, apply_handoff_op, gen_rnd_opers, id, HandoffWrapper};

macro_rules! n_client_nodes {
    () => {
        10
    };
}
macro_rules! n_tests {
    () => {
        100
    };
}
macro_rules! n_oper {
    () => {
        10
    };
} // Each has this number of operations to perform

macro_rules! num_elements {
    () => {
        10
    };
}

pub fn gen_cli_node() -> Vec<HandoffWrapper> {
    let mut res = Vec::new();
    for i in 0..n_client_nodes!() {
        let h: Handoff<i32> = Handoff::new(NodeId::new(i, "C".to_string()), 1);
        let wrapper = HandoffWrapper {
            h,
            opers: Vec::new(),
            curr_oper: 0,
            state: 4,
        };
        res.push(wrapper);
    }
    res
}

pub fn gen_aw_cli_node() -> Vec<AworsetOpt<i32>> {
    let mut res = Vec::new();
    for i in 0..n_client_nodes!() {
        let h: AworsetOpt<i32> = AworsetOpt::new(crdt_sample::NodeId::new(i, "C".to_string()));
        res.push(h);
    }
    res
}

pub fn add_opers(vh: &mut Vec<HandoffWrapper>) {
    for h in vh.iter_mut() {
        let mut opers = gen_rnd_opers(1, num_elements!(), n_oper!());
        h.opers.append(&mut opers);
    }
}

pub fn prepare_merge(h: &mut HandoffWrapper, aw: &mut AworsetOpt<i32>) -> (bool, Option<Op<i32>>) {
    h.update_oper();
    if h.can_consume() {
        if h.state == 3 {
            let oper = h.opers[h.curr_oper].clone();
            apply_handoff_op(&mut h.h, oper.clone());
            apply_aworset_op(aw, oper.clone());
            return (true, None);
        }
        if h.state == 1 {
            let oper = h.opers[h.curr_oper].clone();
            return (true, Some(oper));
        }
        return (true, None); // Return h so it can be merged.
    }
    return (false, None); // Cannot be merged
}

pub fn main() -> (HashSet<i32>, HashSet<i32>){
    let mut vec_cli = gen_cli_node();
    let mut vec_aw_cli = gen_aw_cli_node();

    add_opers(&mut vec_cli);

    let mut server: Handoff<i32> = Handoff::new(NodeId::new(0, "S".to_string()), 0);
    let mut server_aw: AworsetOpt<i32> = AworsetOpt::new(crdt_sample::NodeId::new(1, "A".to_string()));

    let mut rng = rand::thread_rng();
    let mut opers: Vec<Op<i32>> = Vec::new();

    while !vec_cli.is_empty() {
        let index = rng.gen_range(0..vec_cli.len());
        let rnd_h = &mut vec_cli[index];
        let rnd_aw = &mut vec_aw_cli[index];

        if let (true, op) = prepare_merge(rnd_h, rnd_aw) {
            if rnd_h.state % 2 == 1 {
                let elems = server.te.clone();
                server.merge(&rnd_h.h.clone());
                let new_elems = server.te.clone();
                if elems != new_elems{
                    server_aw.join(&rnd_aw);
                }
            } else {
                rnd_h.h.merge(&server);
                rnd_aw.join(&server_aw);
            }
            if let Some(op) = op {
                opers.push(op.clone());
            }
        } else {
            vec_cli.remove(index);
            vec_aw_cli.remove(index);
        }
    }

    let server_elems = server.fetch();
    let elems = server_aw.elements();
    return (elems, server_elems);
}

#[test]
pub fn test(){
    for _ in 0..n_tests!(){
        let (elems, server_elems) = main();
        assert_eq!(elems, server_elems);
    }
}
