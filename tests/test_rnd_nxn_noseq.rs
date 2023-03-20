// This test has a certain number of interactions. 

use std::collections::HashSet;

use crdt_sample::{AworsetOpt, Aworset};
use handoff_register::{handoff::Handoff, types::NodeId};
mod utils;
use utils::{apply_aworset_op, apply_handoff_op, gen_rnd_oper, HandoffWrapper, Op};
use rand::Rng;
mod parse; 

macro_rules! n_tests { () => { 1 }; }
macro_rules! n_clients { () => {2}; }
macro_rules! n_servers { () => {2}; }
macro_rules! num_elements { () => { 10 }; }
macro_rules! n_interactions {() => { 5 };}

// Generates the client nodes (aw and handoff)
pub fn gen_clients() -> (Vec<Handoff<i32>>, Vec<AworsetOpt<i32>>) {
    let mut c = Vec::new();
    let mut c_aw = Vec::new();
    for i in 0..n_clients!() {
        let h: Handoff<i32> = Handoff::new(NodeId::new(i, "C".to_string()), 1);
        let aw: AworsetOpt<i32> = AworsetOpt::new(crdt_sample::NodeId::new(i, "C".to_string()));

        C2T!(CREATE, h);
        c.push(h);
        c_aw.push(aw);
    }
    (c, c_aw)
}

// Generates the servers nodes (aw and handoff)
pub fn gen_servers() -> (Vec<Handoff<i32>>, Vec<AworsetOpt<i32>>) {
    let mut s = Vec::new();
    let mut s_aw = Vec::new();
    for i in 0..n_servers!() {
        let h: Handoff<i32> = Handoff::new(NodeId::new(i, "S".to_string()), 0);

        let aw: AworsetOpt<i32> = AworsetOpt::new(crdt_sample::NodeId::new(i, "S".to_string()));

        C2T!(CREATE, h);
        s.push(h);
        s_aw.push(aw);
    }
    (s, s_aw)
}

// Apply updates in clients.
pub fn update(c: &mut Vec<Handoff<i32>>, c_aw: &mut Vec<AworsetOpt<i32>>, to_send: Vec<HashSet<i32>>){
    let mut rng = rand::thread_rng();
    // Update client
    for i in 0..n_clients!() {
        // 50% propabability of update.
        while rng.gen_range(0..10) <= 5 {
            let oper = gen_rnd_oper(0, num_elements!());
            apply_handoff_op(c.get_mut(i).unwrap(), oper.clone());
            apply_aworset_op(c_aw.get_mut(i).unwrap(), oper.clone());
            C2T!(OPER, c[i], Op, oper);
            // Update to_send vector
            if let Op::RM(val) = oper{
                to_send.get_mut(i).unwrap().remove(&val);
            } else if let Op::ADD(val) = oper {
                to_send.get_mut(i).unwrap().insert(val);
            }
        }
    }
}

pub fn propagate(c: &mut Vec<Handoff<i32>>, c_aw: &mut  Vec<AworsetOpt<i32>>, s: &mut Vec<Handoff<i32>>, s_aw: &mut Vec<AworsetOpt<i32>>, to_send: Vec<HashSet<i32>>){
    propagate_server(c, c_aw, s, s_aw);
    propagate_client(c, c_aw, s, s_aw, to_send);
}

pub fn propagate_server(c: &mut Vec<Handoff<i32>>, c_aw: &mut  Vec<AworsetOpt<i32>>, s: &mut Vec<Handoff<i32>>, s_aw: &mut Vec<AworsetOpt<i32>>){
    let mut rng = rand::thread_rng();
    // Chooses a random server or a random client to send its state.
    for i in 0..n_servers!(){
        // 50% of chance of propagating to a client.
        if rng.gen_range(0..10) <= 5 {
            // Propagate to client.
            let cli_index = rng.gen_range(0..n_clients!());
            let src_server = s[i].clone();
            let trg_cli = c.get_mut(cli_index).unwrap();
            C2T!(MERGE, trg_cli, src_server, false);
            c_aw.get_mut(cli_index).unwrap().join(&s_aw[i]);
        } else {
            // Propagate to vector.
            let ser_index = rng.gen_range(0..n_clients!());
            if ser_index == i {
                continue;
            }
            let src_server = s[i].clone();
            let trg_server = s.get_mut(ser_index).unwrap();
            C2T!(MERGE, trg_server, src_server, false);
            let src_server_aw = s_aw[i].clone();
            s_aw.get_mut(ser_index).unwrap().join(&src_server_aw);
        }
        
    }
}

