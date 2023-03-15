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

use std::{io::Result, path::Path};

pub mod models;
pub mod utils;

fn main() -> Result<()> {
    utils::save_file(Path::new("data"))
}
