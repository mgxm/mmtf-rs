use super::decode;

use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};
use serde_json;
use std::fs::File;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformList {
    pub chain_index_list: Vec<i32>,
    pub matrix: Vec<f32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BioAssemblyList {
    pub transform_list: Vec<TransformList>,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityList {
    pub chain_index_list: Vec<i32>,
    pub description: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub sequence: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupList {
    pub formal_charge_list: Vec<i32>,
    pub atom_name_list: Vec<String>,
    pub element_list: Option<Vec<String>>,
    pub bond_atom_list: Vec<i32>,
    pub bond_order_list: Vec<i32>,
    pub group_name: String,
    pub single_letter_code: String,
    pub chem_comp_type: String,
}

// Struct that hold all the fields from
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mmtf {
    pub mmtf_version: String,
    pub mmtf_producer: String,
    pub unit_cell: Option<Vec<f64>>,
    pub space_group: Option<String>,
    pub structure_id: Option<String>,
    pub title: Option<String>,
    pub deposition_date: Option<String>,
    pub release_date: Option<String>,
    pub ncs_operator_list: Option<Vec<[f32; 16]>>,
    pub bio_assembly_list: Option<Vec<BioAssemblyList>>,
    pub entity_list: Option<Vec<EntityList>>,
    pub experimental_methods: Option<Vec<String>>,
    pub resolution: Option<f64>,
    pub r_free: Option<f32>,
    pub r_work: Option<f64>,
    pub num_bonds: i32,
    pub num_atoms: i32,
    pub num_groups: i32,
    pub num_chains: i32,
    pub num_models: i32,
    pub group_list: Vec<GroupList>,
    #[serde(deserialize_with = "decode::as_decoder")]
    pub bond_atom_list: Vec<i32>,
    #[serde(deserialize_with = "decode::as_decoder")]
    pub bond_order_list: Option<Vec<i8>>,
    #[serde(deserialize_with = "decode::as_decoder")]
    pub x_coord_list: Vec<f32>,
    #[serde(deserialize_with = "decode::as_decoder")]
    pub y_coord_list: Vec<f32>,
    #[serde(deserialize_with = "decode::as_decoder")]
    pub z_coord_list: Vec<f32>,
    #[serde(deserialize_with = "decode::as_decoder")]
    pub b_factor_list: Option<Vec<f32>>,
    #[serde(deserialize_with = "decode::as_decoder")]
    pub atom_id_list: Option<Vec<i32>>,
    #[serde(deserialize_with = "decode::as_decoder")]
    pub alt_loc_list: Option<Vec<char>>,
    #[serde(deserialize_with = "decode::as_decoder")]
    pub occupancy_list: Option<Vec<f32>>,
    #[serde(deserialize_with = "decode::as_decoder")]
    pub group_id_list: Vec<i32>,
    #[serde(deserialize_with = "decode::as_decoder")]
    pub group_type_list: Vec<i32>,
    #[serde(deserialize_with = "decode::as_decoder")]
    pub sec_struct_list: Option<Vec<i8>>,
    #[serde(deserialize_with = "decode::as_decoder")]
    pub ins_code_list: Option<Vec<char>>,
    #[serde(deserialize_with = "decode::as_decoder")]
    pub sequence_index_list: Option<Vec<i32>>,
    #[serde(deserialize_with = "decode::as_decoder")]
    pub chain_id_list: Vec<String>,
    #[serde(deserialize_with = "decode::as_decoder")]
    pub chain_name_list: Option<Vec<String>>,
    pub groups_per_chain: Vec<i32>,
    pub chains_per_model: Vec<i32>,
}

impl Mmtf {
    pub fn from_file(file: &File) -> Self {
        let mut de = Deserializer::new(file);
        let mmtf: Mmtf = Deserialize::deserialize(&mut de).unwrap();
        mmtf
    }
}
