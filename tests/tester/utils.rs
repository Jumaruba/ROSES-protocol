use color_print::cprintln;
use crdt_sample::AworsetOpt;
use handoff_register::{handoff::Handoff, types::NodeId};
use rand::seq::SliceRandom;
use rand::Rng;
use std::fmt::{Debug};
use super::{op::Op::{ADD, RM}, Op};

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
pub fn gen_rnd_oper(min: i32, max: i32) -> Op<i32> {
    let mut rng = rand::thread_rng();
    let element = rng.gen_range(min..max);
    let oper = vec![ADD(element), RM(element)];
    oper.choose(&mut rng).unwrap().clone()
}



/// Generates a random vector of operations, which can be applied in any register based crdt.
pub fn gen_rnd_opers(min: i32, max: i32, n_oper: i32) -> Vec<Op<i32>> {
    let mut operations = Vec::new();
    
    for _ in 0..n_oper {
        let rnd_oper = gen_rnd_oper(min, max);
        if let RM(n) = rnd_oper {
            if count_ADD(&operations, &n) <= count_RM(&operations, &n) {
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

fn count_ADD(operations: &Vec<Op<i32>>, target: &i32 ) -> usize{
    operations.iter().filter(|&op| *op == ADD(*target)).count()
}
fn count_RM(operations: &Vec<Op<i32>>, target: &i32 ) -> usize{
    operations.iter().filter(|&op| *op == RM(*target)).count()
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

    pub fn update_oper(&mut self){
        if self.state == 0 {
            self.curr_oper += 1;
            self.state = 3;
        } else {
            self.state -= 1;
        }
    }


}
