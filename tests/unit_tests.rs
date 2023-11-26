use std::collections::{HashMap, HashSet};

use handoff_register::{
    handoff::Handoff,
    types::{Ck, TDot, NodeId, Payload},
};
mod tester;
use tester::utils::id;


#[test]
pub fn test_create_slot(){
    // Arrange
    let mut c : Handoff<i32> = Handoff::new(id("C", 1), 1);
    let mut s: Handoff<i32> = Handoff::new(id("S", 0), 0);
    let mut result: HashMap<NodeId, Ck> = HashMap::new();
    result.insert(NodeId::new(1, "C".to_string()), Ck::new(1,1));

    // Act
    c.add_elem(10);
    s.merge(&c);    // Create slot

    // Assert
    assert_eq!(s.slots, result);
}


/// This tests verifies a simple scenario of translation.
/// The client creates an element and send it to the server. After that receives a translation.
#[test]
pub fn test_transl(){
    let c_nodeid  = NodeId::new(1, "C".to_string());
    let s_nodeid = NodeId::new(0,  "S".to_string());
    let mut result_cc= HashMap::new();
    result_cc.insert(c_nodeid.clone(), 1);
    result_cc.insert(s_nodeid.clone(), 1);

    // Arrange s
    let mut s: Handoff<i32> = Handoff::new(s_nodeid.clone(), 0);
    s.ck = Ck::new(1,2);
    s.cc.cc.insert(s_nodeid.clone(), 1);
    let mut payload_value = HashSet::new();
    let payload = Payload::new(1,10);
    payload_value.insert(payload);
    s.payload.insert(s_nodeid.clone(), payload_value);
    let transl_ck = Ck::new(1,1);
    s.transl.insert((c_nodeid.clone(), s_nodeid.clone()), (transl_ck, (1,1)));


    // Arrange c
    let mut c : Handoff<i32> = Handoff::new(c_nodeid.clone(), 1);
    c.ck = Ck::new(2,1);
    c.cc.cc.insert(c_nodeid.clone(), 1);
    let mut payload_value = HashSet::new();
    let payload = Payload::new(1,10);
    payload_value.insert(payload);
    c.payload.insert(c_nodeid.clone(), payload_value);
    let token_ck = Ck::new(1,1);
    c.tokens.insert((c_nodeid.clone(), s_nodeid.clone()), (token_ck, (1,2), c.payload.get(&c_nodeid).unwrap().clone()));
    c.last_send_n = 1;


    c.merge(&s);    // Apply translation

    assert_eq!(c.cc.cc, result_cc);
    assert_eq!(c.payload, s.payload);

}




/// This tests verifies a scenario where the element was deleted after sending the token to the server.
/// But the element was deleted before receiving the translation.
/// This test is done with just one element.
#[test]
pub fn test_transl_with_deletion_1(){
    let c_nodeid  = NodeId::new(1, "C".to_string());
    let s_nodeid = NodeId::new(0,  "S".to_string());
    let mut result_cc= HashMap::new();
    result_cc.insert(c_nodeid.clone(), 3);
    result_cc.insert(s_nodeid.clone(), 3);

    // Arrange s
    let mut s: Handoff<i32> = Handoff::new(s_nodeid.clone(), 0);
    s.ck = Ck::new(1,2);
    s.cc.cc.insert(s_nodeid.clone(), 1);
    let mut payload_value = HashSet::new();
    let payload = Payload::new(1,10);
    payload_value.insert(payload);
    s.payload.insert(s_nodeid.clone(), payload_value);
    let transl_ck = Ck::new(1,1);
    s.transl.insert((c_nodeid.clone(), s_nodeid.clone()), (transl_ck, (1,1)));


    // Arrange c
    let mut c : Handoff<i32> = Handoff::new(c_nodeid.clone(), 1);
    c.ck = Ck::new(2,1);
    c.cc.cc.insert(c_nodeid.clone(), 1);

    // The element was deleted after sending the token.
    // Thus there is no insertion in the payload.
    let token_ck = Ck::new(1,1);
    c.tokens.insert((c_nodeid.clone(), s_nodeid.clone()), (token_ck, (1,2), HashSet::new()));
    c.last_send_n = 1;

    c.merge(&s);    // Apply translation

    assert_eq!(c.cc.cc, result_cc);
    assert_eq!(c.payload, HashMap::new());

    println!("{:?}", c);
}



