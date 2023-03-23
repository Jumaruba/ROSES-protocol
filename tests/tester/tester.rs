use handoff_register::{handoff::Handoff, types::NodeId};
use crate::C2T;

use super::{Wrapper, Op};
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug)]
pub struct Tester {
    pub clis: Vec<Handoff<i32>>,
    pub servers: Vec<Handoff<i32>>,
    pub aw_clis: Vec<Wrapper>,
    pub aw_server: Vec<Wrapper>,
    disseminate_prob: f64,
    oper_prob: f64,
}

impl Tester {
    pub fn new() -> Self {
        Self {
            clis: Vec::new(),
            servers: Vec::new(),
            aw_clis: Vec::new(),
            aw_server: Vec::new(),
            disseminate_prob: 0.5,
            oper_prob: 0.3,
        }
    }

    /// Set the probability of a node to disseminate its state.
    pub fn set_disseminate_prob(&mut self, prob: f64) {
        self.disseminate_prob = prob;
    }

    /// Set the probability to a node apply an operation.
    pub fn set_oper_prob(&mut self, prob: f64) {
        self.oper_prob = prob;
    }

    /// Initializes the nodes of the network (i.e. servers and clients).
    pub fn init(&mut self, n_clis: i64, n_servers: i64, n_layers: i32) {
        self.init_clis(n_clis, n_layers);
        self.init_servers(n_servers, n_layers);
    }

    /// Initiliazes a given number of servers.
    fn init_servers(&mut self, n_servers: i64, n_layers: i32) {
        for i in 0..n_layers - 1 {
            for j in 0..n_servers {
                let h: Handoff<i32> = Handoff::new(NodeId::new(j, "S".to_string()), i);
                let aw = Wrapper::new(crdt_sample::NodeId::new(j, "S".to_string()));
                C2T!(CREATE, h);
                self.servers.push(h);
                self.aw_server.push(aw);
            }
        }
    }

    /// Initialize a given number of clients.
    fn init_clis(&mut self, n_clis: i64, n_layers: i32) {
        for i in 0..n_clis {
            let h: Handoff<i32> = Handoff::new(NodeId::new(i, "C".to_string()), n_layers - 1);
            let aw = Wrapper::new(crdt_sample::NodeId::new(i, "C".to_string()));
            self.clis.push(h);
            self.aw_clis.push(aw);
        }
    }

    /// Applies operations to clients given a probability.
    pub fn apply_operation(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..self.clis.len() {
            // Many operations can be applied.
            while rng.gen_range(0.0..1.0) <= self.oper_prob {
                let oper = Self::gen_rnd_oper();
                Self::apply_handoff_op(self.clis.get_mut(i).unwrap(), oper.clone());
                self.aw_clis.get_mut(i).unwrap().apply_oper(oper.clone());
                C2T!(OPER, self.clis[i], Op, oper);
            }
        }
    }

    // Chooses a random node to propagate. 
    pub fn disseminate(&mut self){
        self.disseminate_client();
        self.disseminate_server();
    }

    /// Propagates client nodes state given a probability to a random node, which cannot be a client.
    pub fn disseminate_client(&mut self){
        let mut rng = rand::thread_rng();
        for i in 0..self.clis.len(){
            while rng.gen_range(0.0..1.0) <= self.disseminate_prob {
                let random_index = rng.gen_range(0..self.servers.len());
                let random_h = self.servers.get_mut(random_index).unwrap();
                let random_aw = self.aw_server.get_mut(random_index).unwrap();
                C2T!(MERGE, random_h, self.clis[i]);
                random_aw.join(&self.aw_clis[i]);
            }
        }
    }

    pub fn disseminate_server(&mut self){
        let mut rng = rand::thread_rng();
        let servers_clone = self.servers.clone();
        let aw_servers_clone = self.aw_server.clone();
        for i in 0..self.servers.len(){
            while rng.gen_range(0.0..1.0) <= self.disseminate_prob {
                // Propagate to server by default.
                let random_index = rng.gen_range(0..self.servers.len());
                let mut random_h: &mut Handoff<i32>;
                let mut random_aw: &mut Wrapper;

                // Propagate to client. 
                if rng.gen_range(0..=1) == 1 {
                    random_h = self.clis.get_mut(random_index).unwrap();
                    random_aw = self.aw_clis.get_mut(random_index).unwrap();
                } 
                // Propagate to server. 
                else {
                    random_h = self.servers.get_mut(random_index).unwrap();
                    random_aw = self.aw_server.get_mut(random_index).unwrap();
                    // Server cannot propagate to itself.
                    if random_index == i{
                        return;
                    }
                }
                
                // Propagate 
                let tokens = random_h.tokens.clone();
                C2T!(MERGE, random_h, servers_clone[i]);
                random_aw.join(&aw_servers_clone[i]);
                // Prepare aworset
                if tokens != random_h.tokens {
                    random_aw.prepare_dispatch();
                }
                println!("{:?}", random_aw.clone());

            }
        }
    }

    /// Applies an operation to the handoff structures in two layers.
    pub fn apply_handoff_op(h: &mut Handoff<i32>, oper: Op<i32>) {
        match oper {
            Op::RM(elem) => {
                h.rm_elem(elem);
            }
            Op::ADD(elem) => {
                h.add_elem(elem);
            }
        }
    }

    // Generates a random operation (ADD(elem) or RM(elem)). Elem is a random element.
    pub fn gen_rnd_oper() -> Op<i32> {
        let mut rng = rand::thread_rng();
        let element = rng.gen_range(0..10);
        let oper = vec![Op::ADD(element), Op::RM(element)];
        oper.choose(&mut rng).unwrap().clone()
    }

    pub fn verify(&self) -> bool {
        for i in 0..self.clis.len() {
            let cli_fetch = self.clis[i].fetch();
            let aw_cli_fetch = self.aw_clis[i].fetch();
            if  cli_fetch != aw_cli_fetch {
                println!("CLI {} : {:?} x {:?}", i, cli_fetch, aw_cli_fetch);
                return false; 
            } 
        }

        for i in 0..self.aw_server.len(){
            let server_fetch = self.servers[i].fetch();
            let aw_server_fetch = self.aw_server[i].fetch();
            if  server_fetch != aw_server_fetch {
                println!("SERVER {} : {:?} x {:?}", i, server_fetch, aw_server_fetch);
                return false; 
            }
        }
        return true;
    }
}
