//! # Decoder and Encoder for the Macromolecular Transmission Format (MMTF)
//!
//! > The macromolecular transmission format (MMTF) is a binary encoding of biological structures.
//! > It includes the coordinates, the topology and associated data. Specifically, a large subset of
//! > the data in mmCIF or PDB files can be represented. Pronounced goals are a reduced file size for
//! > efficient transmission over the Internet or from hard disk to memory and fast decoding/parsing speed.
//! > Additionally the format aims to be easy to understand and implement to facilitates its dissemination.
//!
//! For a more detailed information and specifications, please, take a look at their official [documentation](https://mmtf.rcsb.org/).
//!
//! This crate use the deserialize and serialize from Rust MessagePack and their integration with Serde,
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
#![warn(missing_debug_implementations, missing_docs)]

extern crate byteorder;
extern crate itertools;
extern crate num_traits;
extern crate num_integer;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;

use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};


pub mod binary_decoder;
pub mod encoding;
pub mod codec;
pub mod encode;
pub mod decode;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TransformList {
    chain_index_list: Vec<i32>,
    matrix: Vec<f32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BioAssemblyList {
    transform_list: Vec<TransformList>,
    name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EntityList {
    chain_index_list: Vec<i32>,
    description: String,
    #[serde(rename = "type")]
    type_: String,
    sequence: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GroupList {
    formal_charge_list: Vec<i32>,
    atom_name_list: Vec<String>,
    element_list: Vec<String>,
    bond_atom_list: Vec<i32>,
    bond_order_list: Vec<i32>,
    group_name: String,
    single_letter_code: char,
    chem_comp_type: String,
}

// Struct that hold all the fields from
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Mmtf {
    mmtf_version: String,
    mmtf_producer: String,
    unit_cell: Option<Vec<f64>>,
    space_group: Option<String>,
    structure_id: Option<String>,
    title: Option<String>,
    deposition_date: Option<String>,
    release_date: Option<String>,
    ncs_operator_list: Option<Vec<[f32; 16]>>,
    bio_assembly_list: Option<Vec<BioAssemblyList>>,
    entity_list: Option<Vec<EntityList>>,
    experimental_methods: Option<Vec<String>>,
    resolution: Option<f64>,
    r_free: Option<f32>,
    r_work: Option<f64>,
    num_bonds: i32,
    num_atoms: i32,
    num_groups: i32,
    num_chains: i32,
    num_models: i32,
    group_list: Vec<GroupList>,
    bond_atom_list: Option<Vec<u8>>,
    bond_order_list: Option<Vec<u8>>,
    x_coord_list: Vec<u8>,
    y_coord_list: Vec<u8>,
    z_coord_list: Vec<u8>,
    b_factor_list: Option<Vec<u8>>,
    atom_id_list: Option<Vec<u8>>,
    alt_loc_list: Option<Vec<u8>>,
    occupancy_list: Option<Vec<u8>>,
    group_id_list: Vec<u8>,
    group_type_list: Vec<u8>,
    sec_struct_list: Option<Vec<u8>>,
    ins_code_list: Option<Vec<u8>>,
    sequence_index_list: Option<Vec<u8>>,
    chain_id_list: Vec<u8>,
    chain_name_list: Option<Vec<u8>>,
    groups_per_chain: Vec<i32>,
    chains_per_model: Vec<i32>,
}
