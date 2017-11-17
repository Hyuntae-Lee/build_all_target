use std::fs::File;
use std::io::{Read, Write};

pub fn read_file(contents : &mut String, file_path : &str) -> bool {
    // open
    let mut file = match File::open(file_path) {
        Err(r) => {
            println!("{:?}", r);
            return false;
        },
        Ok(x) => x
    };

    // read
    match file.read_to_string(contents) {
        Err(r) => {
            println!("{:?}", r);
            return false;
        },
        Ok(_) => {}
    }

    //
    return true;
}

pub fn write_to_file<'a>(file_path : &str, contents : &str) -> bool {
    // open
    let mut file = match File::create(file_path) {
        Err(_) => {
            return false;
        },
        Ok(x) => x
    };
    // write
    match file.write_all(contents.as_bytes()) {
        Err(_) => {
            return false;
        },
        Ok(_) => {}
    };

    return true;
}
