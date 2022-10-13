#![deny(missing_debug_implementations)]
pub mod aggregate_report;
pub mod error_handling;
use error_handling::ParsingError;

use libflate::gzip;
use serde_xml_rs::from_reader;
use std::fs::{read_dir, File};
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

pub fn parse(path: &Path) -> Result<aggregate_report::feedback, ParsingError> {
    if path.is_dir() {
        panic!("path is a directory")
    }

    let extension = path.extension();

    if let Some(extension) = extension {
        match extension.to_str() {
            Some("xml") => {
                let mut reader = get_file_reader(path)?;
                return parse_reader(&mut reader);
            }
            Some("gz") | Some("gzip") => {
                let reader = get_file_reader(path)?;
                let mut decoder = gzip::Decoder::new(reader)?;
                return parse_reader(&mut decoder);
            }
            Some("zip") => {
                let file = File::open(path)?;
                let mut archive = zip::ZipArchive::new(file)?;
                let mut file = archive.by_index(0)?;
                return parse_reader(&mut file);
            }
            _ => {
                let extension = extension.to_str().unwrap_or("").into();
                return Err(ParsingError::UnknownFile { extension });
            }
        }
    }

    let file = File::open(path)?;
    let mut file = BufReader::new(file);

    parse_reader(&mut file)
}

fn get_file_reader(path: &Path) -> Result<BufReader<File>, ParsingError> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

pub fn parse_reader(reader: &mut dyn Read) -> Result<aggregate_report::feedback, ParsingError> {
    match from_reader(reader) {
        Ok(result) => Ok(result),
        Err(error) => Err(error.into()),
    }
}

/// This will panic if any of the files are not readable
/// and will only print an error on the stderr if parsing 
/// of any specific file fails. Use parse for better error handling
pub fn parse_dir(path: &Path) -> Vec<aggregate_report::feedback> {
    let mut results = Vec::new();

    if path.is_dir() {
        for entry in read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            let result = parse(&path);
            match result {
                Ok(result) => results.push(result),
                Err(error) => eprintln!("could not parse: {:?} because {:?}", path, error),
            }
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_single() {
        parse(Path::new("dmarc.xml")).unwrap();
    }

    #[test]
    fn test_parse_dir() {
        parse_dir(Path::new("./"));
    }
}
