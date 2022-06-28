#[derive(thiserror::Error, Debug)]
pub enum FileError {
    // Represents a failure to read a file directory
    #[error("Could not read directory `{dir:?}`: {source:?}")]
    DirectoryReadError {
        dir: String,
        source: std::io::Error,
    },

    // Represents a failure to read a file
    #[error("Could not read file `{file:?}`: {source:?}")]
    FileReadError {
        file: String,
        source: std::io::Error,
    },

    // Represents a failure to parse a TOML file
    #[error("Could not parse file `{file:?}`: {source:?}")]
    TOMLParseError {
        file: String,
        source: toml::de::Error,
    },

    // Represents a failure to serialize a TOML file
    #[error("Could not serialize file `{file:?}`: {source:?}")]
    TOMLSerializeError {
        file: String,
        source: toml::ser::Error,
    },

    // Represents a failure to write a file
    #[error("Could not write file `{file:?}`: {source:?}")]
    FileWriteError {
        file: String,
        source: std::io::Error,
    },

    // All other IO errors
    #[error(transparent)]
    IOError { source: std::io::Error }
}

pub trait DataTrait {
    type Output;

    fn from_file(file: &str) -> Result<Self::Output, FileError>;

    fn set_path(&mut self, file_name: &str);
}