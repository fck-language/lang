//! # Language verification
//!
//! Contains a [public function](LanguageRaw::is_invalid) to check if a language is valid by using a
//! [private trait](Verification)
use crate::{CLIKeywords, Digits, Keywords, LanguageRaw, Messages};
use itertools::Itertools;
use std::collections::HashSet;

impl LanguageRaw<'_> {
    /// Check if the language is valid
    ///
    /// 'Valid' is defined by the struct impl of the [`Verification`] trait. Here it checks that the
    /// language code doesn't contain a '!' character.
    ///
    /// This calls [`Verification::is_invalid`] for [`Self::keywords`] and [`Self::messages`]
    pub fn is_invalid(&self) -> bool {
        self.name.1.contains("!") || self.keywords.is_invalid() || self.messages.is_invalid()
    }
}

/// # Language verification trait
///
/// Allows a language to check if it's valid
pub trait Verification {
    /// Check if a struct is valid for a given language. Returns `true` if it's valid, otherwise
    /// returns `false`
    // todo: change return to a better error
    fn is_invalid(&self) -> bool;
}

impl Verification for Keywords<'_> {
    fn is_invalid(&self) -> bool {
        if self.digits.is_invalid() { return true }
        let mut kwds = HashSet::from([
            "+", "-", "%", "*", "/", "**", "=", "==", "<", ">", "<=", ">=", "!", "?", ".", ":",
            ";", "!=", "++", "--", "(", ")", "{", "}", "[", "]", "\"", "'", r"\\", r"\*", r"*\",
            "@", r"\\\"
        ]);
        macro_rules! simple_lists {
		    ($kwds:ident, $($l:ident),*$(,)?) => {$(
				for i in self.$l.into_iter() {
					if !$kwds.insert(i) {
						return true
					}
				}
			)*
			};
		}
        simple_lists!(kwds, keywords, type_kwds, builtins, bool);
        if self.manifest_keys.into_iter().unique().count() != self.manifest_keys.len() {
            return true;
        }
        false
    }
}

impl Verification for Digits {
    fn is_invalid(&self) -> bool {
        let mut check = HashSet::from([
            '+', '-', '%', '*', '/', '=', '<', '>', '!', '?',
            '.', ':', ';', '(', ')', '{', '}', '[', ']',
        ]);
        let (pre, digits, byte_lengths) = match self {
            Digits::Short(t) => ([t.bin_pre, t.hex_pre, t.oct_pre], t.digits.to_vec(), t.u8arrays.map(|(_, l)| 4 - l).to_vec()),
            Digits::Long(t) => ([t.bin_pre, t.hex_pre, t.oct_pre], t.digits.to_vec(), t.u8arrays.map(|(_, l)| 4 - l).to_vec()),
        };
        // can be unsafe since we know the vector is non-empty
        let (bl_first, bl_rem) = unsafe { byte_lengths.split_first().unwrap_unchecked() };
        for i in bl_rem {
            if bl_first != i { return true }
        }
        // check the prefixes are unique and are not in 0..=9
        for i in 0..10 {
            if !check.insert(digits[i]) { return true }
        }
        for i in pre {
            if check.contains(&i) { return true }
        }
        // check the digit characters are all unique
        for i in 10..16 {
            if !check.insert(digits[i]) { return true }
        }
        if let Digits::Long(_) = self {
            for i in 16..22 {
                if !check.insert(digits[i]) { return true }
            }
        }
        false
    }
}

impl Verification for Messages<'_> {
    fn is_invalid(&self) -> bool {
        self.cli_keywords.is_invalid()
    }
}

impl Verification for CLIKeywords<'_> {
    fn is_invalid(&self) -> bool {
        if self.commands.into_iter().map(|t| t.0).unique().count() != self.commands.len() {
            return true;
        }
        if self.args.into_iter().map(|t| t.0).unique().count() != self.args.len() {
            return true;
        }
        if self.args.into_iter().map(|t| t.1).unique().count() != self.args.len() {
            return true;
        }
        false
    }
}
