/*----------------------------------------------------------------------------------------------------------
 *  Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/tick-id-rs
 *  Licensed under the MIT License. See LICENSE in the project root for license information.
 *--------------------------------------------------------------------------------------------------------*/
use core::fmt;
use std::ops::{Add, Sub};

#[derive(Debug, PartialEq)]
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

    pub fn new_unchecked(value: u32) -> TickId {
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

impl Sub<u32> for TickId {
    type Output = TickId;

    fn sub(self, other: u32) -> TickId {
        if other > self.0 {
            panic!("tick underflow. tried to do {} - {}", self.0, other)
        }
        Self(self.0 - other)
    }
}

impl Sub<TickId> for TickId {
    type Output = i32;

    fn sub(self, other: TickId) -> i32 {
        (self.0 - other.0) as i32
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
    fn test_new() {
        let _ = TickId::new(12414);
        let _ = TickId::new_unchecked(u32::MAX);
    }

    #[test]
    #[should_panic]
    fn test_sub_underflow() {
        let first = TickId(42);
        let second = TickId(99);
        let _ = first - second;
    }

    #[test]
    #[should_panic]
    fn test_add_overflow() {
        let first = TickId(u32::MAX);
        let _ = first + 1;
    }
}
