use super::decode;

use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};
use std::fs::File;

/// Transform List
///
/// Instructions on how to transform coordinates for an array
/// of chains to create (biological) assemblies.
/// The translational component is given in **Å**.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transform {
    /// Pointers into chain data fields
    pub chain_index_list: Vec<i32>,
    /// 4x4 transformation matrix
    pub matrix: Vec<f32>,
}

/// Bio Assembly
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BioAssembly {
    /// Array of transform objects
    pub transform_list: Vec<Transform>,

    /// Name of the biological assembly
    pub name: String,
}

/// Unique molecular entities within the structure.
///
/// Each entry in [`chain_index_list`](#structfield.chain_index_list)
/// represents an instance of that entity in the structure.
///
/// The entries of [`chain_index_list`](#structfield.chain_index_list) are
/// indices into the [`Mmtf.chain_id_list`](struct.Mmtf.html#structfield.chain_id_list) and
/// [`Mmtf.chain_name_list`](struct.Mmtf.html#structfield.chain_name_list) fields.
/// The sequence string contains the full construct, not just
/// the resolved residues. Its characters are referenced by the
/// entries of the [`Mmtf.sequence_index_list`](struct.Mmtf.html#structfield.sequence_index_list) field.
/// Further, characters follow the IUPAC single letter code for protein
/// or *DNA/RNA* residues, otherwise the character 'X'.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    /// Pointers into chain data fields
    pub chain_index_list: Vec<i32>,
    ///Description of the entity
    pub description: String,
    /// Name of the entity type
    ///
    /// *Note*: This field will be renamed to `type` by serde
    #[serde(rename = "type")]
    pub _type: String,
    /// Sequence of the full construct in one-letter-code
    pub sequence: String,
}

