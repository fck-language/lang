//! # Transition table compression
//!
//! This module provides methods of table compression. These are currently being evaluated. Once a
//! preferable compression method is found, this module will most likely be removed and replaced
//! with the chosen compression method

#![allow(missing_docs)]

#[cfg(feature = "macro")]
mod macro_impls;
mod prelude;
pub mod ser;
mod unique_stream;

pub use prelude::*;
pub use unique_stream::UStream;

pub trait Compress<S: Copy + Clone> {
    fn compress(l: &Vec<[S; 256]>) -> Self;

    /// Perform an optimal version of the [fast compression](Compress::compress)
    #[cfg(feature = "macro")]
    fn optimal(l: &Vec<[S; 256]>) -> Self;
}
