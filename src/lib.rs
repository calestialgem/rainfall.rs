/// An error that can happen in the Rainfall compilation process. This is NOT a compilation diagnostic or result!
#[derive(Debug)]
pub enum Error {
    /// An error happened while reading the command-line arguments.
    ArgumentError(String),
    /// An error happened while reading or writing Rainfall setting or Thrice code files and directories.
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
struct Workspace<SourceRepresentation> {
    /// Path to the workspace directory.
    path: std::path::PathBuf,
    /// Packages in the workspace.
    packages: Vec<Package<SourceRepresentation>>,
}

/// Thrice package that is generic over the source representation.
struct Package<SourceRepresentation> {
    /// Name of the package.
    name: String,
    /// Contents of the package.
    contents: PackageContents<SourceRepresentation>,
}

/// Contents in a Thrice package that is generic over the source representation.
enum PackageContents<SourceRepresentation> {
    /// Package as a top-level module directory.
    Directroy(Module<SourceRepresentation>),
    /// Package as a single source file.
    File(Source<SourceRepresentation>),
}

/// Thrice module that is generic over the source representation.
struct Module<SourceRepresentation> {
    /// Name of the module.
    name: String,
    /// Portion of contained sources that are directly under this module. Does not include sources that are contained by the module's submodules.
    sources: Vec<Source<SourceRepresentation>>,
    /// Portion of contained submodules that are directly under this module. Does not include submodules that are contained by the module's submodules.
    submodules: Vec<Module<SourceRepresentation>>,
}

/// Thrice source that is generic over the source representation.
struct Source<SourceRepresentation> {
    /// Path to the source file.
    path: std::path::PathBuf,
    /// Name of the source.
    name: String,
    /// Representation in Rainfall.
    representation: SourceRepresentation,
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
