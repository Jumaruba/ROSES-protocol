// Tests the structure against a common aworset.
use crate::Op::{ADD, RM};
use crdt_sample::{AworsetOpt};
use rand::seq::SliceRandom;
use rand::Rng;
use thesis_code::types::NodeId;
use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use thesis_code::handoff::Handoff;

pub fn id(id: &str) -> NodeId {
    return NodeId::new(1, id.to_string());
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Op<E: Eq + Clone + Hash + Debug + Display> {
    RM(E),
    ADD(E),
}

// Generates a random operation (ADD(elem) or RM(elem)). Elem is a random element.
pub fn get_rand_oper(min: i32, max: i32) -> Op<i32> {
    let mut rng = rand::thread_rng();
    let element = rng.gen_range(min..max);
    let oper = vec![ADD(element), RM(element)];
    oper.choose(&mut rng).unwrap().clone()
}

/// Generates a random vector of operations, which can be applied in any register based crdt.
pub fn gen_rnd_opers(min: i32, max: i32, n_oper: i32) -> Vec<Op<i32>> {
    let mut operations = Vec::new();

    for _ in 0..n_oper {
        operations.push(get_rand_oper(min, max));
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
pub fn apply_handoff_oper(
    handoff_t0: &mut Handoff<i32>,
    handoff_t1: &mut Handoff<i32>,
    oper: Op<i32>,
    debug: bool,
) {
    match oper {
        RM(elem) => {
            handoff_t1.rm(elem);
        }
        ADD(elem) => {
            handoff_t1.add_elem(elem);
        }
    }
    handoff_t0.merge(handoff_t1); // Create slot
    if debug {
        dbg!("CREATE SLOT, t0:: {}", &handoff_t0);
    }
    handoff_t1.merge(handoff_t0); // Create token
    if debug {
        dbg!("CREATE TOKEN, t1:: {}", &handoff_t1);
    }
    handoff_t0.merge(handoff_t1); // Fill slot
    if debug {
        dbg!("CREATE FILL SLOT, t0:: {}", &handoff_t0);
    }
    handoff_t1.merge(handoff_t0); // Discard token
    if debug {
        dbg!("DISCARD TOKEN, t1:: {}", &handoff_t1);
    }
}

pub fn test(min: i32, max: i32, n_oper: i32, debug: bool) -> (HashSet<i32>, HashSet<i32>, HashSet<i32>) {
    let mut handoff_t0: Handoff<i32> = Handoff::new(id("A"), 0);
    let mut handoff_t1: Handoff<i32> = Handoff::new(id("B"), 1);
    let mut aworset_opt: AworsetOpt<i32> =
        AworsetOpt::new(crdt_sample::NodeId::new(1, "C".to_string()));
    let opers: Vec<Op<i32>> = gen_rnd_opers(min, max, n_oper);
    if debug == true {
        dbg!("OPER: {}", &opers);
    }
    for oper in opers.iter() {
        apply_aworset_oper(&mut aworset_opt, oper.clone());
        apply_handoff_oper(&mut handoff_t0, &mut handoff_t1, oper.clone(), debug);
        if debug == true {
            dbg!(">> APPLY {}=======================", &oper);
            dbg!("> T0:: {}\n", &handoff_t0);
            dbg!("> T1:: {}\n", &handoff_t1);
            dbg!("> AWORSET{}\n", &aworset_opt);
        }
    }

    let elems_aworset = aworset_opt.elements();
    let elems_h1 = handoff_t1.fetch();
    let elems_h0 = handoff_t0.fetch();

    (elems_aworset, elems_h0, elems_h1)
}

#[test]
pub fn multiple_tests() {
    let debug = false;
    for i in 0..100 {
        let (aworset, h0, h1) = test(0, 10, i, debug);
        assert_eq!(aworset, h0);
        assert_eq!(aworset, h1);
        let (aworset, h0, h1) = test(0, 100, i, debug);
        assert_eq!(aworset, h0);
        assert_eq!(aworset, h1);

    }
}