/// This tests verifies a scenario where the element was deleted after sending the token to the server.
/// But the element was deleted before receiving the translation.
/// This test is done with **three** elements in the client and the middle one is deleted.
#[test]
pub fn test_transl_with_deletion_3(){
    let c_nodeid  = NodeId::new(1, "C".to_string());
    let s_nodeid = NodeId::new(0,  "S".to_string());
    let mut result_cc= HashMap::new();
    result_cc.insert(c_nodeid.clone(), 3);
    result_cc.insert(s_nodeid.clone(), 3);
    let mut result_payload = HashMap::new();
    let mut payload_value = HashSet::new();
    payload_value.insert(Payload::new(1,10));
    payload_value.insert(Payload::new(3,12));
    result_payload.insert(s_nodeid.clone(), payload_value);

    // Arrange s
    let mut s: Handoff<i32> = Handoff::new(s_nodeid.clone(), 0);
        // Causal Context
    s.ck = Ck::new(1,2);
    s.cc.cc.insert(s_nodeid.clone(), 3);
        // Payload
    let mut payload_value = HashSet::new();
    payload_value.insert(Payload::new(1,10));
    payload_value.insert(Payload::new(2,11));
    payload_value.insert(Payload::new(3,12));
    s.payload.insert(s_nodeid.clone(), payload_value);
        // Translation
    let transl_ck = Ck::new(1,1);
    s.transl.insert((c_nodeid.clone(), s_nodeid.clone()), (transl_ck, (3,3)));


    // Arrange c
    let mut c : Handoff<i32> = Handoff::new(c_nodeid.clone(), 1);
        // Ck
    c.ck = Ck::new(2,1);
    c.cc.cc.insert(c_nodeid.clone(), 3);
        // Payload
    let mut payload_value = HashSet::new();
    payload_value.insert(Payload::new(1,10));
    payload_value.insert(Payload::new(3,12));   // Element 11 was deleted.
    c.payload.insert(c_nodeid.clone(), payload_value);
        // Token
    let token_ck = Ck::new(1,1);
    c.tokens.insert((c_nodeid.clone(), s_nodeid.clone()), (token_ck, (1,4), c.payload.get(&c_nodeid).unwrap().clone()));
    c.last_send_n = 1;

    println!("{:?}",c);

    c.merge(&s);    // Apply translation

    assert_eq!(c.cc.cc, result_cc);
    assert_eq!(c.payload, result_payload);

}


#[test]
pub fn simple_transl() {
    let mut c: Handoff<i32> = Handoff::new(id("C", 1), 1);
    let mut s: Handoff<i32> = Handoff::new(id("S", 1), 1);

    c.add_elem(10);
    println!("{:?}", c);
    s.merge(&c);    // Create slot
    println!("{:?}", s);
    c.merge(&s);    // Create token
    s.merge(&c);    // Fill slot and create translation.
    println!("{:?}", s);
}


#[test]
pub fn transl_2() {
    let mut c2: Handoff<i32> = Handoff::new(id("C", 1), 1);
    /*
       CLI:: C1, (3, 1)
       tier: 1
       elems: {S1: {(1, 1, 6)}}
       cc: {(S1, 1): 1}
       dc: {}
       slots: {}
       tokens: {(C1, S1): ((2, 2), 1, {(2, 1, 5)})}
       transl: {}
    */

    let mut c2: Handoff<i32> = Handoff::new(id("C", 1), 1);
    let mut s: Handoff<i32> = Handoff::new(id("S", 0), 0);

    println!("ADD 6");
    c2.add_elem(6);
    s.merge(&c2);
    c2.merge(&s);
    println!("ADD 5");
    c2.add_elem(5);
    s.merge(&c2);
    c2.merge(&s);
    println!("ADD 9");
    c2.add_elem(9);
    s.merge(&c2);
    c2.merge(&s);
    s.merge(&c2);
    c2.merge(&s);
    s.merge(&c2);
    s.merge(&c2);

    assert_eq!(c2.fetch(), HashSet::from([6, 5, 9]))
}

