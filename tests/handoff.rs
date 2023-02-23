use thesis_code::{handoff::HandoffAworSet, nodeId::NodeId};

pub fn id(letter: &str) -> NodeId{
    NodeId::new(1, letter.to_string())
}

#[test]
pub fn add(){
    // Given 
    let mut handoff: HandoffAworSet<String> = HandoffAworSet::new(id("A"), 1);
    // When
    handoff.add("A".to_string());
    // Then
    let res = "HandoffAworSet { id: A1, aworset: AworsetOpt { id: A1, set: {(A1, \"A\", 1)}, cc: DotContext { cc: {A1: 1}, dc: {} } }, sck: 0, dck: 0, slots: {}, tokens: {}, tier: 1 }";
    let curr = format!("{:?}", handoff);
    assert_eq!(curr, res);
}

#[test]
pub fn fetch(){
    let mut handoff: HandoffAworSet<String> = HandoffAworSet::new(id("A"), 1);
    handoff.add("A".to_string());
    // When
    let set = handoff.fetch();

    //Then
    let curr = format!("{:?}", set);
    let res = "{\"A\"}";
    assert_eq!(curr, res);
}

#[test]
/// This test should create a slot
pub fn create_slot_1(){
    // Given
    let mut src : HandoffAworSet<String> = HandoffAworSet::new(id("A"), 1);
    let mut dst: HandoffAworSet<String> = HandoffAworSet::new(id("B"), 0);
    src.add("i".to_string());

    // When 
    dst.create_slot(&src);

    // Then 
    let curr = format!("{:?}", dst);
    let res = "HandoffAworSet { id: B1, aworset: AworsetOpt { id: B1, set: {}, cc: DotContext { cc: {}, dc: {} } }, sck: 0, dck: 1, slots: {A1: (0, 0)}, tokens: {}, tier: 0 }";

    assert_eq!(curr, res);

}

#[test]
/// This test must NOT create a slot, because src did not add new elements.
pub fn create_slot_2(){
    // Given
    let src : HandoffAworSet<String> = HandoffAworSet::new(id("A"), 1);
    let mut dst: HandoffAworSet<String> = HandoffAworSet::new(id("B"), 0);

    // When 
    dst.create_slot(&src);

    // Then 
    let curr = format!("{:?}", dst);
    let res = "HandoffAworSet { id: B1, aworset: AworsetOpt { id: B1, set: {}, cc: DotContext { cc: {}, dc: {} } }, sck: 0, dck: 0, slots: {}, tokens: {}, tier: 0 }";

    assert_eq!(curr, res);

}

#[test]
/// This test must NOT create a slot, because the tiers are from the same level.
pub fn create_slot_3(){
    // Given
    let src : HandoffAworSet<String> = HandoffAworSet::new(id("A"), 0);
    let mut dst: HandoffAworSet<String> = HandoffAworSet::new(id("B"), 0);

    // When 
    dst.create_slot(&src);

    // Then 
    let curr = format!("{:?}", dst);
    let res = "HandoffAworSet { id: B1, aworset: AworsetOpt { id: B1, set: {}, cc: DotContext { cc: {}, dc: {} } }, sck: 0, dck: 0, slots: {}, tokens: {}, tier: 0 }";

    assert_eq!(curr, res);
}

#[test]
pub fn create_token(){
    // Given
    let mut src : HandoffAworSet<String> = HandoffAworSet::new(id("A"), 1);
    let mut dst: HandoffAworSet<String> = HandoffAworSet::new(id("B"), 0);
    src.add("i".to_string());

    // When 
    dst.create_slot(&src);
    src.create_token(&dst);

    // Then 
    let curr = format!("{:?}", src);
    let res = "HandoffAworSet { id: A1, aworset: AworsetOpt { id: A1, set: {}, cc: DotContext { cc: {A1: 0}, dc: {}, dtc: {} } }, sck: 1, dck: 0, slots: {}, tokens: {(A1, B1): ((0, 0), DotContext { cc: {A1: 1}, dc: {}, dtc: {} }, {(A1, \"i\", 1)})}, tier: 1 }";
    assert_eq!(curr, res);
}