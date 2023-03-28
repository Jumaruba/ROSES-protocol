use std::collections::HashSet;
use std::vec;

use crdt_sample::AworsetOpt;
use handoff_register::handoff::Handoff;
use handoff_register::types::NodeId;
use rand::seq::SliceRandom;
use rand::Rng;

macro_rules! n_clis {
    () => {
        1
    };
}
macro_rules! n_servers {
    () => {
        1
    };
}
macro_rules! n_layers {
    () => {
        2
    };
}
macro_rules! prob_transmite {
    () => {
        6
    };
} // should be in the range [0,10]. It is better to be even.

macro_rules! n_elements {
    () => {
        3
    };
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Merge(String, String),
    Add(String, i32),
    Rm(String, i32),
}

fn gen_rand(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

/// Initialize a given number of clients.
fn init_clis() -> (Vec<Handoff<i32>>, Vec<AworsetOpt<i32>>) {
    let mut clis = Vec::new();
    let mut aw_clis = Vec::new();
    for i in 0..n_clis!() {
        let h: Handoff<i32> = Handoff::new(NodeId::new(i, "C".to_string()), n_layers!() - 1);
        let aw: AworsetOpt<i32> = AworsetOpt::new(crdt_sample::NodeId::new(i, "C".to_string()));
        clis.push(h);
        aw_clis.push(aw);
    }
    (clis, aw_clis)
}

/// Initiliazes a given number of servers.
fn init_servers() -> (Vec<Handoff<i32>>, Vec<AworsetOpt<i32>>) {
    let mut servers = Vec::new();
    let mut aw_servers = Vec::new();
    for i in 0..n_layers!() - 1 {
        for j in 0..n_servers!() {
            let h: Handoff<i32> = Handoff::new(NodeId::new(j, "S".to_string()), i);
            let aw: AworsetOpt<i32> =
                AworsetOpt::new(crdt_sample::NodeId::new(i.into(), "S".to_string()));
            servers.push(h);
            aw_servers.push(aw);
        }
    }
    (servers, aw_servers)
}

pub fn gen_operations(
    n_operations: i64,
    id_clis: Vec<String>,
    id_servers: Vec<String>,
) -> Vec<Operation> {
    let mut random = rand::thread_rng();
    let mut res = Vec::new();
    for _ in 0..n_operations {
        let choice = gen_rand(0, 10);
        if choice <= prob_transmite!() {
            let mut first = id_servers.choose(&mut random).unwrap().clone();
            let mut sec = id_clis.choose(&mut random).unwrap().clone();
            if gen_rand(0, 1) == 1 {
                (first, sec) = (sec, first);
            }
            res.push(Operation::Merge(first, sec));
        } else if choice > prob_transmite!()
            && choice <= prob_transmite!() + (10 - prob_transmite!()) / 2
        {
            res.push(Operation::Add(
                id_clis.choose(&mut random).unwrap().clone(),
                gen_rand(0, n_elements!()),
            ));
        } else {
            res.push(Operation::Rm(
                id_clis.choose(&mut random).unwrap().clone(),
                gen_rand(0, n_elements!()),
            ));
        }
    }
    res
}

// PROCESS CLI ==========================
pub fn process_cli(operations: &mut Vec<Operation>, cli: String) {
}

pub fn remove_consecutive_repeated(operations: &mut Vec<Operation>) {
    let mut i = 1;
    while i < operations.len() {
        if operations[i] == operations[i - 1] {
            operations.remove(i);
        } else {
            i += 1;
        }
    }
}

#[test]
pub fn test() {
    let (clis, aw_clis) = init_clis();
    let (servers, aw_servers) = init_servers();
    let id_clis: Vec<String> = clis.iter().map(|h| format!("{}", h.id)).collect();
    let id_servers: Vec<String> = servers.iter().map(|h| format!("{}", h.id)).collect();
    let mut operations = gen_operations(10, id_clis, id_servers);
    println!("{:?}", operations);
    remove_consecutive_repeated(&mut operations);
    process_cli(&mut operations, "C0".to_string());
    println!("{:?}", operations);
}
