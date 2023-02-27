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
    let res_dotctx = "DotContext { cc: {(\"A\", 1): 1}, dc: {} }";
    let format_dotctx = format!("{:?}", dotctx);
    assert_eq!(dot, res_dot);
    assert_eq!(format_dotctx, res_dotctx);
}

// Add twice to a dot
#[test]
pub fn makedot_2(){
    // Given
    let mut dotctx: DotContext<String> = DotContext::new();
    // When 
    dotctx.makedot(&"A".to_string(), 1);
    let dot_2 = dotctx.makedot(&"A".to_string(), 1);

    // Then 
    let res_dot = ("A".to_string(), 1, 2); 
    let res_dotctx = "DotContext { cc: {(\"A\", 1): 2}, dc: {} }";
    let format_dotctx = format!("{:?}", dotctx);
    assert_eq!(dot_2, res_dot);
    assert_eq!(format_dotctx, res_dotctx);
}

#[test]
pub fn insert_dot(){
    // Given
    let mut dotctx: DotContext<String> = DotContext::new();

    // When 
    let dot = ("A".to_string(), 3, 4);
    dotctx.insert_dot(&dot,Some(false));
    
    // Then 
    let format_dotctx = format!("{:?}", dotctx);
    let res_dotctx = "DotContext { cc: {}, dc: {(\"A\", 3, 4)} }";
    assert_eq!(format_dotctx, res_dotctx);
}

// There are no common pairs (id, sck)
#[test]
pub fn join(){
    // Given 
}