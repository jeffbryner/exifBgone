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

fn process_target_file(filename: String) -> Result<String, std::io::Error> {
    // default
    let mut exif = String::from("");
    exif.push_str(&filename);
    exif.push_str(": ");
    let path = Path::new(&filename);
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            //let entry_path = entry?;
            // let athing = entry.path();
            // println!("{:?}", athing);
            if let Ok(entry_path) = entry {
                println!("{:?}", entry_path.path());
            }
        }
    }

    if path.is_file() {
        match Metadata::new_from_path(filename.clone()) {
            Ok(exif_metadata) => {
                if exif_metadata.has_exif() {
                    exif.push_str("EXIF");
                }
                if exif_metadata.has_iptc() {
                    exif.push_str("IPTC");
                }
                match exif_metadata.get_gps_info() {
                    Some(_) => exif.push_str("GPS"),
                    None => {}
                };
            }
            Err(_) => exif.push_str("error retrieving metadata"),
        };
    }
    Ok(exif)
}

fn main() {
    println!("exifBgone: ridding the world of exif tags, one file at a time.");
    let target = get_target();
    println!("targeting: {}", target);
    let s_result = process_target_file(target);
    println!(
        "result: {:?}",
        s_result.expect("could not retrieve exif data")
    );
}
