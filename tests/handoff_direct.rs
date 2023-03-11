use std::collections::HashSet;

use color_print::cprintln;
use thesis_code::{handoff::Handoff, types::NodeId};

pub fn show_blue(oper: &str, h: &Handoff<i32>) {
    cprintln!("<blue,bold>[{}]</> {}", oper, h);
}

pub fn show_red(oper: &str, h: &Handoff<i32>) {
    cprintln!("<red,bold>[{}]</> {}", oper, h);
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
    let mut h0: Handoff<i32> = Handoff::new(NodeId::new(1, "B".to_string()), 0);

    h0.merge(&h1); show_blue("CREATE SLOT", &h0);
    h1.merge(&h0); show_blue("CREATE TOKEN", &h1);
    h0.merge(&h1); show_blue("FILL SLOT", &h0);
    h1.merge(&h0); show_blue("DISCARD TOKEN", &h1);
    h0.merge(&h1); show_blue("DISCARD TRANSLATION", &h0);
}

#[test]
pub fn test_rm_1() {
    let mut h1: Handoff<i32> = Handoff::new(NodeId::new(1, "A".to_string()), 1);
    h1.add_elem(22);
    let mut h0: Handoff<i32> = Handoff::new(NodeId::new(1, "B".to_string()), 0);

    h1.rm_elem(22); show_red("RM 22", &h1);
    h0.merge(&h1); show_blue("CREATE SLOT", &h0);
    h1.merge(&h0); show_blue("CREATE TOKEN", &h1);
    h0.merge(&h1); show_blue("FILL SLOT", &h0);
    h1.merge(&h0); show_blue("DISCARD TOKEN", &h1);
    h0.merge(&h1); show_blue("DISCARD TRANSLATION", &h0);


    assert_eq!(HashSet::new(), h1.fetch());
    assert_eq!(HashSet::new(), h0.fetch());

}

#[test]
pub fn test_rm_2() {
    let mut h1: Handoff<i32> = Handoff::new(NodeId::new(1, "A".to_string()), 1);
    h1.add_elem(22);
    let mut h0: Handoff<i32> = Handoff::new(NodeId::new(1, "B".to_string()), 0);

    h0.merge(&h1); show_blue("CREATE SLOT", &h0);
    h1.rm_elem(22); show_red("RM 22", &h1);
    h1.merge(&h0); show_blue("CREATE TOKEN", &h1);
    h0.merge(&h1); show_blue("FILL SLOT", &h0);
    h1.merge(&h0); show_blue("DISCARD TOKEN", &h1);
    h0.merge(&h1); show_blue("DISCARD TRANSLATION", &h0);

    assert_eq!(HashSet::new(), h1.fetch());
    assert_eq!(HashSet::new(), h0.fetch());

}

#[test]
pub fn test_rm_3() {
    let mut h1: Handoff<i32> = Handoff::new(NodeId::new(1, "A".to_string()), 1);
    h1.add_elem(22);
    let mut h0: Handoff<i32> = Handoff::new(NodeId::new(1, "B".to_string()), 0);

    h0.merge(&h1); show_blue("CREATE SLOT", &h0);
    h1.merge(&h0); show_blue("CREATE TOKEN", &h1);
    h1.rm_elem(22); show_red("RM 22", &h1);
    h0.merge(&h1); show_blue("FILL SLOT", &h0);
    h1.merge(&h0); show_blue("DISCARD TOKEN", &h1);
    h0.merge(&h1); show_blue("DISCARD TRANSLATION", &h0);

    assert_eq!(HashSet::new(), h1.fetch());
    assert_eq!(HashSet::new(), h0.fetch());
}

#[test]
pub fn test_rm_4() {
    let mut h1: Handoff<i32> = Handoff::new(NodeId::new(1, "A".to_string()), 1);
    h1.add_elem(22);
    let mut h0: Handoff<i32> = Handoff::new(NodeId::new(1, "B".to_string()), 0);

    h0.merge(&h1); show_blue("CREATE SLOT", &h0);
    h1.merge(&h0); show_blue("CREATE TOKEN", &h1);
    h0.merge(&h1); show_blue("FILL SLOT", &h0);
    h1.rm_elem(22); show_red("RM 22", &h1);
    h1.merge(&h0); show_blue("DISCARD TOKEN", &h1);
    h0.merge(&h1); show_blue("DISCARD TRANSLATION", &h0);

    assert_eq!(HashSet::new(), h1.fetch());
    assert_eq!(HashSet::new(), h0.fetch());

}

#[test]
pub fn test_rm_5() {
    let mut h1: Handoff<i32> = Handoff::new(NodeId::new(1, "A".to_string()), 1);
    h1.add_elem(22);
    let mut h0: Handoff<i32> = Handoff::new(NodeId::new(1, "B".to_string()), 0);

    h0.merge(&h1); show_blue("CREATE SLOT", &h0);
    h1.merge(&h0); show_blue("CREATE TOKEN", &h1);
    h0.merge(&h1); show_blue("FILL SLOT", &h0);
    h1.merge(&h0); show_blue("DISCARD TOKEN", &h1);
    h1.rm_elem(22); show_red("RM 22", &h1);
    h0.merge(&h1); show_blue("DISCARD TRANSLATION", &h0);

    assert_eq!(HashSet::new(), h1.fetch());
    assert_eq!(HashSet::new(), h0.fetch());

}

