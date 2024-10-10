/*----------------------------------------------------------------------------------------------------------
 *  Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/tick-id-rs
 *  Licensed under the MIT License. See LICENSE in the project root for license information.
 *--------------------------------------------------------------------------------------------------------*/
//! The `TickId` type represents a specific tick in a deterministic simulation.
//!
//! A tick typically corresponds to a duration of 16 ms or 32 ms, but it can denote any positive integer time
//! period, excluding zero. The `TickId` is implemented as a `u32`, allowing for a wide range of tick values.
//!
//! ## Example
//!
//! ```
//! use tick_id::TickId;
//!
//! let tick_id = TickId::new(1);
//! println!("Tick ID: {}", tick_id);
//! ```
//!
//! ## Operations
//!
//! The `TickId` type supports addition and subtraction operations, allowing for easy manipulation of tick values.
//! Overflow and underflow are checked during arithmetic operations to ensure safety.

use core::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// A unique identifier for a specific tick in a deterministic simulation.
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct TickId(pub u32);

impl fmt::Display for TickId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tick:{:08X}", self.0)
    }
}

impl TickId {
    /// Creates a new `TickId` with the specified value.
    ///
    /// # Arguments
    ///
    /// * `value` - The underlying `u32` value for the `TickId`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tick_id::TickId;
    ///
    /// let tick_id = TickId::new(1);
    /// assert_eq!(tick_id.value(), 1);
    /// ```
    pub fn new(value: u32) -> Self {
        TickId(value)
    }

    /// Returns the underlying `u32` value of the `TickId`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tick_id::TickId;
    ///
    /// let tick_id = TickId::new(1);
    /// assert_eq!(tick_id.value(), 1);
    /// ```
    pub fn value(&self) -> u32 {
        self.0
    }
}

impl Add<u32> for TickId {
    type Output = TickId;

    /// Adds a `u32` value to the `TickId`.
    ///
    /// # Panics
    ///
    /// This method will panic if the addition results in an overflow.
    fn add(self, other: u32) -> TickId {
        if self.0.checked_add(other).is_none() {
            panic!("tick overflow. tried to do {} + {}", self.0, other)
        }
        TickId(self.0 + other)
    }
}

impl AddAssign<u32> for TickId {
    /// Adds a `u32` value to the `TickId` in place.
    fn add_assign(&mut self, other: u32) {
        self.0 += other;
    }
}

impl Sub<u32> for TickId {
    type Output = TickId;

    /// Subtracts a `u32` value from the `TickId`.
    ///
    /// # Panics
    ///
    /// This method will panic if the subtraction results in an underflow.
    fn sub(self, other: u32) -> TickId {
        if other > self.0 {
            panic!("tick underflow. tried to do {} - {}", self.0, other)
        }
        Self(self.0 - other)
    }
}

impl SubAssign<u32> for TickId {
    /// Subtracts a `u32` value from the `TickId` in place.
    fn sub_assign(&mut self, other: u32) {
        self.0 -= other;
    }
}

impl Sub<TickId> for TickId {
    type Output = i64;

    /// Calculates the difference between two `TickId` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use tick_id::TickId;
    ///
    /// let tick_id1 = TickId::new(10);
    /// let tick_id2 = TickId::new(5);
    /// assert_eq!(tick_id1 - tick_id2, 5);
    /// ```
    fn sub(self, other: TickId) -> i64 {
        (self.0 as i64) - (other.0 as i64)
    }
}
