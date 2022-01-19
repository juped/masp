//! *General MASP primitives.*
//!
//! `masp_primitives` is a library that provides the core structs and functions necessary
//! for working with MASP based on Zcash Sapling.

#![cfg_attr(docsrs, feature(doc_cfg))]
// Catch documentation errors caused by code changes.
#![deny(rustdoc::broken_intra_doc_links)]

pub mod asset_type;
pub mod constants;
pub mod keys;
pub mod merkle_tree;
pub mod pedersen_hash;
pub mod primitives;
pub mod prover;
pub mod redjubjub;
pub mod sapling;
pub mod zip32;

#[cfg(test)]
mod test_vectors;
