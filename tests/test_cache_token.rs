use std::collections::HashSet;

use color_print::cprintln;
use handoff_register::{handoff::Handoff, types::NodeId};
mod tester;
use tester::utils::{show_blue, show_red};


#[test]
pub fn rm_element() {
    let mut handoff: Handoff<i32> = Handoff::new(NodeId::new(1, "A".to_string()), 1);
    handoff.rm_elem(21);
    println!("{:?}", handoff);
}

/// There are no failures and no removals.
#[test]
pub fn test_add() {
    let mut h1: Handoff<i32> = Handoff::new(NodeId::new(1, "A".to_string()), 1);
    h1.add_elem(22);
    let mut server_1: Handoff<i32> = Handoff::new(NodeId::new(1, "S".to_string()), 0);
    let mut server_2: Handoff<i32> = Handoff::new(NodeId::new(2, "S".to_string()), 0);

    server_1.merge(&h1); show_blue("CREATE SLOT", &server_1);
    h1.merge(&server_1); show_blue("CREATE TOKEN", &h1);
    // Server 1 crashes. 
    cprintln!("<red, bold> >> SERVER 1 CRASHED << </>\n");
    server_2.merge(&h1); show_blue("CACHE TOKEN", &server_2);   // Cache token. Do not create slot. 

    // Server 1 back online. 
    cprintln!("<green, bold> >> SERVER 1 RECOVER<< </>\n");
    server_1.merge(&server_2); show_blue("FILL SLOT", &server_1);
    h1.merge(&server_1); show_blue("DISCARD TOKEN", &h1);
    server_1.merge(&h1); show_blue("DISCARD TRANSLATION", &server_1);
}

#[test]
pub fn test_rm_1() {
    let mut h1: Handoff<i32> = Handoff::new(NodeId::new(1, "A".to_string()), 1);
    h1.add_elem(22);
    let mut server_1: Handoff<i32> = Handoff::new(NodeId::new(1, "S".to_string()), 0);
    let mut server_2: Handoff<i32> = Handoff::new(NodeId::new(2, "S".to_string()), 0);

    h1.rm_elem(22); show_red("RM 22", &h1);
    server_1.merge(&h1); show_blue("CREATE SLOT", &server_1);
    h1.merge(&server_1); show_blue("CREATE TOKEN", &h1);
    // Server 1 crashes. 
    cprintln!("<red, bold> >> SERVER 1 CRASHED << </>\n");
    server_2.merge(&h1); show_blue("CACHE TOKEN", &server_2);   // Cache token. Do not create slot. 

    // Server 1 back online. 
    cprintln!("<green, bold> >> SERVER 1 RECOVER<< </>\n");
    server_1.merge(&server_2); show_blue("FILL SLOT", &server_1);
    h1.merge(&server_1); show_blue("DISCARD TOKEN", &h1);
    server_1.merge(&h1); show_blue("DISCARD TRANSLATION", &server_1);

    assert_eq!(HashSet::new(), h1.fetch());
    assert_eq!(HashSet::new(), server_1.fetch());

}

#[test]
pub fn test_rm_2() {
    let mut h1: Handoff<i32> = Handoff::new(NodeId::new(1, "A".to_string()), 1);
    h1.add_elem(22);
    let mut server_1: Handoff<i32> = Handoff::new(NodeId::new(1, "S".to_string()), 0);
    let mut server_2: Handoff<i32> = Handoff::new(NodeId::new(2, "S".to_string()), 0);

    server_1.merge(&h1); show_blue("CREATE SLOT", &server_1);
    h1.rm_elem(22); show_red("RM 22", &h1);
    h1.merge(&server_1); show_blue("CREATE TOKEN", &h1);
    // Server 1 crashes. 
    cprintln!("<red, bold> >> SERVER 1 CRASHED << </>\n");
    server_2.merge(&h1); show_blue("CACHE TOKEN", &server_2);   // Cache token. Do not create slot. 

    // Server 1 back online. 
    cprintln!("<green, bold> >> SERVER 1 RECOVER<< </>\n");
    server_1.merge(&server_2); show_blue("FILL SLOT", &server_1);
    h1.merge(&server_1); show_blue("DISCARD TOKEN", &h1);
    server_1.merge(&h1); show_blue("DISCARD TRANSLATION", &server_1);

    assert_eq!(HashSet::new(), h1.fetch());
    assert_eq!(HashSet::new(), server_1.fetch());

}

