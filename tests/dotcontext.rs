use std::collections::{HashSet, HashMap};

use thesis_code::dotcontext::DotContext;

// Add once to a dot
#[test]
pub fn makedot_1(){
    // Given
    let mut dotctx: DotContext<String> = DotContext::new();
    // When 
    let dot = dotctx.makedot(&"A".to_string(), 1);
    // Then 
    let res_dot = ("A".to_string(), 1, 1); 
    let res_dotctx = "DotContext { cc: {\"A\": {1: 1}}, dc: {} }";
    let format_dotctx = format!("{:?}", dotctx);
    assert_eq!(dot, res_dot);
    assert_eq!(format_dotctx, res_dotctx);
}

// Add twice to a dot
#[test]
pub fn makedot_2(){
    // Given
    let mut dotctx: DotContext<String> = DotContext::new();
    dotctx.makedot(&"A".to_string(), 1);

    // When 
    let dot_2 = dotctx.makedot(&"A".to_string(), 1);

    // Then 
    let res_dot = ("A".to_string(), 1, 2); 
    let res_dotctx = "DotContext { cc: {\"A\": {1: 2}}, dc: {} }";
    let format_dotctx = format!("{:?}", dotctx);
    assert_eq!(dot_2, res_dot);
    assert_eq!(format_dotctx, res_dotctx);
}

// Add twice to a dot, but the third is to another source clock
#[test]
pub fn makedot_3(){
    // Given
    let mut dotctx: DotContext<String> = DotContext::new();
    dotctx.makedot(&"A".to_string(), 1);
    dotctx.makedot(&"A".to_string(), 1);

    // When 
    let dot_3 = dotctx.makedot(&"A".to_string(), 3);

    // Then 
    let res_dot = ("A".to_string(), 3, 1); // (E,sck,n)
    assert_eq!(dot_3, res_dot);
    assert_eq!(*dotctx.cc.get(&"A".to_string()).unwrap().get(&3).unwrap(), 1);
    assert_eq!(*dotctx.cc.get(&"A".to_string()).unwrap().get(&1).unwrap(), 2);
}

/// Insert just one dot
#[test]
pub fn insert_dot_1(){
    // Given
    let mut dotctx: DotContext<String> = DotContext::new();
    // When 
    dotctx.insert_dot(&"A".to_string(), 1, 2, Some(false));
    // Then 
    assert_eq!(dotctx.dc[&"A".to_string()], HashSet::from([(1,2)]));
}

/// Insert two dots
#[test]
pub fn insert_dot_2(){
    // Given
    let mut dotctx: DotContext<String> = DotContext::new();
    dotctx.insert_dot(&"A".to_string(), 1, 4, Some(false));
    // When 
    dotctx.insert_dot(&"A".to_string(), 1, 2, Some(false));
    // Then 
    assert_eq!(dotctx.dc[&"A".to_string()], HashSet::from([(1,2), (1,4)]));

}

/// Dotcontexts with empty intersection.
#[test]
pub fn join_1(){
    // Given 
    let mut dc_1: DotContext<String> = DotContext::new(); 
    dc_1.cc = HashMap::from([("A".to_string(), HashMap::from([(1,3)]))]);
    dc_1.dc = HashMap::from([("A".to_string(), HashSet::from([(1,2), (3,4)])), 
        ("B".to_string(), HashSet::from([(2,4), (3,5)]))]);

    let mut dc_2: DotContext<String> = DotContext::new(); 
    dc_2.cc = HashMap::from([("C".to_string(), HashMap::from([(1,3)]))]);
    dc_2.dc = HashMap::from([("D".to_string(), HashSet::from([(1,2), (3,4)])), 
        ("E".to_string(), HashSet::from([(2,4), (3,5)]))]);

    // When 
    dc_1.join(&dc_2);

    // Then 
    let res_cc = HashMap::from([("A".to_string(), HashMap::from([(1,3)])), ("C".to_string(), HashMap::from([(1,3)]))]);
    let res_dc = HashMap::from([("D".to_string(), HashSet::from([(1,2), (3,4)])), ("E".to_string(), HashSet::from([(2,4), (3,5)])), ("A".to_string(), HashSet::from([(3,4)])), ("B".to_string(), HashSet::from([(2,4), (3,5)]))]);
    assert_eq!(dc_1.cc, res_cc);
    assert_eq!(dc_1.dc, res_dc);
}

/// DotContexts with elements in common 
#[test]
pub fn join_2() {
    // Given 
    let mut dc_1: DotContext<String> = DotContext::new(); 
    dc_1.cc = HashMap::from([("A".to_string(), HashMap::from([(1,3)]))]);
    dc_1.dc = HashMap::from([("A".to_string(), HashSet::from([(1,2), (3,4)])), 
        ("B".to_string(), HashSet::from([(2,4), (3,5)]))]);

    let mut dc_2: DotContext<String> = DotContext::new(); 
    dc_2.cc = HashMap::from([("C".to_string(), HashMap::from([(1,3)]))]);
    dc_2.dc = HashMap::from([("A".to_string(), HashSet::from([(2,1), (3,4)])), 
        ("E".to_string(), HashSet::from([(2,4), (3,5)]))]);

    // When 
    dc_1.join(&dc_2);

    // Then 
    let res_cc = HashMap::from([("A".to_string(), HashMap::from([(2,1),(1,3)])), ("C".to_string(), HashMap::from([(1,3)]))]);
    let res_dc = HashMap::from([("E".to_string(), HashSet::from([(2,4), (3,5)])), ("A".to_string(), HashSet::from([(3,4)])), ("B".to_string(), HashSet::from([(2,4), (3,5)]))]);
    assert_eq!(dc_1.cc, res_cc);
    assert_eq!(dc_1.dc, res_dc);
}

/// Compact and add new element to cc.
#[test]
pub fn compact_1(){
    let mut dc_1: DotContext<String> = DotContext::new(); 
    dc_1.cc = HashMap::from([("A".to_string(), HashMap::from([(1,3)]))]);
    dc_1.dc = HashMap::from([("A".to_string(), HashSet::from([(2,1), (3,4)])), 
        ("B".to_string(), HashSet::from([(2,4), (3,5)]))]);

    // When 
    dc_1.compact();

    // Then 
    let res_cc = HashMap::from([("A".to_string(), HashMap::from([(2,1),(1,3)]))]);
    let res_dc = HashMap::from([("A".to_string(), HashSet::from([(3,4)])), ("B".to_string(), HashSet::from([(2,4), (3,5)]))]);
    assert_eq!(dc_1.cc, res_cc);
    assert_eq!(dc_1.dc, res_dc);
}