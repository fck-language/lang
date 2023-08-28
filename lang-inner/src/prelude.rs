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

#[cfg(debug_assertions)]
impl std::fmt::Debug for LanguageRaw<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.0)
    }
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
#[derive(Clone)]
pub struct Keywords<'a> {
    /// Digits and number prefixes
    pub digits: Digits,
    /// General keywords
    pub keywords: [&'a str; 16],
    /// General keywords
    pub type_kwds: [&'a str; 8],
    /// Localised names for built in types and boolean values
    pub builtins: [&'a str; 6],
    /// Boolean constants
    pub bool: [&'a str; 2],
    /// Keys for [`Keywords::symbols`] used when configuring an inherited language
    pub symbol_keys: [&'a str; 11],
    /// Shell keys for use in the config file
    pub shell_keys: [&'a str; 3],
    /// Manifest key values
    pub manifest_keys: [&'a str; 27],
    /// Words and phrases used when compiling a project
    pub compile_words: [&'a str; 9],
}

/// # Digits type
///
/// Contains the digit characters in the order:
/// - binary prefix (b)
/// - hex prefix (x)
/// - octal prefix (o)
/// - zero to nine characters (0..=9)
/// - ten to fifteen characters for hex (a..=f)
/// - optional repetition of ten to fifteen characters for uppercase usage
#[derive(Clone)]
pub enum Digits {
    /// Short version of digits with no uppercase variants for ten to fifteen characters
    Short {
        /// Digit characters
        digits: [char; 19],
        /// Digit characters as bytes wen cast to a `u32` with their byte length
        u8arrays: [([u8; 4], u8); 19]
    },
    /// Short version of digits with uppercase variants for ten to fifteen characters
    Long {
        /// Digit characters
        digits: [char; 25],
        /// Digit characters as bytes wen cast to a `u32` with their byte length
        u8arrays: [([u8; 4], u8); 25]
    },
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

impl<'a> Index<(u8, u8)> for Errors<'a> {
    type Output = str;
    
    fn index(&self, (i1, i2): (u8, u8)) -> &Self::Output {
        match i1 {
            0 => self.e00[i2 as usize],
            1 => self.e01[i2 as usize],
            2 => self.e02[i2 as usize],
            3 => self.e03[i2 as usize],
            4 => self.e04[i2 as usize],
            _ => panic!("Out of bounds")
        }
    }
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

impl<'a> Index<(u8, u8)> for Warns<'a> {
    type Output = str;
    
    fn index(&self, (i1, i2): (u8, u8)) -> &Self::Output {
        match i1 {
            0 => self.w00[i2 as usize],
            1 => self.w01[i2 as usize],
            2 => self.w02[i2 as usize],
            3 => self.w03[i2 as usize],
            4 => self.w04[i2 as usize],
            _ => panic!("Out of bounds")
        }
    }
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

impl Digits {
    /// Separate a matched digit (`Vec<u8>`) into it's component digit units.
    ///
    /// For example, if we had `vec![50, 48, 51]` for 203, this would separate that into `vec![2, 0, 3]`
    pub fn separate(&self, base: usize, matcher: Vec<u8>) -> Vec<u8> {
        let pos = match self {
            Digits::Short { u8arrays, .. } => u8arrays[3..base + 3].iter().map(|(t, s)| (*t, *s as usize)).enumerate().collect::<Vec<_>>(),
            Digits::Long { u8arrays, .. } => if base == 16 {
                let mut out = u8arrays[3..19].iter().map(|(t, s)| (*t, *s as usize)).enumerate().collect::<Vec<_>>();
                out.extend((10usize..16).zip(u8arrays[19..].iter().map(|(t, s)| (*t, *s as usize))));
                out
            } else {
                u8arrays[3..base + 3].iter().map(|(t, s)| (*t, *s as usize)).enumerate().collect::<Vec<_>>()
            }
        };
        let mut pos = pos.iter().cycle();
        let mut n = 0;
        let mut out = Vec::new();
        while n < matcher.len() {
            if let Some((v, (_, s))) = pos.find(|(_, (t, s))| &t[*s..] == &matcher[n..n + 4 - *s]) {
                out.push(*v as u8);
                n += 4 - s;
            } else { unreachable!("Unable to match digit bytes with digits:\n{:?}\n{:?}", matcher, pos.collect::<Vec<_>>()) }
        }
        out
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
