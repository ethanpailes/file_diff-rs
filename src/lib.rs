//TODO(ethan): licence, header, & documentation

use std::io::{File, BufferedReader};

pub fn diff(f1: File, f2: File) -> bool {
    let mut b1 = BufferedReader::new(f1);
    let mut b2 = BufferedReader::new(f2);

    for l1 in b1.lines() {
        let l2 = b2.read_line();
        if l1 != l2 { return false; }
    }
    true
}

#[cfg(test)]
mod tests {
    use std::io::{File};

    const LIB_RS: &'static str = "./src/lib.rs";
    const CARGO_FILE: &'static str = "./Cargo.toml";
    const RUST_BIN_FILE: &'static str = "./testdata/rust_hello";
    const C_BIN_FILE: &'static str = "./testdata/c_hello";
    const C_BIN_FILE_COPY: &'static str = "./testdata/c_hello_copy";

    #[allow(unused_variables)]
    fn diff_files(s1: &'static str, s2: &'static str) -> bool {
        let f1: File = match File::open(&Path::new(s1)) {
            Ok(f) => f,
            Err(e) => return false,
        };
        let f2: File = match File::open(&Path::new(s2)) {
            Ok(f) => f,
            Err(e) => return false,
        };
        super::diff(f1, f2)
    }

    #[test]
    fn diff_the_same_text_file() { assert!(diff_files(LIB_RS, LIB_RS)); }
    #[test]
    fn diff_the_same_binary_file() { assert!(diff_files(RUST_BIN_FILE, RUST_BIN_FILE)); }
    #[test]
    fn diff_identical_binary_files() { assert!(diff_files(C_BIN_FILE, C_BIN_FILE_COPY)); }
    #[test]
    #[should_fail]
    fn diff_different_text_files() { assert!(diff_files(LIB_RS, CARGO_FILE)); }
    #[test]
    #[should_fail]
    fn diff_different_binary_files() { assert!(diff_files(RUST_BIN_FILE, C_BIN_FILE)); }

    // TODO(ethan): try to pass in files that will throw errors
}

