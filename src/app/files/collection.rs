use std::fs;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::app::app::App;
use crate::app::startup::args::ARGS;
use crate::panic_error;
use crate::request::collection::CollectionFileFormat::{Json, Yaml};
use crate::request::collection::{Collection, CollectionFileFormat};

impl App<'_> {
    /// Set the app request to the requests found in the collection file
    pub fn set_collections_from_file(
        &mut self,
        path_buf: PathBuf,
        file_format: CollectionFileFormat,
    ) {
        let mut file_content = String::new();

        let mut collection_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path_buf.clone())
            .expect("\tCould not open collection file");

        collection_file
            .read_to_string(&mut file_content)
            .expect("\tCould not read collection file");

        let mut collection: Collection = match file_format {
            Json => match serde_json::from_str(&file_content) {
                Ok(collection) => collection,
                Err(e) => panic_error(format!("Could not parse JSON collection\n\t{e}")),
            },
            Yaml => match serde_yaml::from_str(&file_content) {
                Ok(collection) => collection,
                Err(e) => panic_error(format!("Could not parse YAML collection\n\t{e}")),
            },
        };

        collection.path = path_buf;
        collection.file_format = file_format;

        self.collections.push(collection);

        println!("Collection file parsed!");
    }

    /// Save app collection in the collection file through a temporary file
    pub fn save_collection_to_file(&mut self, collection_index: usize) {
        if !ARGS.should_save {
            return;
        }

        let collection = &self.collections[collection_index];

        let temp_file_name = format!(
            "{}_",
            collection.path.file_name().unwrap().to_str().unwrap()
        );

        let temp_file_path = collection.path.with_file_name(temp_file_name);

        let mut temp_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&temp_file_path)
            .expect("Could not open temp file");

        let collection_stringed = match collection.file_format {
            Json => serde_json::to_string_pretty(collection)
                .expect("Could not serialize collection to JSON"),
            Yaml => {
                serde_yaml::to_string(collection).expect("Could not serialize collection to YAML")
            }
        };

        temp_file
            .write_all(collection_stringed.as_bytes())
            .expect("Could not write to temp file");
        temp_file.flush().unwrap();

        fs::rename(temp_file_path, &collection.path)
            .expect("Could not move temp file to collection file");
    }

    /// Delete collection file
    pub fn delete_collection_file(&mut self, collection: Collection) {
        if !ARGS.should_save {
            return;
        }

        fs::remove_file(collection.path).expect("Could not delete collection file");
    }
}
