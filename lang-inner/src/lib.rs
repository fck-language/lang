//! # Lang inner crate
//! This crate contains the base language structs for language things. This includes the internal
//! language representation, language deserialisation (through a public function accessing a private trait)

#![cfg_attr(
    docs,
    feature(doc_auto_cfg),
)]
// #![cfg_attr(
//     docs,
//     feature(doc_auto_cfg),
//     deny(rustdoc::broken_intra_doc_links, missing_docs)
// )]
#![cfg_attr(not(docs), deny(rustdoc::broken_intra_doc_links, missing_docs))]
#![allow(rustdoc::private_intra_doc_links)]

pub mod compress;
mod de;
pub mod prelude;
pub mod tables;
pub mod verify;

pub use prelude::*;
