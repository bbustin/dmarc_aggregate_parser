use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("Cannot open file")]
    Io(#[from] std::io::Error),
    #[error("Parsing error")]
    Parse(#[from] serde_xml_rs::Error),
    #[error("Error unpacking archive")]
    Zip(#[from] zip::result::ZipError),
    #[error("Unexpected file with extension {extension:?})")]
    UnknownFile { extension: std::string::String },
    #[error("Cannot parse a directory. Use parse_dir()")]
    ParseDirectory,
}
