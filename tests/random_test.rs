// Tests the structure against a common aworset.
use crate::Op::{ADD, RM};
use crdt_sample::{aworset_opt, AworsetOpt};
use rand::seq::SliceRandom;
use rand::Rng;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use thesis_code::handoff::Handoff;
use thesis_code::nodeId::NodeId;

pub fn id(id: &str) -> NodeId {
    return NodeId::new(1, id.to_string());
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Op<E: Eq + Clone + Hash + Debug + Display> {
    RM(E),
    ADD(E),
}

// Generates a random operation (ADD(elem) or RM(elem)). Elem is a random element.
pub fn get_rand_oper() -> Op<i32> {
    let mut rng = rand::thread_rng();
    let element = rng.gen_range(0..20);
    let oper = vec![ADD(element), RM(element)];
    oper.choose(&mut rng).unwrap().clone()
}

/// Generates a random vector of operations, which can be applied in any register based crdt.
pub fn gen_rnd_opers() -> Vec<Op<i32>> {
    let mut rng = rand::thread_rng();
    let n_operations = 2; //rng.gen_range(0..100);
    let mut operations = Vec::new();

    for _ in 0..n_operations {
        operations.push(get_rand_oper());
    }
    operations
}

/// Applies an operation to aworset.
pub fn apply_aworset_oper(aworset_opt: &mut AworsetOpt<i32>, oper: Op<i32>) {
    match oper {
        RM(elem) => {
            aworset_opt.rm(elem);
        }
        ADD(elem) => {
            aworset_opt.add(elem);
        }
    }
}

/// Applies an operation to the handoff structures in two layers.
pub fn apply_handoff_oper(handoff_t0: &mut Handoff<i32>, handoff_t1: &mut Handoff<i32>, oper: Op<i32>) {
    match oper {
        RM(elem) => {
            handoff_t0.add(elem);
        }
        ADD(elem) => {
            handoff_t0.add(elem);
        }
    }
    handoff_t1.merge(handoff_t0);           // Create slot
    handoff_t0.merge(handoff_t1);           // Create token
    handoff_t1.merge(handoff_t0);           // Fill slot
    handoff_t0.merge(handoff_t1);           // Discard token

}

#[test]
pub fn test() {
    let mut handoff_t0: Handoff<i32> = Handoff::new(id("A"), 1);
    let mut handoff_t1: Handoff<i32> = Handoff::new(id("B"), 1);
    let mut aworset_opt: AworsetOpt<i32> =
        AworsetOpt::new(crdt_sample::NodeId::new(1, "C".to_string()));
    let opers: Vec<Op<i32>> = gen_rnd_opers();
    println!("OPER: {:?}", opers);
    for oper in opers.iter(){
        apply_aworset_oper(&mut aworset_opt, oper.clone());
        apply_handoff_oper(&mut handoff_t0, &mut handoff_t1, oper.clone());
        println!(">> APPLY {:?}=======================", oper);
        println!("> T0:: {:?}\n", handoff_t0);
        println!("> T1:: {:?}\n", handoff_t1);
        println!("> AWORSET{:?}\n", aworset_opt);
    }

    println!("{:?}", gen_rnd_opers());
}
