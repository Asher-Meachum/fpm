use std::fs;

use crate::Link;

pub fn update_file(files: &Link) {
    match fs::copy(files.upstream(), files.downstream()) {
        Ok(b) => println!("{b}"),
        Err(e) => eprintln!("{e}"),
    }
}