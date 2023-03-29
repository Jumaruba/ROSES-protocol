use crdt_sample::AworsetOpt;
use handoff_register::handoff::Handoff;
use handoff_register::types::NodeId;
use rand::Rng;
use rand::rngs::ThreadRng;
mod tester;
use tester::utils::{apply_aworset_op, apply_handoff_op, gen_rnd_opers, HandoffWrapper};
use tester::Op;


macro_rules! n_servers { () => { 100 }; }
macro_rules! n_tests { () => { 1 }; }
macro_rules! n_oper { () => { 10 }; } // Each has this number of operations to perform
macro_rules! prop_server { () => { 0 }; }
macro_rules! num_elements { () => { 10 }; }

pub fn gen_cli() -> HandoffWrapper {
    let mut h: Handoff<i32> = Handoff::new(NodeId::new(1, "C".to_string()), 1);
    C2T!(CREATE, h);
    HandoffWrapper {
        h,
        opers: Vec::new(),
        curr_oper: 0,
        state: 4,
    }
}

pub fn gen_cli_aw() -> AworsetOpt<i32>{
    AworsetOpt::new(crdt_sample::NodeId::new(1, "C".to_string()))

}

pub fn gen_servers() -> Vec<Handoff<i32>>{
    let mut res: Vec<Handoff<i32>> = Vec::new();
    for i in 0..n_servers!() {
        let h: Handoff<i32> = Handoff::new(NodeId::new(i, "S".to_string()), 0);
        C2T!(CREATE, h);
        res.push(h);
    }
    res
}

pub fn add_opers(h: &mut HandoffWrapper) {
    let mut opers = gen_rnd_opers(1, num_elements!(), n_oper!());
    h.opers.append(&mut opers);
}

pub fn prepare_merge(h: &mut HandoffWrapper, aw: &mut AworsetOpt<i32>) -> bool {
    h.update_oper();
    if h.can_consume() {
        if h.state == 3 {
            let oper = h.opers[h.curr_oper].clone();
            apply_handoff_op(&mut h.h, oper.clone());
            apply_aworset_op(aw, oper.clone());
            C2T!(OPER, h.h, Op, oper);
            return true;
        }
        return true; // Return h so it can be merged.
    }
    return h.state != 0 || h.h.te.get(&h.h.id).is_some() || !h.h.tokens.is_empty(); // Cannot be merged
}


/// Tells if server propagates to client or another server given probability.
pub fn propagate_server(rng: &mut ThreadRng) -> bool {
    return rng.gen_range(0..10) <= prop_server!();
}

pub fn main() -> (Vec<Handoff<i32>>, Handoff<i32>, AworsetOpt<i32>){

    let mut cli = gen_cli();
    let mut cli_aw = gen_cli_aw();
    add_opers(&mut cli);

    let mut vec_server = gen_servers();
    let n_vec_server = vec_server.len(); 

    let mut rng = rand::thread_rng();

    loop {
        let other_index = rng.gen_range(0..n_vec_server);
        let other_s_h = vec_server[other_index].clone();

        // choose random server.
        let index = rng.gen_range(0..n_vec_server);
        let s_h = &mut vec_server[index];

        if prepare_merge(&mut cli, &mut cli_aw) {
            if propagate_server(&mut rng) && other_index != index {
                C2T!(MERGE, s_h, other_s_h);
            } else {
                if cli.state % 2 == 1 {
                    C2T!(MERGE, s_h, cli.h.clone());
                } else {
                    C2T!(MERGE, cli.h, s_h.clone());
                }
            }
        } else {
            break;
        }
    }

    C2T!(START_SYNC, cli.h, vec_server.get(vec_server.len()-1).unwrap());

    // Converge client  
    for i in 0..n_vec_server{
        let server = vec_server.get_mut(i).unwrap();
        C2T!(MERGE, server, cli.h); 
    }


    // Converge servers
    for i in 0..n_vec_server{
        for j in 0..n_vec_server {
            if j != i{
                // Update handoff
                let other = &vec_server[j].clone();
                let to_update = vec_server.get_mut(i).unwrap();
                to_update.merge(&other);
                C2T!(MERGE, to_update, other);
            }
        }
    }

    C2T!(END_SYNC);
    return (vec_server,cli.h, cli_aw);
}

#[test]
pub fn test_rnd_1xn_seq(){
    for i in 0..n_tests!(){
        C2T!(BEGIN);
        println!("======== TEST {} ========", i);
        let (vec_server, cli, cli_aw) = main();
        C2T!(END);
        assert_eq!(cli_aw.elements(), cli.fetch());
        for i in 0..vec_server.len() {
            assert_eq!(cli_aw.elements(), vec_server[i].fetch());
        }

    }
}
