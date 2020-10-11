// rust is bad at marketing
#![allow(non_snake_case)]

use rexiv2::Metadata;
use std::fs;
use std::path::Path;

fn get_target() -> String {
    // find the directory or file to target
    let args: Vec<String> = std::env::args().collect();

    // default
    let mut target = "./";
    if args.len() > 1 {
        target = &args[1];
    }

    target.to_string()
}

fn process_file(filename: &str) -> Result<String, std::io::Error> {
    let mut exif = String::from("");
    exif.push_str(&filename);
    exif.push_str(": ");
    match Metadata::new_from_path(filename.clone()) {
        Ok(exif_metadata) => {
            if exif_metadata.supports_exif() && exif_metadata.has_exif() {
                exif.push_str("EXIF ");
            }
            if exif_metadata.supports_iptc() && exif_metadata.has_iptc() {
                exif.push_str("IPTC ");
            }
            if exif_metadata.supports_xmp() && exif_metadata.has_xmp() {
                exif.push_str("XMP ");
            }
            match exif_metadata.get_gps_info() {
                Some(_) => exif.push_str("GPS "),
                None => {}
            };
        }
        Err(exif_error) => match exif_error {
            rexiv2::Rexiv2Error::Internal(Some(ref msg)) => {
                if msg.contains("The file contains data of an unknown image type") {
                    {}
                } else {
                    println!("internal error --> {:}", msg);
                }
            }
            _ => println!("internal error --> yikes"),
        },
    };
    Ok(String::from(exif.trim_end()))
}
fn process_target(target: String) -> Result<bool, std::io::Error> {
    let path = Path::new(&target);
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            //let entry_path = entry?;
            // let athing = entry.path();
            // println!("{:?}", athing);
            if let Ok(entry_path) = entry {
                if entry_path.path().is_file() {
                    let file_result = process_file(entry_path.path().to_str().unwrap());
                    println!("{:}", file_result.expect("could not process file"));
                }
            }
        }
    }

    if path.is_file() {
        let file_result = process_file(path.to_str().unwrap());
        println!("{:}", file_result.expect("could not process file"));
    }
    Ok(true)
}

fn main() {
    println!("exifBgone: ridding the world of exif tags, one file at a time.");
    let target = get_target();
    println!("targeting: {}", target);
    let _ = process_target(target);
}
