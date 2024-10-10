/*----------------------------------------------------------------------------------------------------------
 *  Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/tick-id-rs
 *  Licensed under the MIT License. See LICENSE in the project root for license information.
 *--------------------------------------------------------------------------------------------------------*/
//! The value type TickId that specifies a specific tick in a deterministic simulation.
//!
//! Usually a tick is 16 ms or 32 ms, but can be any integer period of time, except 0.

use core::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct TickId(pub u32);

impl fmt::Display for TickId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tick:{:04X}", self.0)
    }
}

impl TickId {
    pub fn new(value: u32) -> Self {
        TickId(value)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

impl Add<u32> for TickId {
    type Output = TickId;

    fn add(self, other: u32) -> TickId {
        if self.0.checked_add(other).is_none() {
            panic!("tick overflow. tried to do {} + {}", self.0, other)
        }
        TickId(self.0 + other)
    }
}

impl AddAssign<u32> for TickId {
    fn add_assign(&mut self, other: u32) {
        self.0 += other;
    }
}

impl Sub<u32> for TickId {
    type Output = TickId;

    fn sub(self, other: u32) -> TickId {
        if other > self.0 {
            panic!("tick underflow. tried to do {} - {}", self.0, other)
        }
        Self(self.0 - other)
    }
}

impl SubAssign<u32> for TickId {
    fn sub_assign(&mut self, other: u32) {
        self.0 -= other;
    }
}

impl Sub<TickId> for TickId {
    type Output = i64;

    fn sub(self, other: TickId) -> i64 {
        (self.0 as i64) - (other.0 as i64)
    }
}
