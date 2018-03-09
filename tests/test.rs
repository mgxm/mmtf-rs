extern crate mmtf;
extern crate serde_json;
// extern crate serde_transcode;
use mmtf::Mmtf;
use std::fs::{self, DirEntry, File};
use std::path::Path;
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

        let mmtf_json: Value = serde_json::from_reader(json_file).unwrap();

        let mmtf = Mmtf::from(&mmtf_file).unwrap();

        assert_eq!(
            mmtf.mmtf_version,
            mmtf_json["mmtfVersion"].as_str().unwrap()
        );
        assert_eq!(
            mmtf.mmtf_producer,
            mmtf_json["mmtfProducer"].as_str().unwrap()
        );
        assert_eq!(
            mmtf.unit_cell.unwrap(),
            mmtf_json["unitCell"]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_f64().unwrap())
                .collect::<Vec<f64>>()
        );

        if let Some(space_group) = mmtf.space_group {
            assert_eq!(space_group, mmtf_json["spaceGroup"]);
        };

        if let Some(structure) = mmtf.structure_id {
            assert_eq!(structure, mmtf_json["structureId"]);
        };

        if let Some(title) = mmtf.title {
            assert_eq!(title, mmtf_json["title"]);
        };

        if let Some(deposition_date) = mmtf.deposition_date {
            assert_eq!(deposition_date, mmtf_json["depositionDate"]);
        };

        if let Some(release_date) = mmtf.release_date {
            assert_eq!(release_date, mmtf_json["releaseDate"]);
        };
    }
}

