//! Percy is an application that allows it's users to recursively
//! index a given `data` directory into an `index.json` file.
//!
//! It's goal is to ultimately evolve into a personal search engine,
//! allowing for the creation of knowledge databases that are:
//! * searchable
//! * scalable
//! * deployable (web)
//!
//! You can clone the GitHub Repo and walk through the code
//! on your own machine by doing the following:
//!
//! ```zsh
//! git clone https://github.com/preetiman-misra/percy.git
//! cd percy
//! cargo run
//! ```
//!

use std::{
    ffi::OsString,
    fs::{read_dir, File},
    io::{BufReader, Read, Result, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

/// Struct representing a collection of `NoteFile`s.
#[derive(Debug, Serialize, Deserialize)]
pub struct Files {
    pub notes: Vec<NoteFile>,
}

impl Files {
    /// Create an empty `Files` collection.
    pub fn new() -> Self {
        let notes: Vec<NoteFile> = Vec::new();
        Self { notes }
    }

    /// Recursively index files from the given path.
    pub fn index_dir(&mut self, index_path: &Path) {
        if let Ok(entries) = read_dir(index_path) {
            for entry in entries {
                if let Ok(item) = entry {
                    let file_type = item.file_name();
                    if item.file_type().unwrap().is_dir() {
                        self.index_dir(item.path().as_path());
                    } else {
                        let note = NoteFile::new(item.path(), file_type);
                        self.notes.push(note);
                    }
                }
            }
        }
    }
}

/// Struct containing necessary metadata from indexed files.
#[derive(Debug, Serialize, Deserialize)]
pub struct NoteFile {
    /// Path as of the given `NoteFile`.
    pub path: PathBuf,
    /// File contents of the given `NoteFile`.
    pub content: String,
    /// File Type **(extension)** of the given `NoteFile`.
    pub file_type: String,
}

impl NoteFile {
    /// Create a new `NoteFile` from given
    /// `path: PathBuf` and `file_type: OsString`
    ///
    /// ## Note
    /// `file_type` is simply the file extension, namely
    ///  the string slice after the `.` in a file name.
    /// ```
    /// "hello.txt" => "txt"
    /// ```
    ///
    pub fn new(path: PathBuf, file_type: OsString) -> Self {
        let file = File::open(path.clone()).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();
        let binding = file_type.into_string().unwrap();
        let file_split: Vec<&str> = binding.split(".").collect();
        let file_type = file_split[1].to_string();

        Self {
            path,
            content: contents,
            file_type,
        }
    }
}

/// Saves the indexed `Files` as pretty printed `json`.
fn save_file(data_path: &Path) -> Result<()> {
    let mut files = Files::new();
    files.index_dir(data_path);
    let binding = serde_json::to_string_pretty(&files).unwrap();
    let index_json = binding.as_bytes();

    let mut file = File::create("index.json").unwrap();
    file.write_all(index_json).unwrap();

    Ok(())
}

fn main() -> Result<()> {
    save_file(Path::new("data"))
}
