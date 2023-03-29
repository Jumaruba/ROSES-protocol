use std::{
    collections::{HashMap, HashSet},
    sync,
};

use crate::C2T;
use handoff_register::{handoff::Handoff, types::NodeId};

use super::Op;
use rand::Rng;

#[derive(Debug)]
pub struct Tester {
    pub clis: Vec<Handoff<i32>>,
    pub servers: Vec<Handoff<i32>>,
    pub peculiarity: Vec<Option<i32>>,
    pub times_peculiarity: Vec<i32>,
    pub final_elements: HashMap<i32, i32>,
    pub associated_server: Vec<i32>,
    disseminate_prob: f64,
    fail_prob: f64,
    oper_prob: f64,
    n_elements: i64,
}

impl Tester {
    pub fn new() -> Self {
        Self {
            clis: Vec::new(),
            servers: Vec::new(),
            peculiarity: Vec::new(),
            times_peculiarity: Vec::new(), // How many times the peculiarity was activated.
            final_elements: HashMap::new(), // Elements that should be in the final state.
            associated_server: Vec::new(),
            disseminate_prob: 0.3, // Probability to disseminate to another server.
            oper_prob: 0.3,
            fail_prob: 0.5,
            n_elements: 10,
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
        self.set_peculiarity();
        self.associate_server();
    }

    /// Initiliazes a given number of servers.
    fn init_servers(&mut self, n_servers: i64, n_layers: i32) {
        for i in 0..n_layers - 1 {
            for j in 0..n_servers {
                let h: Handoff<i32> = Handoff::new(NodeId::new(j, "S".to_string()), i);
                C2T!(CREATE, h);
                self.servers.push(h);
            }
        }
    }

    /// Initialize a given number of clients.
    fn init_clis(&mut self, n_clis: i64, n_layers: i32) {
        for i in 0..n_clis {
            let h: Handoff<i32> = Handoff::new(NodeId::new(i, "C".to_string()), n_layers - 1);
            C2T!(CREATE, h);
            self.clis.push(h);
        }
    }

    /// Each node has a probability of having a peculiarity.
    /// A peculiarity is a number that is not desired by the node.
    pub fn set_peculiarity(&mut self) {
        let mut rng = rand::thread_rng();
        let mut pec: HashSet<i32> = HashSet::new(); // store peculiarities, to generate unique ones.
        for i in 0..self.clis.len() {
            self.times_peculiarity.push(0);
            self.peculiarity.push(None);
            if rng.gen_bool(0.5) {
                let element: i32 = rng.gen_range(0..self.n_elements).try_into().unwrap();
                if !pec.contains(&element) {
                    pec.insert(element);
                    self.peculiarity[i] = Some(element);
                }
            }
        }
    }

    /// Associates a server to a client.
    pub fn associate_server(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..self.clis.len() {
            let server_index: i32 = rng.gen_range(0..self.servers.len()).try_into().unwrap();
            self.associated_server.push(server_index);
        }
    }

    /// Activates peculiarity if necessary.
    pub fn activate_peculiarity(&mut self, pos: usize) {
        if self.peculiarity[pos].is_some()
            && self.clis[pos]
                .fetch()
                .contains(&self.peculiarity[pos].unwrap())
        {
            Self::apply_handoff_op(
                self.clis.get_mut(pos).unwrap(),
                Op::RM(self.peculiarity[pos].unwrap()),
            );
            self.times_peculiarity[pos] += 1;
        }
    }

    /// Applies operations to clients given a probability.
    pub fn apply_operation(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..self.clis.len() {
            self.activate_peculiarity(i);
            // Many operations can be applied.
            while rng.gen_bool(self.oper_prob) {
                let element = rng.gen_range(0..10);
                Self::apply_handoff_op(self.clis.get_mut(i).unwrap(), Op::ADD(element));
                // Store how many times the element was added.
                self.final_elements
                    .entry(element)
                    .and_modify(|times| *times += 1)
                    .or_insert(1);
            }
        }
    }

    // Chooses a random node to propagate.
    pub fn disseminate(&mut self) {
        self.disseminate_client();
        self.disseminate_server();
    }

    pub fn disseminate_client(&mut self) {
        let mut rng = rand::thread_rng();

        for (i, cli) in self.clis.iter_mut().enumerate() {
            let pos: usize = self.associated_server[i].try_into().unwrap();
            let server: &mut Handoff<i32> = self.servers.get_mut(pos).unwrap();
            C2T!(MERGE, server, cli);
            // Server sends information back if it doesnt fail.
            if !rng.gen_bool(self.fail_prob) {
                C2T!(MERGE, cli, server);
            } else {
                let random_index = rng.gen_range(0..self.servers.len());
                self.associated_server[i] = random_index.try_into().unwrap();
            }
        }
    }

    pub fn disseminate_server(&mut self) {
        let mut rng = rand::thread_rng();
        let servers_size = self.servers.len();
        for i in 0..servers_size {
            while rng.gen_bool(self.disseminate_prob) {
                let servers_clone = self.servers.clone();
                let random_index = rng.gen_range(0..servers_size);
                // Server cannot propagate to itself.
                if random_index == i {
                    return;
                }
                let random_h = &mut self.servers[random_index];
                C2T!(MERGE, random_h, servers_clone[i]);
            }
        }
    }

    /// Applies an operation to the handoff structures in two layers.
    pub fn apply_handoff_op(h: &mut Handoff<i32>, oper: Op<i32>) {
        match oper {
            Op::RM(elem) => {
                h.rm_elem(elem);
                C2T!(OPER, h, Op, Op::RM(elem));
            }
            Op::ADD(elem) => {
                h.add_elem(elem);
                C2T!(OPER, h, Op, Op::ADD(elem));
            }
        }
    }

    pub fn sync_clis_servers(&mut self) {
        for cli in self.clis.iter_mut() {
            for server in self.servers.iter_mut(){
                C2T!(MERGE, server, cli);
                C2T!(MERGE, cli, server);
            }
        }
    }

    pub fn sync_servers(&mut self) {
        // Sync between servers.
        let server_size = self.servers.len();
        for server_2 in 0..server_size {
            let server_clone = self.servers[server_2].clone();
            for (i, server) in self.servers.iter_mut().enumerate() {
                if server_2 != i {
                    C2T!(MERGE, server, server_clone);
                }
            }
        }
    }


    /// Returns true case the states are correct, and false otherwise.
    pub fn verify(&mut self) -> bool {
        for i in 0..4 {
            // send remaining tokens.
            self.sync_clis_servers();

            // Synchronize between servers.
            self.sync_servers();

            self.sync_clis_servers();

            // Activate peculiarities
            for i in 0..self.clis.len() {
                self.activate_peculiarity(i);
            }

            self.sync_clis_servers();

            self.sync_clis_servers();
        }

        // =========================================
        // Check peculiarity activations
        println!("VERIFY PECULIARITY");
        for i in 0..self.peculiarity.len() {
            if let Some(peculiarity) = self.peculiarity[i] {
                let n_activations = self.times_peculiarity[i];
                if n_activations > *self.final_elements.get(&peculiarity).unwrap_or(&0) {
                    return false;
                }
            }
        }
        // Remove peculiar elements from desired list.
        for i in self.peculiarity.iter() {
            if let Some(pec) = i {
                self.final_elements.remove(pec);
            }
        }

        let final_elements: HashSet<i32> = self.final_elements.keys().cloned().collect();
        println!("VERIFY CLIENTS");
        // Check clients
        for cli in self.clis.iter() {
            if cli.fetch() != final_elements {
                println!("==== FAIL === ");
                println!("FINAL {:?}", final_elements);
                println!("CLIENT {:?}", cli.fetch());
                return false;
            }
        }

        // Check servers.
        println!("VERIFY SERVERS");
        for server in self.servers.iter() {
            if server.fetch() != final_elements {
                println!("=== FAIL ====");
                println!("FINAL {:?}", final_elements);
                println!("SERVER {:?}", server.fetch());
                return false;
            }
        }
        true
    }
}