pub fn propagate_client(c: &mut Vec<Handoff<i32>>, c_aw: &mut  Vec<AworsetOpt<i32>>, s: &mut Vec<Handoff<i32>>, s_aw: &mut Vec<AworsetOpt<i32>>, to_send: Vec<HashSet<i32>>){
    let mut rng = rand::thread_rng();
    // Chooses a random server or a random client to send its state.
    for i in 0..n_clients!(){
        // Propagate to server.
        let ser_index = rng.gen_range(0..n_servers!());
        let prev_te = s[ser_index].te.clone(); 
        let trg_server = s.get_mut(ser_index).unwrap();
        C2T!(MERGE, trg_server , c[i], false);
        if prev_te != s[ser_index].te {
            for i in to_send[ser_index]{
                s_aw.get_mut(ser_index).unwrap();
            }
        }
    }
}

fn sync(c: &mut Vec<Handoff<i32>>, s: &mut Vec<Handoff<i32>>) {
    C2T!(START_SYNC, c.get(n_clients!()-1).unwrap(), s.get(n_servers!()-1).unwrap());

    // Converge clients with servers 
    for i in 0..n_clients!(){
        for j in 0..n_servers!() {
            let client = c.get_mut(i).unwrap();
            C2T!(MERGE, client , s[j], false); 
        }
    }
    // Converge server with clients
    for i in 0..n_servers!(){
        for j in 0..n_clients!() {
            let server = s.get_mut(i).unwrap();
            C2T!(MERGE, server, c[j], false); 
        }
    }


    // Converge servers
    for i in 0..n_servers!(){
        for j in 0..n_servers!() {
            if j != i{
                // Update handoff
                let other = &s[j].clone();
                let to_update = s.get_mut(i).unwrap();
                to_update.merge(&other);
                C2T!(MERGE, to_update, other, false);
            }
        }
    }

    C2T!(END_SYNC);
}

fn run() -> (Vec<Handoff<i32>>, Vec<AworsetOpt<i32>>, Vec<Handoff<i32>>, Vec<AworsetOpt<i32>>) {
    let (mut c, mut c_aw) = gen_clients();
    let (mut s, mut s_aw) = gen_servers();

    // Elements to send to the servers. 
    let to_send : Vec<HashSet<i32>> = Vec::new();
    for _ in 0..n_clients!(){
        to_send.push(HashSet::new());
    }

    for _ in 0..n_interactions!() {
        update(&mut c, &mut c_aw);
        propagate(&mut c, &mut c_aw, &mut s, &mut s_aw, to_send);
    }

    // SYNC STEP 
    sync(&mut c, &mut s);
    (c, c_aw, s, s_aw)
}

#[test]
fn test_rnd_nxn_noseq(){
    for _ in 0..n_tests!(){
        C2T!(BEGIN);
        let (c, c_aw, s, s_aw) = run();
        C2T!(END);
        for i in 0..n_clients!() {
            println!("{:?}\n {:?}", c_aw[i], c[i]);
            assert_eq!(c_aw[i].elements(), c[i].fetch());
            println!("===================");
        }
        for i in 0..n_servers!(){
            println!("{:?}\n {:?}", s_aw[i], s[i]);
            assert_eq!(s_aw[i].elements(), s[i].fetch());
            println!("===================");
        }
    }

}
