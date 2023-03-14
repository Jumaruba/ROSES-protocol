use color_print::cprintln;
use crdt_sample::AworsetOpt;
use handoff_register::{handoff::Handoff, types::NodeId};
use rand::seq::SliceRandom;
use rand::Rng;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use Op::{ADD, RM};

// DISPLAY ================================================
pub fn show_blue(oper: &str, h: &Handoff<i32>) {
    cprintln!("<blue,bold>[{}]</> {}", oper, h);
}

pub fn show_red(oper: &str, h: &Handoff<i32>) {
    cprintln!("<red,bold>[{}]</> {}", oper, h);
}

pub fn id(id: &str) -> NodeId {
    return NodeId::new(1, id.to_string());
}

// RANDOM TESTS ===========================================
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Op<E: Eq + Clone + Hash + Debug + Display> {
    RM(E),
    ADD(E),
}

/// Applies an operation to aworset.
pub fn apply_aworset_op(aworset_opt: &mut AworsetOpt<i32>, oper: Op<i32>) {
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
pub fn apply_handoff_op(handoff_t1: &mut Handoff<i32>, oper: Op<i32>) {
    match oper {
        RM(elem) => {
            handoff_t1.rm_elem(elem);
        }
        ADD(elem) => {
            handoff_t1.add_elem(elem);
        }
    }
}

// Generates a random operation (ADD(elem) or RM(elem)). Elem is a random element.
pub fn get_rnd_oper(min: i32, max: i32) -> Op<i32> {
    let mut rng = rand::thread_rng();
    let element = rng.gen_range(min..max);
    let oper = vec![ADD(element), RM(element)];
    oper.choose(&mut rng).unwrap().clone()
}



/// Generates a random vector of operations, which can be applied in any register based crdt.
pub fn gen_rnd_opers(min: i32, max: i32, n_oper: i32) -> Vec<Op<i32>> {
    let mut operations = Vec::new();
    
    for _ in 0..n_oper {
        let rnd_oper = get_rnd_oper(min, max);
        if let RM(n) = rnd_oper {
            if !operations.contains(&ADD(n)) {
                operations.push(ADD(n));
            } else {
                operations.push(rnd_oper);
            }
        }else {
            operations.push(rnd_oper);
        }
    }
    operations
}

#[derive(Clone, Debug)]
pub struct HandoffWrapper {
    pub h: Handoff<i32>,
    pub opers: Vec<Op<i32>>,
    pub curr_oper: usize,
    pub state: i32,
}

impl HandoffWrapper {
    pub fn can_consume(&self) -> bool {
        self.curr_oper < self.opers.len()
    }   

    fn update_oper(&mut self){
        if self.state == 0 {
            self.curr_oper += 1;
            self.state = 3;
        } else {
            self.state -= 1;
        }
    }

    pub fn prepare_merge(&mut self) -> (bool, Option<Op<i32>>) {
        self.update_oper();
        if self.can_consume() {
            if self.state == 3 {
                let oper =  self.opers[self.curr_oper].clone();
                apply_handoff_op(&mut self.h, oper.clone());
                println!("### {:?}, {}", oper, self.h);
                return (true, None);
            }
            if self.state == 1 {
                let oper =  self.opers[self.curr_oper].clone();
                return (true, Some(oper));
            }
            return (true, None);    // Return h so it can be merged. 
        }
        return (false, None);    // Cannot be merged 
    }
}