/*
Test input line by line for the config entries created with this library
*/

use directories::UserDirs;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[test]
fn config_entries() {
    let file_path = UserDirs::new().unwrap().home_dir().join(".oci/config");
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut line = lines.next().unwrap().unwrap();
    // check entry against regex pattern ^ocid1\.tenancy\.([a-zA-Z0-9_]){32}$ #1 
    assert_eq!(line, "tenancy = ^ocid1\\.tenancy\\.([a-zA-Z0-9_]){32}$");
}