// Cache transl and discard translation on the right time.
#[test]
pub fn transl_4() {
    let mut C1: Handoff<i32> = Handoff::new(NodeId::new(1, "C".to_string()), 1);
    let mut S0: Handoff<i32> = Handoff::new(NodeId::new(0, "S".to_string()), 0);
    let mut S1: Handoff<i32> = Handoff::new(NodeId::new(1, "S".to_string()), 0);
    println!("ADD 6");
    C1.add_elem(6);
    println!("{}", C1);
    S0.merge(&C1); // ok
    println!("{}", S0);
    println!("ADD 5");
    C1.add_elem(5); // ok
    println!("{}", C1);
    S1.merge(&C1);  // ok
    println!("{}", S1);
    C1.merge(&S0);  // ok
    println!("{}", C1);
    S1.merge(&C1);  // ok token caching.
    println!("{}", S1);
    C1.merge(&S0);  // ok ind
    println!("{}", C1);
    println!("ADD 1");
    C1.add_elem(1); // ok
    println!("{}", C1);


    S0.merge(&C1);  // ok
    println!("{}", S0);
    C1.merge(&S1);  // ok
    println!("{}", C1);
    S1.merge(&C1);  // ok
    println!("{}", S1);
    C1.merge(&S1);
    println!("{}", C1);
    println!("ADD 5");
    C1.add_elem(5);
    println!("{}", C1);
    S0.merge(&C1);
    println!("{}", S0);
    C1.merge(&S1);
    println!("{}", C1);
    S1.merge(&S0);
    println!("{}", S1);
    S0.merge(&S1);
    println!("{}", S0);
    println!("RM 5");
    C1.rm_elem(5);
    println!("{}", C1);
    S0.merge(&C1);
    println!("{}", S0);
    C1.merge(&S0);
    println!("{}", C1);
    S0.merge(&C1);
    println!("{}", S0);
    C1.merge(&S1);
    println!("{}", C1);
    S1.merge(&C1);
    println!("{}", S1);
    C1.merge(&S0);
    println!("{}", C1);
    S1.merge(&C1);
    println!("{}", S1);
    C1.merge(&S1);
    println!("{}", C1);
    S0.merge(&C1);
    println!("{}", S0);
    C1.merge(&S1);
    println!("{}", C1);
    S0.merge(&S1);
    println!("{}", S0);
    println!("SYNC STEP");
    C1.merge(&S0);
    println!("{}", C1);
    C1.merge(&S1);
    println!("{}", C1);
    S0.merge(&S1);
    println!("{}", S0);
    S1.merge(&S0);
    println!("{}", S1);
    assert_eq!(HashSet::from([1, 6]), S1.fetch());
    assert_eq!(HashSet::from([1, 6]), S1.fetch());
    assert_eq!(HashSet::from([1, 6]), S1.fetch());
}

