/*----------------------------------------------------------------------------------------------------------
 *  Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/tick-id-rs
 *  Licensed under the MIT License. See LICENSE in the project root for license information.
 *--------------------------------------------------------------------------------------------------------*/
use core::fmt;
use std::ops::{Add, Sub};

#[derive(Debug, PartialEq)]
pub struct TickId(u32);

impl fmt::Display for TickId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TickId: {}", self.0)
    }
}

impl Add<TickId> for TickId {
    type Output = TickId;

    fn add(self, other: TickId) -> TickId {
        if let Some(sum) = self.0.checked_add(other.0) {
            if sum == u32::MAX {
                panic!("tick reached reserved value {} + {}", self.0, other.0)
            }
        } else {
            panic!("tick overflow. tried to do {} + {}", self.0, other.0)
        }
        TickId(self.0 + other.0)
    }
}

impl Sub<TickId> for TickId {
    type Output = TickId;

    fn sub(self, other: TickId) -> TickId {
        if other.0 > self.0 {
            panic!("tick underflow. tried to do {} - {}", self.0, other.0)
        }
        TickId(self.0 - other.0)
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
        let second = TickId(99);
        let result = first + second;
        assert_eq!(result, TickId(141));
        assert_eq!(result.to_string(), "TickId: 141");

        println!("output: {}", result);
    }

    #[test]
    fn test_sub() {
        let first = TickId(12414);
        let second = TickId(1144);
        let result = first - second;
        assert_eq!(result, TickId(11270));
        assert_eq!(result.to_string(), "TickId: 11270");

        println!("output: {}", result);
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
        let second = TickId(1);
        let _ = first + second;
    }

    #[test]
    #[should_panic]
    fn test_add_reserved() {
        let first = TickId(u32::MAX-1);
        let second = TickId(1);
        let _ = first + second;
    }
}
