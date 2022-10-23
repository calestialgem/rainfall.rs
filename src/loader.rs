use std::{
    fs::ReadDir,
    path::{Path, PathBuf},
};

use crate::{Error, Package, Result, Workspace};

/// Representation of a Thrice source file as a linear array of characters.
pub struct Linear {}

/// Load the workspace at the given directory.
pub fn Load(loaded_path: &Path) -> Result<Workspace<Linear>> {
    match loaded_path.read_dir() {
        Ok(directory) => {
            let mut result: Workspace<Linear>;
            for entry in directory {
                match entry {
                    Ok(entry) => {}
                    Err(entry) => {}
                }
            }
            Ok(result)
        }
        Err(io_error) => Err(Error::IOError(
            String::from("Could not read the entries of the workspace directory!"),
            io_error,
        )),
    }
}

/// Load the Thrice package at the given path.
fn LoadPackage(loaded_path: &Path) -> Result<Package<Linear>> {}
