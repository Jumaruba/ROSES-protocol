use crdt_sample::AworsetOpt;
use handoff_register::handoff::Handoff;
use handoff_register::types::NodeId;
use rand::Rng;
use rand::rngs::ThreadRng;
mod utils;

use utils::{apply_aworset_op, apply_handoff_op, gen_rnd_opers, HandoffWrapper};

macro_rules! n_server_nodes { () => { 3 }; }
macro_rules! n_tests { () => { 100 }; }
macro_rules! n_oper { () => { 10 }; } // Each has this number of operations to perform
macro_rules! prop_server { () => { 0 }; }
macro_rules! num_elements { () => { 10 }; }

pub fn gen_cli() -> HandoffWrapper {
    let h: Handoff<i32> = Handoff::new(NodeId::new(1, "C".to_string()), 1);
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
    for i in 0..n_server_nodes!() {
        let h: Handoff<i32> = Handoff::new(NodeId::new(i, "S".to_string()), 0);
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
            println!("{:?} {}", oper, h.h);
            println!("{:?}", oper);
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
            if propagate_server(&mut rng) {
                s_h.merge(&other_s_h);
                println!("MERGE WITH SERVER {}", s_h);
            } else {
                if cli.state % 2 == 1 {
                    s_h.merge(&cli.h.clone());
                    println!("SERVER {}", s_h);
                } else {
                    cli.h.merge(&s_h);
                    println!("CLI {}", cli.h); 
                }
            }
        } else {
            break;
        }
    }



    // Converge client
    for i in 0..n_vec_server{
        let server = vec_server.get(i).unwrap();
        cli.h.merge(server);
    }


    println!("FINAL");
    // Converge servers
    for i in 0..n_vec_server{
        for j in 0..n_vec_server {
            // Update handoff
            let other = &vec_server[j].clone();
            let to_update = vec_server.get_mut(i).unwrap();
            to_update.merge(&other);
        }
        println!("server {}", vec_server[i]);
    }
    return (vec_server,cli.h, cli_aw);
}

#[test]
pub fn test(){
    for _ in 0..n_tests!(){
        println!("NEW TEST");
        let (vec_server, cli, cli_aw) = main();
        if cli_aw.elements() != cli.fetch(){
            println!("{}", cli);
            println!("{:?}", cli_aw);
        }
        assert_eq!(cli_aw.elements(), cli.fetch());
        for i in 0..vec_server.len() {
            assert_eq!(cli_aw.elements(), vec_server[i].fetch());
        }
        println!("{}", vec_server[0]);

    }
}
