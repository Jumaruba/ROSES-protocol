mod tester;
use std::fs::File;
use std::io::Write;
use crdt_sample::{NodeId, AworsetOpt};
use handoff_register::handoff::Handoff;
use tester::Tester; 
use rand::prelude::*; 
use indicatif::ProgressBar;

fn apply_random_op_handoff(handoff: &mut Handoff<i32>, rng: &mut ThreadRng){
    // Probability of making an operation
    if rng.gen_bool(0.5){
        let n: i32 = rng.gen_range(0..100);
        if rng.gen_bool(0.7) {
            handoff.add_elem(n);
        } else {
            handoff.rm_elem(n);
        }
    }
}

fn apply_random_op_AworsetOpt(AworsetOpt: &mut AworsetOpt<i32>, rng: &mut ThreadRng){
    // Probability of performing an operation
    if rng.gen_bool(0.5) {
        let n: i32 = rng.gen_range(0..100);
        if rng.gen_bool(0.7) {
            AworsetOpt.add(n);
        } else {
            AworsetOpt.rm(n);
        }
    }
}

#[test]
fn metrics_handoff() -> std::io::Result<()>{
    // SETUP 
    let NTESTS = 1; 
    const NOPER: i32 = 10; 
    let n_clis: i64 = 100; 
    let n_servers: i64 = 10; 

    let file = File::create("metrics").unwrap();
    let mut rng: ThreadRng = rand::thread_rng(); 
    let bar = ProgressBar::new((NOPER * NTESTS).try_into().unwrap()); 
    // CC size 
    let mut h_vec_state_size: [f64; NOPER as usize] = [0.0; NOPER as usize]; 
    let mut vec_state_size : [f64; NOPER as usize] = [0.0; NOPER as usize]; 
    /// Create file to save info;
    let mut file = File::create("metrics.csv").unwrap(); 

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
    for _ in 0..NTESTS {
        for k in 0..NOPER{
            // Apply operations to all elements in handoff
            for i in 0..n_clis{
                let mut h_cli = handoff_clis.get_mut(i as usize).unwrap(); 
                apply_random_op_handoff(h_cli, &mut rng);

                // Propagate handoff 
                let server_index = rng.gen_range(0..n_servers);
                let h_server = handoff_servers.get_mut(server_index as usize).unwrap();
                h_server.merge(h_cli);
                h_cli.merge(h_server);
                h_server.merge(h_cli);
                h_cli.merge(h_server);
            }

            // Apply operation to all elements in aworset. 
            for i in 0..(n_clis + n_servers) {
                let src_node = aworset.get_mut(i as usize).unwrap();
                apply_random_op_AworsetOpt(src_node, &mut rng);
                let src_node = aworset.get(i as usize).unwrap().clone();

                // Propagate AworsetOpt 
                let index = rng.gen_range(0..(n_clis + n_servers));
                let dst_node = aworset.get_mut(index as usize).unwrap();
                dst_node.join(&src_node);
                let dst_node = aworset.get(index as usize).unwrap().clone();
                aworset.get_mut(i as usize).unwrap().join(&dst_node);
            }

            // Calculate state size 
            for i in 0..n_clis {
                h_vec_state_size[k as usize] += handoff_clis.get_mut(i as usize).unwrap().cc.cc.keys().len() as f64;
                vec_state_size[k as usize] += aworset.get_mut(i as usize).unwrap().cc.cc.keys().len() as f64;
            }

            for i in 0..n_servers{
                h_vec_state_size[k as usize] += handoff_servers.get_mut(i as usize).unwrap().cc.cc.keys().len() as f64;
                vec_state_size[k as usize] += aworset.get_mut((i + n_clis) as usize).unwrap().cc.cc.keys().len() as f64;
            }
            bar.inc(1);
        }
    } 

    writeln!(file, "Handoff,Aworset");
    for k in 0..NOPER {
        h_vec_state_size[k as usize] = h_vec_state_size[k as usize] as f64 / (NTESTS as i64 * (n_servers + n_clis)) as f64; 
        vec_state_size[k as usize] = vec_state_size[k as usize] as f64 / (NTESTS as i64 * (n_servers + n_clis)) as f64; 
        writeln!(file, "{},{}", h_vec_state_size[k as usize], vec_state_size[k as usize]);
    }

    println!("{:?}", h_vec_state_size);
    println!("{:?}", vec_state_size);
    Ok(())
}

