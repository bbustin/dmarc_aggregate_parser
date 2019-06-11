#![deny(missing_debug_implementations)]
pub mod aggregate_report;

use std::fs::{File, read_dir};
use std::io::BufReader;
use std::path::Path;
use serde_xml_rs::from_reader;
use libflate::gzip;
use zip;
use std::io::Read;

pub fn parse(path: &Path) -> Result<aggregate_report::feedback, Box<std::error::Error>> {
    if path.is_dir() {
        panic!("{} is a directory")
    }

    let extension = path.extension();

    match extension {
        Some(extension) => {
            match extension.to_str().unwrap() {
                "xml" => {
                    let mut reader = get_file_reader(&path)?;
                    return parse_reader(&mut reader);
                },
                "gz" | "gzip" => {
                    let reader = get_file_reader(&path)?;
                    let mut decoder = gzip::Decoder::new(reader)?;
                    return parse_reader(&mut decoder);
                },
                "zip" => {
                    let file = File::open(&path)?;
                    let mut archive = zip::ZipArchive::new(file)?;
                    let mut file = archive.by_index(0)?;
                    return parse_reader(&mut file);
                },
                _       => {
                    let error_message = format!("Do not know how to handle {} files :-(", extension.to_str().unwrap());
                    return Err(error_message.into());
                }
            }
        },
        None            => ()
    }

    let file = File::open(&path)?;
    let mut file = BufReader::new(file);

    parse_reader(&mut file)
}

fn get_file_reader(path: &Path) -> Result<BufReader<File>, Box<std::error::Error>> {
    let file = File::open(&path)?;
    Ok(BufReader::new(file))
}

fn parse_reader(reader: &mut Read) -> Result<aggregate_report::feedback, Box<std::error::Error>> {
    match from_reader(reader) {
        Ok(result) => Ok(result),
        Err(error) => Err(error.into())
    }
}

pub fn parse_dir(path: &Path) -> Vec<aggregate_report::feedback> {
    let mut results = Vec::new();

    if path.is_dir() {
        for entry in read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            let result = parse(&path);
            match result {
                Ok(result) => results.push(result),
                Err(error) => println!("could not parse: {:?} because {:?}", path, error)
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
        parse(Path::new("dmarc.xml"));
    }

    #[test]
    fn test_parse_dir() {
        parse_dir(Path::new("./"));
    }
}
