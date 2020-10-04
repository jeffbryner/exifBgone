// rust is bad at marketing
#![allow(non_snake_case)]

//use std::fs;

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

fn main() {
    println!("exifBgone: ridding the world of exif tags, one file at a time.");
    let target = get_target();
    println!("targeting: {}", target);
}
