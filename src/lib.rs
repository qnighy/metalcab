//! # MetalCab: Safer Rust binding for MeCab
//!
//! MetalCab is developed as a safer alternative to the [mecab](https://crates.io/crates/mecab) crate. The main differences are:
//!
//! - [`Model::createTagger`][compat::Model::createTagger] generates a lifetime-bounded tagger, keeping it from being orphan.
//! - [`Model::swap`][compat::Model::swap] is marked unsafe because of several implied race conditions.
//! - [`Tagger`][compat::Tagger], [`Model`][compat::Model], and [`Lattice`][compat::Lattice] are explicitly marked `Send + Sync`.
//! - It exposes the raw FFI declarations for more advanced usages.
//! - I'm planning to provide more Rusty, higher-level wrappers for these C++-compatible APIs.
//!
//! [compat::Lattice]: compat/struct.Lattice.html
//! [compat::Model]: compat/struct.Model.html
//! [compat::Model::createTagger]: compat/struct.Model.html#method.createTagger
//! [compat::Model::swap]: compat/struct.Model.html#method.swap
//! [compat::Tagger]: compat/struct.Tagger.html
//!
//! ## Example
//!
//! ```toml
//! [dependencies]
//! metalcab = { git = "https://github.com/qnighy/metalcab.git", rev = "ea5fcfc" }
//! ```
//!
//! ```rust
//! extern crate metalcab;
//! use metalcab::compat::*;
//! use std::ffi::CStr;
//!
//! fn main() {
//!     let input = "太郎は次郎が持っている本を花子に渡した。";
//!
//!     // Create tagger object
//!     let mut mecab = Tagger::create2(CStr::from_bytes_with_nul(b"\0").unwrap()).unwrap();
//!
//!     // Gets tagged result in string.
//!     {
//!         let result = mecab.parse(input).unwrap();
//!         println!("INPUT: {}", input);
//!         print!("RESULT:\n{}", result);
//!     }
//!
//!     {
//!         let mut optnode = Some(mecab.parseToNode(input).unwrap());
//!         println!("INPUT: {}", input);
//!         while let Some(node) = optnode {
//!             print!("/{}", node.surface().unwrap());
//!             optnode = node.next();
//!         }
//!         println!("");
//!     }
//! }
//! ```

pub mod compat;
pub mod raw;
