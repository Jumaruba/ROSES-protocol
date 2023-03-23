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
        println!("++ println!(\"SYNC STEP\");");
    };

    (END_SYNC) => {
        println!("-- end");                                         // GRAPH 
    };



    // Two handoff arguments is considered a MERGE.
    (MERGE, $h1: expr, $h2: expr) => {
        // Generates code.
        println!("++ {}.merge(&{});", $h1.id, $h2.id);
        println!("++ println!(\"{{}}\", {});", $h1.id);
        // Executes merge bewtween h1 and h2.
        $h1.merge(&$h2);

        println!("-- {}->>{}: ", $h2.id, $h1.id);                            // GRAPH
        println!("-- Note over {}: {}<-{}: {:?}", $h1.id, $h1.id, $h2.id, $h1.fetch());              // GRAPH
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
                println!("-- Note over {}: ADD {}, {:?}", $h.id, elem, $h.fetch());        // GRAPH
                println!("++ println!(\"ADD {}\");", elem);
                println!("++ {}.add_elem({});", $h.id, elem);
                println!("++ println!(\"{{}}\", {});", $h.id);

            }
            $enum::RM(elem) => {
                println!("-- Note over {}: RM {}, {:?}", $h.id, elem, $h.fetch());         // GRAPH
                println!("++ println!(\"RM {}\");", elem);
                println!("++ {}.rm_elem({});", $h.id, elem);
                println!("++ println!(\"{{}}\", {});", $h.id);

            }
        }
    };


}


