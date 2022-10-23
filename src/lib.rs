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
