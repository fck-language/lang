//! # All main language structs
//!
//! This defines all of the main language structs starting at [`LanguageRaw`].
//!
//! It has the following struct dependency layout:
//! - [`LanguageRaw`]
//! 	- [`Keywords`]
//!         - [`Digits`]
//!             - [`DigitsRaw`]
//!         - [`ControlKwds`]
//!         - [`TypeKwds`]
//!         - [`PrimitiveKwds`]
//!         - [`BoolKwds`]
//! 	- [`Messages`]
//! 		- [`Errors`]
//!			- [`Warns`]
//!			- [`CLIKeywords`]
//!             - [`CLICommands`]
//!             - [`CLIArgs`]

mod keywords;
mod err_warn;
pub use keywords::*;
pub use err_warn::*;

use std::fmt::Formatter;
use std::ops::Index;

use crate::de::Deserialize;

/// # Language struct
///
/// This defines a language in fck. This is an internal representation of an fckl file
#[derive(Clone)]
pub struct LanguageRaw<'a> {
    /// Language name: `(full name, language code)`
    pub name: (&'a str, &'a str),
    /// Is the language left-to-right or right-to-left
    pub left_right: bool,
    /// Keywords for the language
    pub keywords: Keywords<'a>,
    /// Messages for the language
    pub messages: Messages<'a>,
}

/// # Text messages
///
/// These are the second part of a language, the first being keywords
#[derive(Copy, Clone)]
pub struct Messages<'a> {
    /// Errors for the language
    pub errors: Errors<'a>,
    /// Warnings for the language
    pub warnings: Warns<'a>,
    /// CLI values for the language
    pub cli_keywords: CLIKeywords<'a>,
}

#[cfg(debug_assertions)]
impl std::fmt::Debug for LanguageRaw<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.0)
    }
}

impl<'a> LanguageRaw<'a> {
    /// Try to deserialize an fckl file into a [`LanguageRaw`] struct
    ///
    /// This is a public wrapper around a call to the [`Deserialize::deserialize`] function
    pub fn from_text(s: &'a str) -> Result<Self, String> {
        let mut lines = s.lines();
        Deserialize::deserialize(&mut lines)
    }
}

/// Table element trait
///
/// This is used instead of [`Index`](std::ops::Index) to give more control over call and return types
pub trait Table<T> {
    /// Get the element at the given row and column
    fn element(&self, row: u16, col: u8) -> T;
}

impl<T: Table<L>, L> Table<L> for &T {
    fn element(&self, row: u16, col: u8) -> L {
        T::element(self, row, col)
    }
}

impl<T: Copy> Table<T> for Vec<[T; 256]> {
    fn element(&self, row: u16, col: u8) -> T {
        self[row as usize][col as usize]
    }
}
