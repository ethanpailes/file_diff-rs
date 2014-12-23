//TODO(ethan): licence, header, & documentation

//! File Diff
//!
//! This module provides an atomic file diffing function for use in unit tests.
//!
//! The diff_files() function takes two file handles and determines returns true
//! if they point to identical files.
//!
//! ```
//! use file_diff::{diff_files};
//! use std::io::{File};
//!
//! let mut file1 = match File::open(&Path::new("./src/lib.rs")) {
//!     Ok(f) => f,
//!     Err(e) => panic!("{}", e),
//! };
//! let mut file2 = match File::open(&Path::new("./src/lib.rs")) {
//!     Ok(f) => f,
//!     Err(e) => panic!("{}", e),
//! };
//!
//! diff_files(file1, file2);
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

use std::io::{File, BufferedReader};

/// Takes two file arguments and returns true if the two files are identical
pub fn diff_files(f1: File, f2: File) -> bool {
    let mut b1 = BufferedReader::new(f1);
    let mut b2 = BufferedReader::new(f2);

    for l1 in b1.lines() {
        let l2 = b2.read_line();
        if l1 != l2 { return false; }
    }
    true
}

/// Takes two string filepaths and returns true if the two files are identical and exist
#[allow(unused_variables)]
pub fn diff(f1: &str, f2: &str) -> bool {
    let file1: File = match File::open(&Path::new(f1)) {
        Ok(f) => f,
        Err(e) => return false,
    };
    let file2: File = match File::open(&Path::new(f2)) {
        Ok(f) => f,
        Err(e) => return false,
    };
    diff_files(file1, file2)
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
    #[should_fail]
    fn diff_different_text_files() { assert!(super::diff(LIB_RS, CARGO_FILE)); }
    #[test]
    #[should_fail]
    fn diff_different_binary_files() { assert!(super::diff(RUST_BIN_FILE, C_BIN_FILE)); }
}

