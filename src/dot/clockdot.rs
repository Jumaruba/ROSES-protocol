use std::{collections::HashSet, cmp::max, fmt::Display};

use priority_queue::PriorityQueue;

#[derive(Debug, Clone)]
pub struct ClockDot {
    pub ck: i64,                        // Clock
    pub compact: i64,                   // Compacted
    pub cloud: PriorityQueue<i64, i64>, // Cloud
}

impl ClockDot {
    pub fn new(ck: i64) -> Self {
        Self {
            ck,
            compact: 0,
            cloud: PriorityQueue::new(),
        }
    }

    /// Compacts counters in the clock dot.
    /// 
    /// # Example
    /// ```
    /// use thesis_code::dot::clockdot::ClockDot;
    ///
    /// let mut cd = ClockDot::new(1);
    /// cd.cloud.push(1,-1);
    /// cd.cloud.push(3, -3);
    /// cd.cloud.push(4, -4);
    /// cd.compact();
    /// assert_eq!(cd.ck, 1);
    /// assert_eq!(cd.cloud.len(), 2);
    /// ```
    pub fn compact(&mut self) {
        while let Some((&n, _)) = self.cloud.peek() {
            if n == self.compact + 1 {
                self.cloud.pop();
                self.compact += 1;
            } else if n <= self.compact {
                self.cloud.pop();
            } 
            else {
                break;
            }
        }
    }


    pub fn get_ck(&self) -> i64 {
        return self.ck;
    }

    /// Creates a new dot entry. 
    pub fn makedot(&mut self) -> i64 {
        self.compact();
        self.ck += 1;
        self.ck
    }
    
    /// Joins two clockdots, moving all the items in other.cloud to self.cloud. 
    /// The result is compacted. 
    /// 
    /// # Example
    /// ```
    /// use thesis_code::dot::clockdot::ClockDot;
    ///
    /// let mut cd_1 = ClockDot::new(1);
    /// cd_1.compact = 3; 
    /// cd_1.cloud.push(5, -5);
    /// let mut cd_2 = ClockDot::new(1);
    /// cd_2.compact = 2; 
    /// cd_2.cloud.push(3,-3);
    /// cd_1.join(&mut cd_2.clone());
    /// let f = format!("{:?}", cd_1);
    /// assert_eq!(1, cd_1.ck);
    /// assert_eq!(5, *cd_1.cloud.peek().unwrap().0);
    /// 
    /// ```
    /// 
    pub fn join(&mut self, other: &mut Self){
        if other.ck == self.ck {
            self.ck = max(self.ck, other.ck);
            self.cloud.append(&mut other.cloud);
        }
        self.compact();
    }
}
