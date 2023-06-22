//! # Language crate for fck
//!
//! This provides some pre-deserialized languages and DFA maps, as well as all the required structs
//! and functions to do language parsing and lexing

#![cfg_attr(
    docs,
    feature(doc_auto_cfg),
    deny(rustdoc::broken_intra_doc_links, missing_docs)
)]
#![cfg_attr(not(docs), warn(rustdoc::broken_intra_doc_links, missing_docs))]
#![allow(rustdoc::private_intra_doc_links)]

mod lexer;

use lang_inner::compress::UStream;
use lang_inner::{LanguageRaw, Table};
pub use lexer::{tokenize, comments_filter};
pub mod tok;

use lang_macros::languages;
pub mod prelude {
    //! Re-exported structs from [`lang-inner::prelude`](lang_inner::prelude)
    pub use lang_inner::prelude::*;
}

languages!(en, ens, de);
