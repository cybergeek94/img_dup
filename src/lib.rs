//! As a library crate, `img_dup` provides tools for searching for images, hashing them in
//! parallel, and collating their hashes to find near or complete duplicates.

extern crate img_hash;
extern crate image;
extern crate rayon;
extern crate vec_vp_tree as vp_tree;

#[macro_use]
extern crate serde_derive;

pub mod model;
pub mod hash;
pub mod search;

use hash::HashType;
use model::{HashSettings, Image, ImgResults};

use image::ImageError;

use rayon::ThreadPool;

use std::path::Path;

/// A builder struct for bootstrapping an `img_dup` session.
pub struct Settings<'a> {
    pub outfile: PathBuf,

    /// The size of the hash to use.
    ///
    /// See the `HashType` documentation for the actual size
    /// of a hash generated by each hash type.
    pub hash_size: u32,

    /// The type of the hash to use. See `HashType` for more information.
    pub hash_type: HashType,

    pub threads: usize,

    pub comp_type: CompareType,

    pub pretty_indent: Option<usize>,
}

impl Default for Settings<'static> {
    fn default() -> Self {
        // Are these really magic values if they're all in one place?
        Settings {
            outfile: "img-dup.json".as_ref(),
            hash_size: 8,
            hash_type: HashType::default(),
            threads: num_cpus::get(),
            comp_type: CompareType::KNearest(5),
            pretty_indent: None,
        }
    }
}

pub struct Error {
    pub path: PathBuf,
    pub error: ImageError,
}

pub type Result<T> = ::std::result::Result<T, Error>;

pub fn rayon_pool(threads: Option<usize>) -> ThreadPool {
    use rayon::Configuration;

    let config = if let Some(threads) = threads {
        Configuration::new().set_num_threads(threads)
    } else {
        Configuration::new()
    };

    ThreadPool::new(config).expect("Error initializing thread pool")
}