#[test]
pub fn test_rm_3() {
    let mut h1: Handoff<i32> = Handoff::new(NodeId::new(1, "A".to_string()), 1);
    h1.add_elem(22);
    let mut server_1: Handoff<i32> = Handoff::new(NodeId::new(1, "S".to_string()), 0);
    let mut server_2: Handoff<i32> = Handoff::new(NodeId::new(2, "S".to_string()), 0);

    server_1.merge(&h1); show_blue("CREATE SLOT", &server_1);
    h1.merge(&server_1); show_blue("CREATE TOKEN", &h1);
    h1.rm_elem(22); show_red("RM 22", &h1);

    // Server 1 crashes. 
    cprintln!("<red, bold> >> SERVER 1 CRASHED << </>\n");
    server_2.merge(&h1); show_blue("CACHE TOKEN", &server_2);   // Cache token. Do not create slot. 

    // Server 1 back online. 
    cprintln!("<green, bold> >> SERVER 1 RECOVER<< </>\n");
    server_1.merge(&server_2); show_blue("FILL SLOT", &server_1);
    h1.merge(&server_1); show_blue("DISCARD TOKEN", &h1);
    server_1.merge(&h1); show_blue("DISCARD TRANSLATION", &server_1);

    assert_eq!(HashSet::new(), h1.fetch());
    assert_eq!(HashSet::new(), server_1.fetch());
}

#[test]
pub fn test_rm_4() {
    let mut h1: Handoff<i32> = Handoff::new(NodeId::new(1, "A".to_string()), 1);
    h1.add_elem(22);
    let mut server_1: Handoff<i32> = Handoff::new(NodeId::new(1, "S".to_string()), 0);
    let mut server_2: Handoff<i32> = Handoff::new(NodeId::new(2, "S".to_string()), 0);

    server_1.merge(&h1); show_blue("CREATE SLOT", &server_1);
    h1.merge(&server_1); show_blue("CREATE TOKEN", &h1);

    // Server 1 crashes. 
    cprintln!("<red, bold> >> SERVER 1 CRASHED << </>\n");
    server_2.merge(&h1); show_blue("CACHE TOKEN", &server_2);   // Cache token. Do not create slot. 
    h1.rm_elem(22); show_red("RM 22", &h1);

    // Server 1 back online. 
    cprintln!("<green, bold> >> SERVER 1 RECOVER<< </>\n");
    server_1.merge(&server_2); show_blue("FILL SLOT", &server_1);
    h1.merge(&server_1); show_blue("DISCARD TOKEN", &h1);
    server_1.merge(&h1); show_blue("DISCARD TRANSLATION", &server_1);

    assert_eq!(HashSet::new(), h1.fetch());
    assert_eq!(HashSet::new(), server_1.fetch());

}

#[test]
pub fn test_rm_5() {
    let mut h1: Handoff<i32> = Handoff::new(NodeId::new(1, "A".to_string()), 1);
    h1.add_elem(22);
    let mut server_1: Handoff<i32> = Handoff::new(NodeId::new(1, "S".to_string()), 0);
    let mut server_2: Handoff<i32> = Handoff::new(NodeId::new(2, "S".to_string()), 0);

    server_1.merge(&h1); show_blue("CREATE SLOT", &server_1);
    h1.merge(&server_1); show_blue("CREATE TOKEN", &h1);

    // Server 1 crashes. 
    cprintln!("<red, bold> >> SERVER 1 CRASHED << </>\n");
    server_2.merge(&h1); show_blue("CACHE TOKEN", &server_2);   // Cache token. Do not create slot. 
    
    // Server 1 back online. 
    cprintln!("<green, bold> >> SERVER 1 RECOVER<< </>\n");
    server_1.merge(&server_2); show_blue("FILL SLOT", &server_1);
    h1.rm_elem(22); show_red("RM 22", &h1);
    h1.merge(&server_1); show_blue("TRANSLATE TOKEN", &h1);
    server_1.merge(&h1); show_blue("DISCARD TRANSLATION", &server_1);
    h1.merge(&server_1); show_blue("DISCARD TOKEN", &h1);

    assert_eq!(HashSet::new(), h1.fetch());
    assert_eq!(HashSet::new(), server_1.fetch());

}

