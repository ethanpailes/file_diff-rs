File Diff

This module provides an atomic file diffing function for use in unit tests.

The diff_files() function takes two file handles and determines returns true
if they point to identical files.

```rust
use file_diff::{diff_files};
use std::io::{File};

let mut file1 = match File::open(&Path::new("./src/lib.rs")) {
    Ok(f) => f,
    Err(e) => panic!("{}", e),
};
let mut file2 = match File::open(&Path::new("./src/lib.rs")) {
    Ok(f) => f,
    Err(e) => panic!("{}", e),
};

diff_files(file1, file2); // true
```

The diff() function takes string representations of the files and returns true
if the strings represent real files and those files are identical.

```rust
use file_diff::{diff};

diff("./src/lib.rs", "./src/lib.rs"); // true
```
