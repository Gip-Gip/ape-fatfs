//! ## *versitile FAT library for embedded systems*
//!
//! # Usage
//!
//! This crate is can be used by adding `fatfs` to the dependencies in your
//! project's `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! ape-fatfs = "0.1.0"
//! # Comment out the above and uncomment the below to enable no_std support
//! # ape-fatfs = { default_features = false, version = 0.1.0 }
//! ```
//!
//! # Examples
//!
//! ```rust
//! use std::io::prelude::*;
//! use ape_fatfs::{
//!     fs::{
//!         FsOptions,
//!         FileSystem,
//!     }
//! };
//!
//! fn main() {
//!     # std::fs::copy("resources/fat16.img", "fat.img").unwrap();
//!     // Initialize a filesystem object
//!     let img_file = std::fs::OpenOptions::new().read(true).write(true)
//!         .open("fat.img").unwrap();
//!     let buf_stream = fscommon::BufStream::new(img_file);
//!     let fs = FileSystem::new(buf_stream, FsOptions::new()).unwrap();
//!     let root_dir = fs.root_dir();
//!
//!     // Write a file
//!     root_dir.create_dir("foo").unwrap();
//!     let mut file = root_dir.create_file("foo/hello.txt").unwrap();
//!     file.truncate().unwrap();
//!     file.write_all(b"Hello World!").unwrap();
//!
//!     // Read a directory
//!     let dir = root_dir.open_dir("foo").unwrap();
//!     for r in dir.iter() {
//!         let entry = r.unwrap();
//!         println!("{}", entry.file_name());
//!     }
//!     # std::fs::remove_file("fat.img").unwrap();
//! }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
// Disable warnings to not clutter code with cfg too much
#![cfg_attr(
    not(all(feature = "alloc", feature = "lfn", feature = "std")),
    allow(dead_code, unused_imports)
)]

extern crate log;

#[macro_use]
mod log_macros;

pub mod boot_sector;
pub mod dir;
pub mod dir_entry;
pub mod error;
pub mod file;
pub mod fs;
pub mod io;
pub mod table;
pub mod time;
