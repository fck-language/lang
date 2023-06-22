//! # All main language structs
//!
//! This defines all of the main language structs starting at [`LanguageRaw`].
//!
//! It has the following struct dependency layout:
//! - [`LanguageRaw`]
//! 	- [`Keywords`]
//! 	- [`Messages`]
//! 		- [`Errors`]
//!			- [`Warns`]
//!			- [`CLIKeywords`]

use crate::de::Deserialize;
use std::ops::Index;

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

/// # Keywords struct
///
/// Holds all the various keywords and keys for a language
#[derive(Copy, Clone)]
pub struct Keywords<'a> {
    /// General keywords
    pub keywords: [&'a str; 17],
    /// General keywords
    pub type_kwds: [&'a str; 8],
    /// Localised names for built in types and boolean values
    pub builtins: [&'a str; 6],
    /// Boolean constants
    pub bool: [&'a str; 2],
    /// General symbols (string delimiters, comment delimiters, character delimiters,
    /// doc comment symbol)
    pub symbols: [&'a str; 11],
    /// Keys for [`Keywords::symbols`] used when configuring an inherited language
    pub symbol_keys: [&'a str; 11],
    /// Shell keys for use in the config file
    pub shell_keys: [&'a str; 3],
    /// Manifest key values
    pub manifest_keys: [&'a str; 15],
    /// Optional manifest key aliases. For example 'deps' for 'dependencies'
    pub manifest_keys_short: [Option<&'a str>; 15],
    /// Words and phrases used when compiling a project
    pub compile_words: [&'a str; 7],
}

/// # Error message and descriptors
///
/// Holds all the error messages and descriptors. Also includes implementations to make life easy
#[derive(Copy, Clone)]
#[warn(missing_docs)]
pub struct Errors<'a> {
    /// Language based errors
    pub e00: [&'a str; 7],
    pub e01: [&'a str; 2],
    pub e02: [&'a str; 9],
    pub e03: [&'a str; 1],
    pub e04: [&'a str; 2],
}

/// # Warning message and descriptors
///
/// Holds all the warning messages and descriptors. Also includes implementations to make life easy
#[derive(Copy, Clone)]
#[warn(missing_docs)]
pub struct Warns<'a> {
    pub w00: [&'a str; 0],
    pub w01: [&'a str; 0],
    pub w02: [&'a str; 0],
    pub w03: [&'a str; 0],
    pub w04: [&'a str; 0],
}

/// Holds all the CLI commands, arguments, and help descriptions
///
/// Unfortunately, we have to split up single and double flag arguments. sorry
#[derive(Copy, Clone)]
pub struct CLIKeywords<'a> {
    /// fck CLI description
    pub desc: &'a str,
    /// Commands and help descriptions
    pub commands: [(&'a str, &'a str); 10],
    /// Single flag arguments with help messages
    pub args: [(&'a str, char, &'a str); 10],
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
/// This is used instead of [`Index`] to give more control over call and return types
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
