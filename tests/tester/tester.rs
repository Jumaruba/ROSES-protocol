use std::collections::{HashMap, HashSet};

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
    disseminate_prob: f64,
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
            disseminate_prob: 0.5,
            oper_prob: 0.3,
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
            while rng.gen_range(0.0..1.0) <= self.oper_prob {
                let element = rng.gen_range(0..10);
                Self::apply_handoff_op(self.clis.get_mut(i).unwrap(), Op::ADD(element));
                self.final_elements
                    .entry(element)
                    .and_modify(|times| *times += 1)
                    .or_insert(1);
                C2T!(OPER, self.clis[i], Op, Op::ADD(element));
            }
        }
    }

    // Chooses a random node to propagate.
    pub fn disseminate(&mut self) {
        self.disseminate_client();
        self.disseminate_server();
    }

    /// Propagates client nodes state given a probability to a random node, which cannot be a client.
    pub fn disseminate_client(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..self.clis.len() {
            while rng.gen_range(0.0..1.0) <= self.disseminate_prob {
                let random_index = rng.gen_range(0..self.servers.len());
                let random_h = self.servers.get_mut(random_index).unwrap();
                C2T!(MERGE, random_h, self.clis[i]);
            }
        }
    }

    pub fn disseminate_server(&mut self) {
        let mut rng = rand::thread_rng();
        let servers_clone = self.servers.clone();
        for i in 0..self.servers.len() {
            while rng.gen_range(0.0..1.0) <= self.disseminate_prob {
                let random_index = rng.gen_range(0..self.servers.len());
                let random_h: &mut Handoff<i32>;
                let id = format!("{}", self.servers[i].id);
                // Propagate to client.
                if rng.gen_range(0..=1) == 1 {
                    random_h = self.clis.get_mut(random_index).unwrap();
                }
                // Propagate to server.
                else {
                    // Server cannot propagate to itself.
                    if random_index == i {
                        return;
                    }
                    random_h = self.servers.get_mut(random_index).unwrap();
                }

                C2T!(MERGE, random_h, servers_clone[i]);
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

    /// Returns true case the states are correct, and false otherwise.
    pub fn verify(&mut self) -> bool {
        for i in 0..self.clis.len(){
            self.activate_peculiarity(i);
        }

        // Sync with user.
        for _ in 0..6 {
            for cli in self.clis.iter_mut() {
                for server in self.servers.iter_mut() {
                    C2T!(MERGE, server, cli);
                    C2T!(MERGE, cli, server);
                }
            }
        }
        // Sync between servers.
        let server_size = self.servers.len();
        for _ in 0..3 {
            for server_2 in 0..server_size {
                let server_clone = self.servers[server_2].clone();
                for server in self.servers.iter_mut() {
                    C2T!(MERGE, server, server_clone);
                }
            }
        }



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
