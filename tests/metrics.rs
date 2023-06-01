mod tester;
use std::fs::File;
use std::io::Write;
use crdt_sample::{NodeId, AworsetOpt};
use handoff_register::handoff::Handoff;
use tester::Tester; 
use rand::prelude::*; 
use indicatif::ProgressBar;
use std::mem::size_of_val;

fn apply_random_op_handoff(handoff: &mut Handoff<i32>, rng: &mut ThreadRng){
    let n: i32 = rng.gen_range(0..100);
    if rng.gen_bool(0.4) {
        handoff.add_elem(n);
    } else {
        handoff.rm_elem(n);
    }
}

fn apply_random_op_AworsetOpt(aworsetOpt: &mut AworsetOpt<i32>, rng: &mut ThreadRng){
    let n: i32 = rng.gen_range(0..100);
    if rng.gen_bool(0.4) {
        aworsetOpt.add(n);
    } else {
        aworsetOpt.rm(n);
    }
}


#[test]
fn metrics_handoff() -> std::io::Result<()>{
    // SETUP 
    let NTESTS = 10; 
    const TOTAL_TIME: i32 = 30;       
    const NOPER_TIME: i32 = 20;     // Time when it stops making operations. 
    const RESP_PROB: f64 = 0.7; 
    const MAX_OPER: i32 = 10;       // Maximum number of operations per time. 
    let n_clis: i64 = 30; 
    let n_servers: i64 = 10; 

    let mut rng: ThreadRng = rand::thread_rng(); 
    let bar = ProgressBar::new((TOTAL_TIME * NTESTS).try_into().unwrap()); 
    // CC size 
    let mut h_vec_state_size: [f64; TOTAL_TIME as usize] = [0.0; TOTAL_TIME as usize]; 
    let mut vec_state_size : [f64; TOTAL_TIME as usize] = [0.0; TOTAL_TIME as usize]; 
    /// Create file to save info;
    let mut file = File::create("metrics.csv").unwrap(); 

    for _ in 0..NTESTS {
        /// Init structures 
        let mut handoff_clis = Vec::new(); 
        let mut handoff_servers =  Vec::new(); 
        let mut aworset = Vec::new(); 

        for i in 0..n_clis {
            let cli: Handoff<i32>  = Handoff::new(handoff_register::types::NodeId::new(i, "c".to_string()), 1);
            handoff_clis.push(cli);
            let cli : AworsetOpt<i32> = AworsetOpt::new(crdt_sample::NodeId::new(i, "a".to_string()));
            aworset.push(cli);
        }

        for i in 0..n_servers {
            let server: Handoff<i32> = Handoff::new(handoff_register::types::NodeId::new(i, "s".to_string()), 0);
            handoff_servers.push(server);
            let server: AworsetOpt<i32> = AworsetOpt::new(crdt_sample::NodeId::new(n_clis + i, "a".to_string()));
            aworset.push(server);
        }
        
        // Make operations 
        for k in 0..TOTAL_TIME{

            // Apply operations to all elements in handoff
            for i in 0..n_clis {
                let mut h_cli = handoff_clis.get_mut(i as usize).unwrap(); 

                // Stop making operations after NOPER_TIME.
                if k <= NOPER_TIME {
                    // Apply a random number of operations
                    let n_oper = rng.gen_range(0..MAX_OPER); 
                    for _ in 0..n_oper {
                        apply_random_op_handoff(h_cli, &mut rng);
                    }
                }

                // Propagate handoff 
                let num_targets = rng.gen_range(0..n_servers); 
                for i in 0..num_targets{
                    // Get a random server 
                    let server_index = rng.gen_range(0..n_servers);
                    let h_server = handoff_servers.get_mut(server_index as usize).unwrap();
                    // Send information to the servers. 
                    h_server.merge(h_cli);
                    // There is a high change for the server replying.
                    if rng.gen_bool(RESP_PROB) {
                        h_cli.merge(h_server);
                    }
                }
            }


            // Apply operation to all elements in aworset. 
            for i in 0..(n_clis + n_servers) {
                let src_node = aworset.get_mut(i as usize).unwrap();
                if k <= NOPER_TIME {
                    let n_oper = rng.gen_range(0..MAX_OPER); 
                    for _ in 0..n_oper {
                        apply_random_op_AworsetOpt(src_node, &mut rng);
                    }
                }
                let src_node = aworset.get(i as usize).unwrap().clone();

                // Propagate AworsetOpt 
                let index = rng.gen_range(0..(n_clis + n_servers));
                let dst_node = aworset.get_mut(index as usize).unwrap();
                dst_node.join(&src_node);
                // There is a high probability of the other server response. 
                if rng.gen_bool(RESP_PROB){
                    let dst_node = aworset.get(index as usize).unwrap().clone();
                    aworset.get_mut(i as usize).unwrap().join(&dst_node);
                }
            }

            // Calculate state size 
            for i in 0..n_clis {
                h_vec_state_size[k as usize] += handoff_clis.get(i as usize).unwrap().get_num_bytes() as f64;
                vec_state_size[k as usize] += aworset.get(i as usize).unwrap().get_bytes_size() as f64;
            }

            bar.inc(1);
        }


    } 

    writeln!(file, "Handoff,Aworset");
    for k in 0..TOTAL_TIME {
        h_vec_state_size[k as usize] = h_vec_state_size[k as usize] as f64 / (NTESTS as i64 * (n_clis)) as f64; 
        vec_state_size[k as usize] = vec_state_size[k as usize] as f64 / (NTESTS as i64 * (n_clis)) as f64; 
        writeln!(file, "{},{}", h_vec_state_size[k as usize], vec_state_size[k as usize]);
    }

    println!("{:?}", h_vec_state_size);
    println!("{:?}", vec_state_size);
    Ok(())
}

