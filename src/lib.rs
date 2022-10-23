use std::{collections::HashMap, path::PathBuf};

/// An error that can happen in the Rainfall compilation process. This is NOT a
/// compilation diagnostic or result!
#[derive(Debug)]
pub enum Error {
    /// An error happened while reading the command-line arguments.
    ArgumentError(String),
    /// An error happened while reading or writing Rainfall setting or Thrice
    /// code files and directories.
    IOError(String, std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArgumentError(message) => {
                write!(f, "Argument Error: {}\nUsage: rainfall", message)
            }
            Self::IOError(message, io_error) => {
                write!(f, "IO Error: {}\nCause: {}", message, io_error)
            }
        }
    }
}

impl std::error::Error for Error {}

/// Rainfall workspace that is generic over the source representation.
struct Workspace<Model> {
    /// Path to the workspace directory.
    path: PathBuf,
    /// Packages in the workspace.
    packages: HashMap<String, Package<Model>>,
}

/// Thrice package that is generic over the source representation.
struct Package<Model> {
    /// Name of the package.
    name: String,
    /// Contents of the package.
    contents: Contents<Model>,
}

/// Contents in a Thrice package that is generic over the source representation.
enum Contents<Model> {
    /// Package as a top-level module directory.
    Directory(Module<Model>),
    /// Package as a single source file.
    File(Source<Model>),
}

/// Thrice module that is generic over the source representation.
struct Module<Model> {
    /// Name of the module.
    name: String,
    /// Portion of contained sources that are directly under this module. Does
    /// not include sources that are contained by the module's submodules.
    sources: HashMap<String, Source<Model>>,
    /// Portion of contained submodules that are directly under this module.
    /// Does not include submodules that are contained by the module's
    /// submodules.
    submodules: HashMap<String, Module<Model>>,
}

/// Thrice source that is generic over the source representation.
struct Source<Model> {
    /// Path to the source file.
    path: PathBuf,
    /// Name of the source.
    name: String,
    /// Representation of the Thrice source in Rainfall.
    model: Model,
}

/// Loads source files to memory as character arrays.
mod loader;

/// Lexes character arrays to arrays of groups of characters.
mod lexer;

/// Parses arrays of groups of characters to forests of groups of characters.
mod parser;

/// Analyzes forests of groups of characters to forests of Thrice objects.
mod analyzer;

/// Generates a C file from forests of Thrice objects.
mod generator;

/// Compiles and runs the generated C file.
mod compiler;
