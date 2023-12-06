use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_lines(path: &str) -> Vec<String> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(err) => panic!("{}", err)
    };
    let reader = BufReader::new(file);
    let mut ret = vec![];
    for line in reader.lines() {
        match line {
            Ok(l) => ret.push(l),
            _ => {},
        }
    }
    ret
}