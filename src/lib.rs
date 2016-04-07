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

    let mut buff1 : &mut [u8] = &mut [0; 1024];
    let mut buff2 : &mut [u8] = &mut [0; 1024];
    
    loop {

        match f1.read(buff1) {
            Err(_) => return false,
            Ok(f1_read_len) => match f2.read(buff2) {
                Err(_) => return false,
                Ok(f2_read_len) => {
                    if f1_read_len != f2_read_len {
                        return false;
                    }
                    if f1_read_len == 0 {
                        return true;
                    }
                    if &buff1[0..f1_read_len] != &buff2[0..f2_read_len] {
                        return false;
                    }
                }
            }
        }
    }
}

/// Takes two string filepaths and returns true if the two files are identical and exist.
pub fn diff(f1: &str, f2: &str) -> bool {
    let mut fh1 = File::open(f1);
    let mut fh2 = File::open(f2);

    fh1.as_mut().and_then(|file1|
        fh2.as_mut().and_then(|file2|
            Ok(diff_files(file1, file2)))).unwrap_or(false)
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

