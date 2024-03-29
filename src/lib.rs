#![forbid(unsafe_code)]
/*#![warn(missing_docs)]*/
#![deny(missing_debug_implementations)]

//! The DMARC Aggregate Parser is intended to provide a programmatical way to access information in an DMARC report.

pub mod aggregate_report;
pub mod error_handling;
use error_handling::ParsingError;

use libflate::gzip;
use serde_xml_rs::from_reader;
use std::fs::{read_dir, File};
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

/// This function takes a reference to a file to be parsed. If the file can not be parsed a [ParsingError] is returned.
/// #Example:
/// ```rust
/// use dmarc_aggregate_parser::parse;
/// # use std::path::Path;
/// # let path = Path::new("./sample-data/dmarc.xml");
/// let result = parse(path);
/// assert!(result.is_ok(), true);
/// if let Ok(result) = result {
///     assert_eq!(result.report_metadata.email,
///                "postmaster@aol.com".to_string()
///             );
/// }
/// ```
/// # Errors
/// See [ParsingError]
pub fn parse<T: std::convert::AsRef<std::ffi::OsStr>>(
    path: T,
) -> Result<aggregate_report::feedback, ParsingError> {
    let path = std::path::Path::new(&path);
    if path.is_dir() {
        let path_str = path.to_string_lossy().to_string();
        return Err(ParsingError::ParseDirectory { path_str });
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
                let mut decoder = gzip::Decoder::new(reader).map_err(|e| ParsingError::Io {
                    source: e,
                    path: path.to_string_lossy().to_string(),
                })?;
                return parse_reader(&mut decoder);
            }
            Some("zip") => {
                let file = File::open(path).map_err(|e| ParsingError::Io {
                    source: e,
                    path: path.to_string_lossy().to_string(),
                })?;
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

    let file = File::open(path).map_err(|e| ParsingError::Io {
        source: e,
        path: path.to_string_lossy().to_string(),
    })?;
    let mut file = BufReader::new(file);

    parse_reader(&mut file)
}

fn get_file_reader(path: &Path) -> Result<BufReader<File>, ParsingError> {
    let file = File::open(path).map_err(|e| ParsingError::Io {
        source: e,
        path: path.to_string_lossy().to_string(),
    })?;
    Ok(BufReader::new(file))
}

fn parse_reader(reader: &mut dyn Read) -> Result<aggregate_report::feedback, ParsingError> {
    match from_reader(reader) {
        Ok(result) => Ok(result),
        Err(error) => Err(error.into()),
    }
}

/// This function takes a directory path as an argument. If no dmarc report are found, an empty `Vec` is returned.
/// Any subdirectories are ignored.
/// Any files that can not be parsed will be reported through STDERR
/// # Note: Not suitable for production level use.
/// This function is only really suitable for testing. You are advised to use [parse()] within your own directory
/// processing code. In this way you can manage files as you wish.
pub fn parse_dir(path: &Path) -> Vec<aggregate_report::feedback> {
    let mut results = Vec::new();

    if path.is_dir() {
        for entry in read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if !path.is_dir() {
                // Only parse the path if it is a file and not a directory.
                let result = parse(&path);
                match result {
                    Ok(result) => results.push(result),
                    Err(error) => eprintln!("could not parse: {:?} because {:?}", path, error),
                }
            }
        }
    }
    results
}

#[cfg(test)]
mod parse {
    use super::*;
    #[test]
    fn single_xml() {
        let result = parse(Path::new("./sample-data/dmarc.xml")).unwrap();
        assert_eq!(result.version, None);
        assert_eq!(
            result.report_metadata.email,
            "postmaster@aol.com".to_string()
        );
    }
    #[test]
    #[ignore = "Requires the existence of a file with read permissions removed. Can not be added to git."]
    fn unreadable_xml() {
        let result = parse(Path::new("./sample-data/unreadable.xml"));
        assert_eq!(
            result.unwrap_err().to_string(),
            "Cannot open file: './sample-data/unreadable.xml' Permission denied (os error 13)"
        );
    }
    #[test]
    fn does_not_exist_xml() {
        let result = parse(Path::new("./sample-data/no_such_file.xml"));
        assert_eq!(
            result.unwrap_err().to_string(),
            "Cannot open file: './sample-data/no_such_file.xml' No such file or directory (os error 2)"
        );
    }
    #[test]
    fn test_error_when_parse_is_given_a_directory() {
        let result = parse(Path::new("./sample-data/"));
        assert!(result.is_err(), "{}", true);
        assert!(matches!(
                result,
            Err(ParsingError::ParseDirectory { path_str }) if path_str == "./sample-data/".to_string()
        ));
    }
}
#[cfg(test)]
mod parse_dir {

    use super::*;

    #[test]
    fn test_parse_dir() {
        parse_dir(Path::new("./"));
    }
    #[test]
    fn test_non_existent_dir() {
        parse_dir(Path::new("./sample-data/non-existent-dir/"));
    }
    #[test]
    fn test_parse_empty_dir() {
        parse_dir(Path::new("./sample-data/emtpy-dir/"));
    }
    #[test]
    fn test_parse_unreadable_dir() {
        parse_dir(Path::new("./sample-data/unreadable-dir/"));
    }
}
