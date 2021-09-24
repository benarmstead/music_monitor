use std::fs;
use std::fs::File;
use std::io::Write;

// Takes a filename and returns the contents as a String
#[allow(dead_code)]
pub fn read(filename: String) -> String {
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

// Takes a filename and contents and writes to the given filename
#[allow(dead_code)]
pub fn write(filename: String, contents: String) {
    let mut file = File::create(filename).expect("create failed");
    file.write_all(contents.as_bytes()).expect("write failed");
}