#[test]
pub fn transl_3() {
    let mut C1: Handoff<i32> = Handoff::new(NodeId::new(1, "C".to_string()), 1);
    let mut S0: Handoff<i32> = Handoff::new(NodeId::new(0, "S".to_string()), 0);
    let mut S1: Handoff<i32> = Handoff::new(NodeId::new(1, "S".to_string()), 0);
    let mut S2: Handoff<i32> = Handoff::new(NodeId::new(2, "S".to_string()), 0);
    println!("ADD 7");
    C1.add_elem(7);
    println!("{}", C1);
    S2.merge(&C1);
    println!("{}", S2);
    C1.merge(&S0);
    println!("{}", C1);
    S0.merge(&C1);
    println!("{}", S0);
    C1.merge(&S2);
    println!("{}", C1);
    println!("ADD 5");
    C1.add_elem(5);
    println!("{}", C1);
    S1.merge(&C1);
    println!("{}", S1);
    C1.merge(&S1);
    println!("{}", C1);
    S2.merge(&C1);
    println!("{}", S2);
    C1.merge(&S1);
    println!("{}", C1);
    println!("ADD 2");
    C1.add_elem(2);
    println!("{}", C1);
    S1.merge(&C1);
    println!("{}", S1);
    C1.merge(&S1);
    println!("{}", C1);
    S0.merge(&C1);
    println!("{}", S0);
    C1.merge(&S2);
    println!("{}", C1);
    println!("ADD 3");
    C1.add_elem(3);
    println!("{}", C1);
    S0.merge(&C1);
    println!("{}", S0);
    C1.merge(&S1);
    println!("{}", C1);
    S2.merge(&C1);
    println!("{}", S2);
    C1.merge(&S1);
    println!("{}", C1);
    println!("ADD 6");
    C1.add_elem(6);
    println!("{}", C1);
    S1.merge(&C1);
    println!("{}", S1);
    C1.merge(&S1);
    println!("{}", C1);
    S2.merge(&C1);
    println!("{}", S2);
    C1.merge(&S1);
    println!("{}", C1);
    println!("ADD 4");
    C1.add_elem(4);
    println!("{}", C1);
    S2.merge(&C1);
    println!("{}", S2);
    C1.merge(&S1);
    println!("{}", C1);
    S1.merge(&C1);
    println!("{}", S1);
    C1.merge(&S0);
    println!("{}", C1);
    println!("ADD 8");
    C1.add_elem(8);
    println!("{}", C1);
    S1.merge(&S2);
    println!("{}", S1);
    C1.merge(&S2);
    println!("{}", C1);
    S0.merge(&C1);
    println!("{}", S0);
    C1.merge(&S2);
    println!("{}", C1);
    println!("RM 3");
    C1.rm_elem(3);
    println!("{}", C1);
    S0.merge(&S1);
    println!("{}", S0);
    C1.merge(&S2);
    println!("{}", C1);
    S2.merge(&S0);
    println!("{}", S2);
    C1.merge(&S2);
    println!("{}", C1);
    println!("ADD 6");
    C1.add_elem(6);
    println!("{}", C1);
    S1.merge(&C1);
    println!("{}", S1);
    C1.merge(&S2);
    println!("{}", C1);
    S0.merge(&C1);
    println!("{}", S0);
    C1.merge(&S2);
    println!("{}", C1);
    println!("RM 4");
    C1.rm_elem(4);
    println!("{}", C1);
    S1.merge(&S2);
    println!("{}", S1);
    C1.merge(&S1);
    println!("{}", C1);
    S2.merge(&C1);
    println!("{}", S2);
    C1.merge(&S2);
    println!("{}", C1);
    S1.merge(&C1);
    println!("{}", S1);
    C1.merge(&S0);
    println!("{}", C1);
    S2.merge(&C1);
    println!("{}", S2);
    C1.merge(&S0);
    println!("{}", C1);
    S1.merge(&C1);
    println!("{}", S1);
    C1.merge(&S0);
    println!("{}", C1);
    S0.merge(&C1);
    println!("{}", S0);
    S2.merge(&S0);
    println!("{}", S2);
    S2.merge(&C1);
    println!("{}", S2);
    C1.merge(&S1);
    println!("{}", C1);
    S0.merge(&C1);
    println!("{}", S0);
    println!("SYNC STEP");
    C1.merge(&S0);
    println!("{}", C1);
    C1.merge(&S1);
    println!("{}", C1);
    C1.merge(&S2);
    println!("{}", C1);
    S0.merge(&S1);
    println!("{}", S0);
    S0.merge(&S2);
    println!("{}", S0);
    S1.merge(&S0);
    println!("{}", S1);
    S1.merge(&S2);
    println!("{}", S1);
    S2.merge(&S0);
    println!("{}", S2);
    S2.merge(&S1);
    println!("{}", S2);
    /*

    thread 'test_rnd_1xn_seq' panicked at 'assertion failed: `(left == right)`
      left: `{6, 8, 5, 7, 2}`,
     right: `{2, 8, 6, 5, 7, 3}`', tests/test_rnd_1xn_seq.rs:137:9
     */
    assert_eq!(HashSet::from([6, 8, 5, 7, 2]), C1.fetch());
    assert_eq!(HashSet::from([6, 8, 5, 7, 2]), S1.fetch());
    assert_eq!(HashSet::from([6, 8, 5, 7, 2]), S2.fetch());
    assert_eq!(HashSet::from([6, 8, 5, 7, 2]), S0.fetch());
}


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

