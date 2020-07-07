//! This module contains implementations of `Dummy` trait

#[cfg(feature = "chrono")]
pub mod chrono;
#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "uuid")]
pub mod uuid;

pub mod std;
