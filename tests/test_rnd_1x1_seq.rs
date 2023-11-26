use crdt_sample::AworsetOpt;
use handoff_register::handoff::Handoff;
use std::collections::HashSet;

mod tester;
use tester::Op;
use tester::utils::{apply_aworset_op, apply_handoff_op, id, gen_rnd_opers};

pub fn handoff_protocol(handoff_t0: &mut Handoff<i32>, handoff_t1: &mut Handoff<i32>) {
    handoff_t0.merge(handoff_t1); // Create slot
    handoff_t1.merge(handoff_t0); // Create token
    handoff_t0.merge(handoff_t1); // Fill slot
    handoff_t1.merge(handoff_t0); // Discard token
}

pub fn test(min: i32, max: i32, n_oper: i32) -> (HashSet<i32>, HashSet<i32>, HashSet<i32>) {
    // Declare crdts
    let mut handoff_t0: Handoff<i32> = Handoff::new(id("S", 0), 0);
    let mut handoff_t1: Handoff<i32> = Handoff::new(id("C", 1), 1);
    let mut aworset_opt: AworsetOpt<i32> =
        AworsetOpt::new(crdt_sample::NodeId::new(1, "C".to_string()));
    let opers: Vec<Op<i32>> = gen_rnd_opers(min, max, n_oper);


    // Apply operations
    for oper in opers.iter() {
        apply_aworset_op(&mut aworset_opt, oper.clone());
        apply_handoff_op(&mut handoff_t1, oper.clone());
        handoff_protocol(&mut handoff_t0, &mut handoff_t1);
    }

    // Return results
    let elems_aworset = aworset_opt.elements();
    let elems_h1 = handoff_t1.fetch();
    let elems_h0 = handoff_t0.fetch();

    (elems_aworset, elems_h0, elems_h1)
}

#[test]
pub fn multiple_tests() {
    for i in 0..2000 {
        let (aworset, h0, h1) = test(0, 10, i);
        assert_eq!(aworset, h0);
        assert_eq!(aworset, h1);
    }
}
