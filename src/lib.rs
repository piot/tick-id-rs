/*----------------------------------------------------------------------------------------------------------
 *  Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/tick-id-rs
 *  Licensed under the MIT License. See LICENSE in the project root for license information.
 *--------------------------------------------------------------------------------------------------------*/
//! The value type TickId that specifies a specific tick in a simulation.
//!
//! Usually a tick is 16 ms, but can be any integer period of time.

use core::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TickId(pub u32);

impl fmt::Display for TickId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TickId: {}", self.0)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tick_id_display() {
        let tick_id = TickId(42);
        let result = tick_id.to_string();
        assert_eq!(result, "TickId: 42");

        println!("output: {}", result);
    }

    #[test]
    fn test_add() {
        let first = TickId(42);
        let result = first + 99;
        assert_eq!(result, TickId(141));
        assert_eq!(result.to_string(), "TickId: 141");

        println!("output: {}", result);
    }


    #[test]
    fn test_sub() {
        let first = TickId(12414);
        let second = TickId(1144);
        let result = first - second;
        assert_eq!(result, 11270);
    }

    #[test]
    fn test_tick_id_sub() {
        let first = TickId(12414);
        let second = TickId(1144);
        let result = second - first;
        assert_eq!(result, -11270);
    }

    #[test]
    fn test_tick_id_sub_2() {
        let first = TickId(u32::MAX);
        let second = TickId(0);
        let result = second - first;
        assert_eq!(result, -4294967295);
    }

    #[test]
    fn test_tick_id_sub_3() {
        let first = TickId(0);
        let second = TickId(u32::MAX);
        let result = second - first;
        assert_eq!(result, 4294967295);
    }

    #[test]
    fn test_greater() {
        let first = TickId(12414);
        let second = TickId(1144);
        assert_eq!(first > second, true);
    }

    #[test]
    fn test_less() {
        let first = TickId(12414);
        let second = TickId(1144);
        assert_eq!(first < second, false);
        assert_eq!(second < first, true);
    }

    #[test]
    fn test_less_or_equal() {
        let first = TickId(1144);
        let second = TickId(1144);
        assert_eq!(first <= second, true);
    }

    #[test]
    fn test_add_assign() {
        let mut first = TickId(1144);
        first += 1;
        assert_eq!(first.value(), 1145);
    }

    #[test]
    fn test_sub_assign() {
        let mut first = TickId(1144);
        first -= 1;
        assert_eq!(first.value(), 1143);
    }

    #[test]
    fn test_new() {
        let tick_id = TickId::new(12414);
        assert_eq!(tick_id.value(), 12414);
    }

    #[test]
    #[should_panic]
    fn test_add_overflow() {
        let first = TickId(u32::MAX);
        let _ = first + 1;
    }
}
