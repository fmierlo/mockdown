//!
//! Mockdown is a single file and macro/dependency free mock library for Rust.
//!
//! # Examples
//!
//! ## Simple
//!
//! ```
//! #[cfg(not(test))]
//! mod math {
//!     pub fn add(x: i32, y: i32) -> i32 {
//!         x + y
//!     }
//! }
//!
//! mod lib {
//!     #[cfg(not(test))]
//!     use super::math;
//!
//!     #[cfg(test)]
//!     use mocks::math;
//!
//!     fn add(x: i32, y: i32) -> i32 {
//!         math::add(x, y)
//!     }
//!
//!     #[cfg(test)]
//!     mod mocks {
//!         pub mod math {
//!             use mockdown::{mockdown, Static};
//!
//!             #[derive(Debug, PartialEq)]
//!             pub struct Add(pub i32, pub i32);
//!
//!             pub fn add(x: i32, y: i32) -> i32 {
//!                 let args = Add(x, y);
//!                 mockdown().mock(args).unwrap()
//!             }
//!         }
//!     }
//!
//!     #[cfg(test)]
//!     mod tests {
//!         use super::math;
//!         use mockdown::{mockdown, Static};
//!
//!         #[test]
//!         fn test_add() {
//!             mockdown().expect(|args| {
//!                 assert_eq!(math::Add(1, 1), args);
//!                 2
//!             });
//!
//!             let z = super::add(1, 1);
//!             assert_eq!(z, 2);
//!         }
//!     }
//! }
//! ```

pub mod global;
pub mod refcell;
pub mod times;

mod mockdown;

pub use mockdown::*;
