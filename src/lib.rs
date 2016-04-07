// (c) 2014, Ethan Pailes
// All Rights Reserved
//
// Licensed under the "Revised" BSD 3-clause licence. See LICENCE file at the
// root directory of this repository.

//! File Diff
//!
//! This module provides an atomic file diffing function for use in unit tests.
//!
//! The diff_files() function takes two file handles and determines returns true
//! if they point to identical files.
//!
//! ```
//! use file_diff::{diff_files};
//! use std::fs::{File};
//!
//! let mut file1 = match File::open("./src/lib.rs") {
//!     Ok(f) => f,
//!     Err(e) => panic!("{}", e),
//! };
//! let mut file2 = match File::open("./src/lib.rs") {
//!     Ok(f) => f,
//!     Err(e) => panic!("{}", e),
//! };
//!
//! diff_files(&mut file1, &mut file2);
//! ```
//!
//! The diff() function takes string representations of the files and returns true
//! if the strings represent real files and those files are identical.
//!
//! ```
//! use file_diff::{diff};
//!
//! diff("./src/lib.rs", "./src/lib.rs");
//! ```

use std::io::Read;



use std::fs::{File};

/// Takes two file arguments and returns true if the two files are identical.
pub fn diff_files(f1: &mut File, f2: &mut File) -> bool {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();

    match f1.read_to_end(&mut v1) {
        Err(_) => false
      , Ok(b1) => match f2.read_to_end(&mut v2) {
            Err(_) => false 
          , Ok(b2) => b1 == b2
        }
    }
}

/// Takes two string filepaths and returns true if the two files are identical and exist.
pub fn diff(f1: &str, f2: &str) -> bool {
    let mut file1: File = match File::open(f1) {
        Ok(f) => f,
        Err(_) => return false,
    };
    let mut file2: File = match File::open(&f2) {
        Ok(f) => f,
        Err(_) => return false,
    };
    diff_files(&mut file1, &mut file2)
}

#[cfg(test)]
mod tests {
    const LIB_RS: &'static str = "./src/lib.rs";
    const CARGO_FILE: &'static str = "./Cargo.toml";
    const RUST_BIN_FILE: &'static str = "./testdata/rust_hello";
    const C_BIN_FILE: &'static str = "./testdata/c_hello";
    const C_BIN_FILE_COPY: &'static str = "./testdata/c_hello_copy";

    #[test]
    fn diff_the_same_text_file() { assert!(super::diff(LIB_RS, LIB_RS)); }

    #[test]
    fn diff_the_same_binary_file() { assert!(super::diff(RUST_BIN_FILE, RUST_BIN_FILE)); }

    #[test]
    fn diff_identical_binary_files() { assert!(super::diff(C_BIN_FILE, C_BIN_FILE_COPY)); }

    #[test]
    fn diff_different_text_files() { assert!( !super::diff(LIB_RS, CARGO_FILE)); }

    #[test]
    fn diff_different_binary_files() { assert!( !super::diff(RUST_BIN_FILE, C_BIN_FILE)); }
}

