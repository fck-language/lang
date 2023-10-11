//! Serialization module
//!
//! Makes a crate public trait [`Serialize`] and implements it for all items in [`lang_inner::prelude`].
//!
//! This module uses the same structure as [`lang_inner::prelude`].

use lang_inner::prelude::{LanguageRaw, Messages};
use proc_macro2::TokenStream;
use quote::quote;

mod keywords;
mod err_warn;

pub(crate) trait Serialize {
    fn serialize(self) -> TokenStream;
}

impl Serialize for LanguageRaw<'_> {
    fn serialize(self) -> TokenStream {
        let (n1, n2) = self.name;
        let name = quote!{name: (#n1, #n2)};
        let left_right = self.left_right;
        let left_right = quote!{left_right: #left_right};
        let kwds = self.keywords.serialize();
        let messages = self.messages.serialize();
        quote!{
            LanguageRaw { #name, #left_right, keywords: #kwds, messages: #messages }
        }
    }
}

impl Serialize for Messages<'_> {
    fn serialize(self) -> TokenStream {
        let errors = self.errors.serialize();
        let warnings = self.warnings.serialize();
        let cli_keywords = self.cli_keywords.serialize();
        quote!{
            Messages { errors: #errors, warnings: #warnings, cli_keywords: #cli_keywords }
        }
    }
}

impl Serialize for &[&str] {
    fn serialize(self) -> TokenStream {
        let t = self;
        quote!{[#(#t),*]}
    }
}
