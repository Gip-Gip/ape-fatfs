//! A FAT filesystem library implemented in Rust.
//!
//! # Usage
//!
//! This crate is [on crates.io](https://crates.io/crates/fatfs) and can be
//! used by adding `fatfs` to the dependencies in your project's `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! ape_fatfs = "0.1.0"
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
//! fn main() -> std::io::Result<()> {
//!     # std::fs::copy("resources/fat16.img", "tmp/fat.img")?;
//!     // Initialize a filesystem object
//!     let img_file = std::fs::OpenOptions::new().read(true).write(true)
//!         .open("tmp/fat.img")?;
//!     let buf_stream = fscommon::BufStream::new(img_file);
//!     let fs = FileSystem::new(buf_stream, FsOptions::new())?;
//!     let root_dir = fs.root_dir();
//!
//!     // Write a file
//!     root_dir.create_dir("foo")?;
//!     let mut file = root_dir.create_file("foo/hello.txt")?;
//!     file.truncate()?;
//!     file.write_all(b"Hello World!")?;
//!
//!     // Read a directory
//!     let dir = root_dir.open_dir("foo")?;
//!     for r in dir.iter() {
//!         let entry = r?;
//!         println!("{}", entry.file_name());
//!     }
//!     # std::fs::remove_file("tmp/fat.img")?;
//!     # Ok(())
//! }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
// Disable warnings to not clutter code with cfg too much
#![cfg_attr(not(all(feature = "alloc", feature = "lfn")), allow(dead_code, unused_imports))]

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