/// Group data
///
/// The fields in the following sections hold group-related data.
/// The mmCIF format allows for so-called micro-heterogeneity on
/// the group-level. For groups (residues) with micro-heterogeneity
/// there are two or more entries given that have the same sequence
/// index, group id (and insertion code) but are of a different group
/// type. The defining property is their identical sequence index.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupType {
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
    /// The version number of the specification the file adheres to.
    pub mmtf_version: String,

    /// The name and version of the software used to produce the file.
    /// For development versions it can be useful to also include the
    /// checksum of the commit. The main purpose of this field is to
    /// identify the software that has written a file, for instance
    /// because it has format errors.
    pub mmtf_producer: String,

    /// Array of six values defining the unit cell. The first three
    /// entries are the length of the sides *a*, *b*, and *c* in **Å**.
    /// The last three angles are the *alpha*, *beta*, and *gamma* angles in **degree**.
    pub unit_cell: Option<Vec<f64>>,

    /// The [Hermann-Mauguin](https://en.wikipedia.org/wiki/Hermann–Mauguin_notation)
    /// space-group symbol.
    pub space_group: Option<String>,

    /// An ID for the structure, for example the PDB ID if applicable.
    /// If not in conflict with the format of the ID, it must be given
    /// in uppercase.
    pub structure_id: Option<String>,

    /// A short description of the structural data included in the file.
    pub title: Option<String>,

    /// A date that relates to the deposition of the structure in a
    /// database, e.g. the wwPDB archive.
    /// Type: String with the format YYYY-MM-DD
    pub deposition_date: Option<String>,

    /// A date that relates to the release of the structure in a
    /// database, e.g. the wwPDB archive.
    ///
    /// *Type*: `String` with the format `YYYY-MM-DD`
    pub release_date: Option<String>,

    /// Array of arrays representing *4x4* transformation matrices that
    /// are stored linearly in row major order. Thus, the translational
    /// component comprises the 4th, 8th, and 12th element.
    /// The transformation matrices describe noncrystallographic symmetry
    /// operations needed to create all molecules in the unit cell.
    pub ncs_operator_list: Option<Vec<[f32; 16]>>,

    /// `Vec` of [BioAssembly](BioAssembly) objects.
    pub bio_assembly_list: Option<Vec<BioAssembly>>,

    /// `Vec` of unique molecular entities within the structure.
    pub entity_list: Option<Vec<Entity>>,

    /// The array of experimental methods employed for structure determination.
    pub experimental_methods: Option<Vec<String>>,

    /// The experimental resolution in Angstrom. If not applicable the field must be omitted.
    pub resolution: Option<f64>,

    /// The R-free value. If not applicable the field must be omitted.
    pub r_free: Option<f32>,

    /// The R-work value. If not applicable the field must be omitted.
    pub r_work: Option<f64>,

    /// The overall number of bonds. This number must reflect both the
    /// bonds given in [`bond_atom_list`](#structfield.bond_atom_list)
    /// and the bonds given in the [`GroupType.bond_atom_list`](GroupType)
    /// entries in [`group_list`](#structfield.group_list)
    pub num_bonds: i32,

    /// The overall number of atoms in the structure. This also includes
    /// atoms at alternate locations ([alt_loc_list](#structfield.alt_loc_list)).
    pub num_atoms: i32,

    /// The overall number of groups in the structure. This also includes extra
    /// groups due to micro-heterogeneity.
    pub num_groups: i32,

    /// The overall number of chains in the structure.
    pub num_chains: i32,

    /// The overall number of models in the structure.
    pub num_models: i32,

    /// `Vec` of [`GroupType`](GroupType) objects.
    pub group_list: Vec<GroupType>,

    /// Pairs of values represent indices of covalently bonded atoms.
    /// The indices point to the Atom data arrays. Only covalent bonds may be given.
    #[serde(deserialize_with = "decode::as_decoder")]
    pub bond_atom_list: Vec<i32>,

    /// Array of bond orders for bonds in [`bond_atom_list`](#structfield.bond_atom_list).
    ///
    /// *Note*: Must be values between 1 and 4, defining **single**,
    /// **double**, **triple**, and **quadruple** bonds.
    #[serde(deserialize_with = "decode::as_decoder")]
    pub bond_order_list: Option<Vec<i8>>,

    /// Array of *x* atom coordinates, in **Å**. One entry for each atom and coordinate.
    #[serde(deserialize_with = "decode::as_decoder")]
    pub x_coord_list: Vec<f32>,

    /// Array of *y* atom coordinates, in **Å**. One entry for each atom and coordinate.
    #[serde(deserialize_with = "decode::as_decoder")]
    pub y_coord_list: Vec<f32>,

    /// Array of *z* atom coordinates, in **Å**. One entry for each atom and coordinate.
    #[serde(deserialize_with = "decode::as_decoder")]
    pub z_coord_list: Vec<f32>,

    /// Array of atom B-factors in in **Å^2**. One entry for each atom.
    #[serde(deserialize_with = "decode::as_decoder")]
    pub b_factor_list: Option<Vec<f32>>,

    /// `Vec` of atom serial numbers. One entry for each atom.
    #[serde(deserialize_with = "decode::as_decoder")]
    pub atom_id_list: Option<Vec<i32>>,

    /// `Vec` of alternate location labels, one for each atom.
    /// The lack of an alternate location label must be denoted by a 0 byte.
    #[serde(deserialize_with = "decode::as_decoder")]
    pub alt_loc_list: Option<Vec<char>>,

    /// `Vec` of atom occupancies, one for each atom.
    #[serde(deserialize_with = "decode::as_decoder")]
    pub occupancy_list: Option<Vec<f32>>,

    /// `Vec` of group (residue) numbers. One entry for each group/residue.
    #[serde(deserialize_with = "decode::as_decoder")]
    pub group_id_list: Vec<i32>,

    /// `Vec` of pointers to [`GroupType`](GroupType) entries
    /// in [`group_list`](#structfield.group_list) by their keys.
    /// One entry for each residue, thus the number of residues
    /// is equal to the length of this field.
    #[serde(deserialize_with = "decode::as_decoder")]
    pub group_type_list: Vec<i32>,

    /// Array of secondary structure assignments coded according
    /// to the following table, which shows the eight different
    /// types of secondary structure the
    /// [DSSP](https://dx.doi.org/10.1002%2Fbip.360221211)
    /// algorithm distinguishes. If the field is included there
    /// must be an entry for each group (residue) either in all
    /// models or only in the first model.
    ///
    /// | Code | Name         |
    /// |------|--------------|
    /// |    0 | pi helix     |
    /// |    1 | bend         |
    /// |    2 | alpha helix  |
    /// |    3 | extended     |
    /// |    4 | 3-10 helix   |
    /// |    5 | bridge       |
    /// |    6 | turn         |
    /// |    7 | coil         |
    /// |   -1 | undefined    |
    #[serde(deserialize_with = "decode::as_decoder")]
    pub sec_struct_list: Option<Vec<i8>>,

    /// Array of insertion codes, one for each group (residue).
    /// The lack of an insertion code must be denoted by a 0 byte.
    #[serde(deserialize_with = "decode::as_decoder")]
    pub ins_code_list: Option<Vec<char>>,

    /// Array of indices that point into the [`sequence`](struct.Entity.html#structfield.sequence)
    /// property of an [`Entity`](Entity) object in the [`entity_list`](#structfield.entity_list) field that
    /// is associated with the chain the group belongs to (i.e. the index of the chain is
    /// included in the [`chain_index_list`](struct.Entity.html#structfield.chain_index_list) of the `entity`).
    /// There is one entry for each group (residue). It must be set to -1 when a group entry has no associated entity
    /// (and thus no sequence), for example water molecules.
    #[serde(deserialize_with = "decode::as_decoder")]
    pub sequence_index_list: Option<Vec<i32>>,

    /// `Vec` of chain IDs, for storing data from `mmCIF` files.
    /// This field should contain the value from `the label_asym_id` `mmCIF` data item
    /// and the [`chain_name_list`](#structfield.chain_name_list) the `auth_asym_id`
    /// `mmCIF` data item.
    ///
    /// In PDB files there is only a single name/identifier for chains that corresponds
    /// to the `auth_asym_id` item. When there is only a single chain identifier available
    /// it must be stored in the `chain_id_list` field.
    #[serde(deserialize_with = "decode::as_decoder")]
    pub chain_id_list: Vec<String>,

    /// `Vec` of chain names. This field allows to specify an additional set of labels/names
    /// for chains.
    ///
    /// For example, it can be used to store both, the `label_asym_id`
    /// (in [`chain_id_list`](#structfield.chain_id_list)) and the `auth_asym_id`
    /// (in [`chain_name_list`](#structfield.chain_name_list)) from mmCIF files.
    #[serde(deserialize_with = "decode::as_decoder")]
    pub chain_name_list: Option<Vec<String>>,

    /// `Vec` of the number of groups (aka residues) in each chain.
    /// The number of chains is thus equal to the length of the
    /// `groups_per_chain` field.
    /// In conjunction with [`chains_per_model`](#structfield.chains_per_model),
    /// the array allows looping over all chains
    pub groups_per_chain: Vec<i32>,

    /// The number of models in a structure is equal to the length of the
    /// `chains_per_model` field.
    /// The `chains_per_model` field also defines which chains belong to each model.
    pub chains_per_model: Vec<i32>,
}

impl Mmtf {
    /// Deserialize a `MMTF` from given file
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::path::Path;
    /// # use std::env;
    /// use std::fs::File;
    /// use mmtf::Mmtf;
    ///
    /// # let file_path = Path::new(&env::current_dir().unwrap())
    /// #                                    .join("tests")
    /// #                                    .join("data")
    /// #                                    .join("173D.mmtf");
    /// # let display = file_path.display();
    /// let mmtf_file = File::open(&file_path).unwrap();
    /// let mmtf = Mmtf::from_file(&mmtf_file);
    ///
    /// assert_eq!("1.0.0", mmtf.mmtf_version);
    /// ```
    pub fn from_file(file: &File) -> Self {
        let mut de = Deserializer::new(file);
        let mmtf: Mmtf = Deserialize::deserialize(&mut de).unwrap();
        mmtf
    }
}
