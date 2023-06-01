use std::{fmt::{Display, Debug}, mem::size_of};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ck {
    pub sck: i64,
    pub dck: i64,
}

impl Ck {
    pub fn new(sck: i64, dck: i64) -> Self {
        Self { sck, dck }
    }

    pub fn get_num_bytes(&self) -> usize {
        return size_of::<i64>() + size_of::<i64>(); 
    }
}

impl Display for Ck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.sck, self.dck)
    }
}

impl Debug for Ck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

