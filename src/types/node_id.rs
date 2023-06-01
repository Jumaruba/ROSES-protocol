use std::{
    fmt::{Debug, Display},
    mem::size_of,
};

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct NodeId {
    pub port: i64,
    pub addr: String,
}
impl NodeId {
    pub fn new(port: i64, addr: String) -> Self {
        Self { port, addr }
    }

    pub fn get_num_bytes(&self) -> usize {
        return self.addr.len() + size_of::<i64>() as usize;
    }
}

impl Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.addr, self.port)
    }
}

impl Debug for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
