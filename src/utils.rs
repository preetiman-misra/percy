//! Utility function used in `percy`.

use std::{
    fs::File,
    io::{Result, Write},
    path::Path,
};

use crate::models::Files;

/// Saves the indexed `Files` as pretty printed `json`.
pub fn save_file(data_path: &Path) -> Result<()> {
    let mut files = Files::new();
    files.index_dir(data_path);
    let binding = serde_json::to_string_pretty(&files).unwrap();
    let index_json = binding.as_bytes();

    let mut file = File::create("index.json").unwrap();
    file.write_all(index_json).unwrap();

    Ok(())
}
