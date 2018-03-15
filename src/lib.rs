//! # Decoder and Encoder for the Macromolecular Transmission Format (MMTF)
//!
//! > The macromolecular transmission format (MMTF) is a binary encoding of biological structures.
//! > It includes the coordinates, the topology and associated data. Specifically, a large subset of
//! > the data in `mmCIF` or `PDB` files can be represented. Pronounced goals are a reduced file size for
//! > efficient transmission over the Internet or from hard disk to memory and fast decoding/parsing speed.
//! > Additionally the format aims to be easy to understand and implement to facilitates its dissemination.
//!
//! For a more detailed information and specifications, please, take a look at their official [documentation](https://mmtf.rcsb.org/).
//!
//! This crate use the deserialize and serialize from Rust `MessagePack` and their integration with Serde,
//! providing a simple and easily decoder and decoder for MMTF structures
//!
//! # Examples
//! ## Decoder
//! ```rust
//! extern crate mmtf;
//!
//! #
//! # fn main() {}
//! ```
//!
//!
#![warn(missing_debug_implementations)]

extern crate byteorder;
extern crate itertools;
extern crate num_integer;
extern crate num_traits;
extern crate rmp_serde as rmps;
#[macro_use]
extern crate serde;
extern crate serde_bytes;
#[macro_use]
extern crate serde_derive;

pub mod binary_decoder;
pub mod encoding;
pub mod codec;
pub mod encode;
pub mod decode;
pub mod mmtf;
pub mod de;

pub use mmtf::Mmtf;
