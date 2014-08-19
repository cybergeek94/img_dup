extern crate image;
extern crate time;

use processing::{ProcessingError, ImageResult, Total};
use img::{Image, UniqueImage};
use hash::ImageHash;
use output::{ProcessingResults, output_results};
use parse_args::{ProgramSettings, HashSettings, parse_args};

use image::{GenericImage, ImageError};

use self::time::now;

use std::ascii::AsciiExt;
use std::io::fs;
use std::os;

#![feature(macro_rules)]

mod dct;
mod img;
mod hash;
mod output;
mod parse_args;
mod processing;

fn main() {
    let args = os::args();

    let settings = parse_args(args.as_slice());

    println!("Searching for images...");

    let mut image_paths = find_images(&settings.dir, 
                                      settings.exts.as_slice(), 
                                      settings.recurse);

    let image_count = image_paths.len();

    println!("Images found: {}", image_count);

    if settings.limit > 0 {
        println!("Limiting to: {}", settings.limit);
        image_paths.truncate(settings.limit);
    }

    println!("Processing images in {} threads. Please wait...\n", 
             settings.threads);

    let results = processing::process(&settings, image_paths);

    println!("");

    output::output_results(results).unwrap()   
}

fn find_images(dir: &Path, exts: &[String], recurse: bool) -> Vec<Path> {
    let exts: Vec<&str> = exts.iter().map(|string| string.as_slice()).collect();

    if recurse {
        fs::walk_dir(dir)
            .unwrap()
            .filter( |file| check_ext(file, exts.as_slice()) )
            .collect()   
    } else {
        fs::readdir(dir)
            .unwrap()
            .move_iter()
            .filter( |file| !file.is_dir() && check_ext(file, exts.as_slice()) )
            .collect()
    } 
}

fn check_ext<'a>(file: &'a Path, exts: &'a [&'a str]) -> bool {   
    match file.extension_str() {
        Some(ext) => exts.iter().any(|&a| a.eq_ignore_ascii_case(ext)),
        None => false
    }
}

