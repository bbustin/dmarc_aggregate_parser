//! Provides the required error types that are expected to be possible while parsing a file.
use thiserror::Error;
/// An enumeration of possible errors that might occur during the parsing event
#[derive(Error, Debug)]
pub enum ParsingError {
    /// If there is IO error this error will be returned
    #[error("Cannot open file: '{path}' {source}")]
    Io {
        /// Original source error from std::io::Error
        source: std::io::Error,
        /// The path of the file being parsed
        path: String,
    },
    /// This error occurs when the xml file can not be parsed.
    #[error("Parsing error")]
    Parse(#[from] serde_xml_rs::Error),
    /// Error when attempting to de-archive the file.
    #[error("Error unpacking archive")]
    Zip(#[from] zip::result::ZipError),
    /// If the parser encounters an unexpected file
    #[error("Unexpected file with extension {extension:?})")]
    UnknownFile {
        /// The file extension of the file being passed
        extension: std::string::String,
    },
    /// the function [crate::parse()] has be passed a directory instead of file
    #[error("Cannot parse '{path_str}' as it is a directory. Use parse_dir()")]
    ParseDirectory {
        /// The path of the directory that was passed to [crate::parse()]
        path_str: std::string::String,
    },
}
