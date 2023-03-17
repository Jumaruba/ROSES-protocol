// This file contains a specific domain language to create a code to replicate a test.
// Expression to text.
#[macro_export]
macro_rules! C2T {
    (BEGIN) => {
        println!(">> BEGIN"); 
    };

    (END) => {
        println!(">> END");
    };

    (START_SYNC, $x: expr, $y: expr) => {
        println!("-- par SYNC {}, {}", $x.id, $y.id);                // GRAPH 
    };

    (END_SYNC) => {
        println!("-- end");                                         // GRAPH 
    };



    // Two handoff arguments is considered a MERGE.
    (MERGE, $h1: expr, $h2: expr, $show_state: expr) => {
        // Generates code.
        println!("++ {}.merge(&{});", $h1.id, $h2.id);
        println!("++ println!(\"{{}}\", {});", $h1.id);
        // Executes merge bewtween h1 and h2.
        $h1.merge(&$h2);

        // Shows state after merge.
        if $show_state {
            println!("[ MERGE ] {} < {}", $h1.id, $h2.id);
            println!("{}", $h1);
        }

        println!("-- {}->>{}: ", $h1.id, $h2.id);                            // GRAPH
        println!("-- Note over {}: {:?}", $h1.id, $h1.fetch());              // GRAPH
    };

    // Replicas the creation.
    (CREATE, $h: expr) => {
        println!(
            "++ let mut {}: Handoff<i32> = Handoff::new(NodeId::new({}, \"{}\".to_string()), {});",
            $h.id, $h.id.port, $h.id.addr, $h.tier
        );
    };

    (OPER, $h: expr, $enum: tt, $oper: expr) => {
        match $oper {
            $enum::ADD(elem) => {
                println!("-- Note over {}: ADD {}", $h.id, elem);        // GRAPH
                println!("++ println!(\"RM {}\");", elem);
                println!("++ {}.rm_elem({});", $h.id, elem);
            }
            $enum::RM(elem) => {
                println!("-- Note over {}: RM {}", $h.id, elem);         // GRAPH
                println!("++ println!(\"ADD {}\");", elem);
                println!("++ {}.add_elem({});", $h.id, elem);
            }
        }
    };


}


