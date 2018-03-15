extern crate mmtf;
extern crate serde_json;

use mmtf::Mmtf;
use std::fs::{self, DirEntry, File};
use std::path::Path;
use std::string::String;
use serde_json::Value;

#[test]
fn it_decode_files() {
    let path = Path::new(file!()).parent().unwrap().join("data");

    let mut files: Vec<DirEntry> = Vec::new();

    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = &entry.path();
            let ext = path.extension().unwrap();

            if ext == "mmtf" {
                files.push(entry);
            }
        }
    }

    // Open and decode every mmtf file
    // and open respective json file, that are decoded mmtf files
    // and compare then
    for file in files {
        // extract jus the filename and format str with a json extension
        let json_file_name = format!(
            "{}/{}.json",
            file.path().parent().unwrap().to_str().unwrap(),
            file.path().file_stem().unwrap().to_str().unwrap()
        );

        let mut json_file = File::open(json_file_name).unwrap();
        let mmtf_file = File::open(file.path()).unwrap();

        let mmtf_json: Mmtf = serde_json::from_reader(json_file).unwrap();

        let mmtf = Mmtf::from(&mmtf_file).unwrap();

        assert_eq!(mmtf.mmtf_version, mmtf_json.mmtf_version);

        assert_eq!(mmtf.mmtf_producer, mmtf_json.mmtf_producer);

        assert_eq!(mmtf.unit_cell.unwrap(), mmtf_json.unit_cell.unwrap());

        assert_eq!(mmtf.space_group, mmtf_json.space_group);

        assert_eq!(mmtf.structure_id, mmtf_json.structure_id);

        assert_eq!(mmtf.title, mmtf_json.title);

        assert_eq!(mmtf.deposition_date, mmtf_json.deposition_date);

        assert_eq!(mmtf.release_date, mmtf_json.release_date);

        assert_eq!(mmtf.experimental_methods, mmtf_json.experimental_methods);

        assert_eq!(mmtf.resolution, mmtf_json.resolution);

        assert_eq!(mmtf.r_free, mmtf_json.r_free);

        assert_eq!(mmtf.r_work, mmtf_json.r_work);

        assert_eq!(mmtf.num_bonds, mmtf_json.num_bonds);

        assert_eq!(mmtf.num_atoms, mmtf_json.num_atoms);

        assert_eq!(mmtf.num_groups, mmtf_json.num_groups);

        assert_eq!(mmtf.num_chains, mmtf_json.num_chains);

        assert_eq!(mmtf.num_models, mmtf_json.num_models);

        // TODO: binary operation `==` cannot be applied to type `std::vec::Vec<mmtf::mmtf::GroupType>`
        // assert_eq!(mmtf.group_list, mmtf_json.group_list);

        assert_eq!(mmtf.bond_atom_list, mmtf_json.bond_atom_list);





        // if let Some(bio_assembly) = mmtf.bio_assembly_list {
        //     let bio_assembly_json = &mmtf_json["bioAssemblyList"];

        //     for (bio_index, assembly) in bio_assembly.iter().enumerate() {
        //         assert_eq!(assembly.name, bio_assembly_json[bio_index]["name"]);
        //         for (trans_index, transform) in assembly.transform_list.iter().enumerate() {
        //             let chain_index = bio_assembly_json[bio_index]["transformList"][trans_index]
        //                 ["chainIndexList"]
        //                 .as_array()
        //                 .unwrap()
        //                 .iter()
        //                 .map(|x| i32::from(x) )
        //                 .collect::<Vec<i32>>();
        //             assert_eq!(transform.chain_index_list, chain_index);
        //         }
        //     }
        // };
    }
}
